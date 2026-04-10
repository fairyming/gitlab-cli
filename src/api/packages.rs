use super::types::{encode_project, Package, PackageFile};
use crate::client::GitLabClient;

pub fn upload_package(
    gitlab: &GitLabClient,
    project: &str,
    package_name: &str,
    package_version: &str,
    file_name: &str,
    file_path: &std::path::Path,
) -> anyhow::Result<()> {
    let url = format!(
        "{}/projects/{}/packages/generic/{}/{}/{}",
        gitlab.api_url,
        encode_project(project),
        urlencoding::encode(package_name),
        urlencoding::encode(package_version),
        urlencoding::encode(file_name),
    );

    let file_bytes = std::fs::read(file_path)?;

    gitlab.put(&url, file_bytes)?;

    println!("Package '{}' version '{}' uploaded successfully", package_name, package_version);

    Ok(())
}

pub fn download_package(
    gitlab: &GitLabClient,
    project: &str,
    package_name: &str,
    package_version: &str,
    file_name: &str,
    output: &std::path::Path,
) -> anyhow::Result<()> {
    let url = format!(
        "{}/projects/{}/packages/generic/{}/{}/{}",
        gitlab.api_url,
        encode_project(project),
        urlencoding::encode(package_name),
        urlencoding::encode(package_version),
        urlencoding::encode(file_name),
    );

    let bytes = gitlab.get(&url)?;

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output, &bytes)?;
    println!("Package '{}' version '{}' downloaded to {}", package_name, package_version, output.display());

    Ok(())
}

pub fn list_packages(gitlab: &GitLabClient, project: &str, package_name: Option<&str>) -> anyhow::Result<Vec<Package>> {
    let mut url = format!(
        "{}/projects/{}/packages?per_page=100&order_by=version&sort=desc",
        gitlab.api_url,
        encode_project(project),
    );

    if let Some(name) = package_name {
        url.push_str(&format!("&package_name={}", urlencoding::encode(name)));
    }

    let body = gitlab.get(&url)?;
    let packages: Vec<Package> = serde_json::from_slice(&body)?;
    Ok(packages)
}

pub fn delete_package(gitlab: &GitLabClient, project: &str, package_id: u64) -> anyhow::Result<()> {
    let url = format!("{}/projects/{}/packages/{}", gitlab.api_url, encode_project(project), package_id);
    gitlab.delete(&url)?;
    println!("Package {} deleted successfully", package_id);
    Ok(())
}

pub fn list_package_files(gitlab: &GitLabClient, project: &str, package_id: u64) -> anyhow::Result<Vec<PackageFile>> {
    let url = format!("{}/projects/{}/packages/{}/package_files", gitlab.api_url, encode_project(project), package_id,);

    let body = gitlab.get(&url)?;
    let files: Vec<PackageFile> = serde_json::from_slice(&body)?;
    Ok(files)
}
