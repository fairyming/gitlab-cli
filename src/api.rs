use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pipeline {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub id: u64,
    pub name: String,
    pub version: String,
    pub package_type: String,
}

pub(crate) fn encode_project(project: &str) -> String {
    urlencoding::encode(project).to_string()
}

pub async fn get_pipeline_by_ref(
    client: &Client,
    api_url: &str,
    project: &str,
    r#ref: &str,
) -> anyhow::Result<Pipeline> {
    let url = format!(
        "{}/projects/{}/pipelines?ref={}&per_page=1&order_by=id&sort=desc",
        api_url,
        encode_project(project),
        urlencoding::encode(r#ref),
    );

    let pipelines: Vec<Pipeline> = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    pipelines
        .into_iter()
        .next()
        .context(format!("No pipeline found for ref '{}'", r#ref))
}

pub async fn get_pipeline_by_sha(
    client: &Client,
    api_url: &str,
    project: &str,
    sha: &str,
) -> anyhow::Result<Pipeline> {
    let url = format!(
        "{}/projects/{}/pipelines?sha={}&per_page=1&order_by=id&sort=desc",
        api_url,
        encode_project(project),
        urlencoding::encode(sha),
    );

    let pipelines: Vec<Pipeline> = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    pipelines
        .into_iter()
        .next()
        .context(format!("No pipeline found for commit '{}'", sha))
}

pub async fn find_job(
    client: &Client,
    api_url: &str,
    project: &str,
    pipeline_id: u64,
    job_name: &str,
) -> anyhow::Result<Job> {
    let url = format!(
        "{}/projects/{}/pipelines/{}/jobs?scope[]=success&per_page=100",
        api_url,
        encode_project(project),
        pipeline_id
    );

    let jobs: Vec<Job> = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    jobs.into_iter()
        .find(|j| j.name == job_name)
        .with_context(|| format!("Job '{}' not found in pipeline {}", job_name, pipeline_id))
}

pub async fn download_artifacts(
    client: &Client,
    api_url: &str,
    project: &str,
    job_id: u64,
    output: &std::path::Path,
) -> anyhow::Result<()> {
    let url = format!(
        "{}/projects/{}/jobs/{}/artifacts",
        api_url,
        encode_project(project),
        job_id
    );

    let bytes = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output, &bytes)?;
    println!("Artifacts saved to {}", output.display());

    Ok(())
}

pub async fn upload_package(
    client: &Client,
    api_url: &str,
    project: &str,
    package_name: &str,
    package_version: &str,
    file_name: &str,
    file_path: &std::path::Path,
) -> anyhow::Result<()> {
    let url = format!(
        "{}/projects/{}/packages/generic/{}/{}/{}",
        api_url,
        encode_project(project),
        urlencoding::encode(package_name),
        urlencoding::encode(package_version),
        urlencoding::encode(file_name),
    );

    let file_bytes = tokio::fs::read(file_path).await?;

    let response = client
        .put(&url)
        .body(file_bytes)
        .send()
        .await?
        .error_for_status();

    match response {
        Ok(_) => println!(
            "Package '{}' version '{}' uploaded successfully",
            package_name, package_version
        ),
        Err(e) => {
            // PUT may return 409 for conflict, try DELETE + re-upload
            if e.status() == Some(reqwest::StatusCode::CONFLICT) {
                println!("Package already exists, updating...");
                return upload_package_force(
                    client,
                    api_url,
                    project,
                    package_name,
                    package_version,
                    file_name,
                    file_path,
                )
                .await;
            }
            return Err(e.into());
        }
    }

    Ok(())
}

async fn upload_package_force(
    client: &Client,
    api_url: &str,
    project: &str,
    package_name: &str,
    package_version: &str,
    file_name: &str,
    file_path: &std::path::Path,
) -> anyhow::Result<()> {
    let url = format!(
        "{}/projects/{}/packages/generic/{}/{}/{}",
        api_url,
        encode_project(project),
        urlencoding::encode(package_name),
        urlencoding::encode(package_version),
        urlencoding::encode(file_name),
    );

    let file_bytes = tokio::fs::read(file_path).await?;

    // Delete the existing package file first, then re-upload
    client.delete(&url).send().await?;

    client
        .put(&url)
        .body(file_bytes)
        .send()
        .await?
        .error_for_status()?;

    println!(
        "Package '{}' version '{}' uploaded successfully",
        package_name, package_version
    );

    Ok(())
}

pub async fn download_package(
    client: &Client,
    api_url: &str,
    project: &str,
    package_name: &str,
    package_version: &str,
    file_name: &str,
    output: &std::path::Path,
) -> anyhow::Result<()> {
    let url = format!(
        "{}/projects/{}/packages/generic/{}/{}/{}",
        api_url,
        encode_project(project),
        urlencoding::encode(package_name),
        urlencoding::encode(package_version),
        urlencoding::encode(file_name),
    );

    let bytes = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output, &bytes)?;
    println!(
        "Package '{}' version '{}' downloaded to {}",
        package_name,
        package_version,
        output.display()
    );

    Ok(())
}

pub async fn list_packages(
    client: &Client,
    api_url: &str,
    project: &str,
    package_name: Option<&str>,
) -> anyhow::Result<Vec<Package>> {
    let mut url = format!(
        "{}/projects/{}/packages?per_page=100&order_by=version&sort=desc",
        api_url,
        encode_project(project),
    );

    if let Some(name) = package_name {
        url.push_str(&format!("&package_name={}", urlencoding::encode(name)));
    }

    let packages: Vec<Package> = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(packages)
}

pub async fn delete_package(
    client: &Client,
    api_url: &str,
    project: &str,
    package_id: u64,
) -> anyhow::Result<()> {
    let url = format!(
        "{}/projects/{}/packages/{}",
        api_url,
        encode_project(project),
        package_id
    );

    client.delete(&url).send().await?.error_for_status()?;

    println!("Package {} deleted successfully", package_id);

    Ok(())
}
