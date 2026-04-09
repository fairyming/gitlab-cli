use reqwest::blocking::Client;

use super::types::{encode_project, Package};

pub fn upload_package(
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

    let file_bytes = std::fs::read(file_path)?;

    let result = client.put(&url).body(file_bytes).send();
    let response = match result {
        Ok(resp) => resp,
        Err(e) => return Err(e.into()),
    };

    if response.status().is_success() {
        println!("Package '{}' version '{}' uploaded successfully", package_name, package_version);
        return Ok(());
    }

    if response.status() == reqwest::StatusCode::CONFLICT {
        println!("Package already exists, updating...");
        return upload_package_force(client, api_url, project, package_name, package_version, file_name, file_path);
    }

    super::check_response(response, &url)?;
    unreachable!()
}

fn upload_package_force(
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

    let file_bytes = std::fs::read(file_path)?;

    client.delete(&url).send()?;

    let response = client.put(&url).body(file_bytes).send()?;
    super::check_response(response, &url)?;

    println!("Package '{}' version '{}' uploaded successfully", package_name, package_version);

    Ok(())
}

pub fn download_package(
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

    let response = client.get(&url).send()?;
    let response = super::check_response(response, &url)?;

    let bytes = response.bytes()?;

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output, &bytes)?;
    println!("Package '{}' version '{}' downloaded to {}", package_name, package_version, output.display());

    Ok(())
}

pub fn list_packages(
    client: &Client,
    api_url: &str,
    project: &str,
    package_name: Option<&str>,
) -> anyhow::Result<Vec<Package>> {
    let mut url =
        format!("{}/projects/{}/packages?per_page=100&order_by=version&sort=desc", api_url, encode_project(project),);

    if let Some(name) = package_name {
        url.push_str(&format!("&package_name={}", urlencoding::encode(name)));
    }

    let response = client.get(&url).send()?;
    let response = super::check_response(response, &url)?;

    let packages: Vec<Package> = response.json()?;

    Ok(packages)
}

pub fn delete_package(client: &Client, api_url: &str, project: &str, package_id: u64) -> anyhow::Result<()> {
    let url = format!("{}/projects/{}/packages/{}", api_url, encode_project(project), package_id);

    let response = client.delete(&url).send()?;
    super::check_response(response, &url)?;

    println!("Package {} deleted successfully", package_id);

    Ok(())
}
