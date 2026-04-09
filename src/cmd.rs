use anyhow::{bail, Context};
use std::path::Path;

use crate::api;
use crate::config::{self, GitLabConfig};

// --- Artifacts ---

async fn download_by_branch(
    client: &reqwest::Client,
    config: &GitLabConfig,
    project: &str,
    branch: &str,
    job_name: &str,
    output: &Path,
) -> anyhow::Result<()> {
    println!("Looking for pipeline by ref '{}'...", branch);
    let pipeline = api::get_pipeline_by_ref(client, &config.api_url, project, branch).await?;
    println!("Found pipeline: {}", pipeline.id);

    println!("Searching for job '{}'...", job_name);
    let job = api::find_job(client, &config.api_url, project, pipeline.id, job_name).await?;
    println!("Found job: {} (id={})", job.name, job.id);

    println!("Downloading artifacts...");
    api::download_artifacts(client, &config.api_url, project, job.id, output).await?;

    Ok(())
}

async fn download_by_commit(
    client: &reqwest::Client,
    config: &GitLabConfig,
    project: &str,
    sha: &str,
    job_name: &str,
    output: &Path,
) -> anyhow::Result<()> {
    println!("Looking for pipeline by commit '{}'...", sha);
    let pipeline = api::get_pipeline_by_sha(client, &config.api_url, project, sha).await?;
    println!("Found pipeline: {}", pipeline.id);

    println!("Searching for job '{}'...", job_name);
    let job = api::find_job(client, &config.api_url, project, pipeline.id, job_name).await?;
    println!("Found job: {} (id={})", job.name, job.id);

    println!("Downloading artifacts...");
    api::download_artifacts(client, &config.api_url, project, job.id, output).await?;

    Ok(())
}

pub async fn handle_download(
    project: &str,
    branch: Option<&str>,
    commit: Option<&str>,
    job_name: &str,
    output: &Path,
    insecure: bool,
) -> anyhow::Result<()> {
    let config = config::load_config().context("Failed to load GitLab configuration")?;
    let client = config::build_client(&config, insecure)?;

    match (branch, commit) {
        (Some(b), None) => download_by_branch(&client, &config, project, b, job_name, output).await,
        (None, Some(c)) => download_by_commit(&client, &config, project, c, job_name, output).await,
        _ => bail!("Must specify exactly one of --branch or --commit"),
    }
}

// --- Packages ---

pub async fn handle_package_upload(
    project: &str,
    name: &str,
    version: &str,
    file: &Path,
    insecure: bool,
) -> anyhow::Result<()> {
    let config = config::load_config().context("Failed to load GitLab configuration")?;
    let client = config::build_client(&config, insecure)?;

    let file_name = file
        .file_name()
        .context("Invalid file path")?
        .to_str()
        .context("Invalid file name encoding")?;

    println!(
        "Uploading '{}' version '{}' from '{}'...",
        name, version, file.display()
    );

    api::upload_package(
        &client,
        &config.api_url,
        project,
        name,
        version,
        file_name,
        file,
    )
    .await?;

    Ok(())
}

pub async fn handle_package_download(
    project: &str,
    name: &str,
    version: &str,
    file: &str,
    output: &Path,
    insecure: bool,
) -> anyhow::Result<()> {
    let config = config::load_config().context("Failed to load GitLab configuration")?;
    let client = config::build_client(&config, insecure)?;

    println!(
        "Downloading '{}' version '{}' ({})...",
        name, version, file
    );

    api::download_package(
        &client,
        &config.api_url,
        project,
        name,
        version,
        file,
        output,
    )
    .await?;

    Ok(())
}

pub async fn handle_package_delete(
    project: &str,
    package_id: u64,
    insecure: bool,
) -> anyhow::Result<()> {
    let config = config::load_config().context("Failed to load GitLab configuration")?;
    let client = config::build_client(&config, insecure)?;

    println!("Deleting package {}...", package_id);
    api::delete_package(&client, &config.api_url, project, package_id).await?;

    Ok(())
}

pub async fn handle_package_list(
    project: &str,
    name: Option<&str>,
    insecure: bool,
) -> anyhow::Result<()> {
    let config = config::load_config().context("Failed to load GitLab configuration")?;
    let client = config::build_client(&config, insecure)?;

    let packages = api::list_packages(&client, &config.api_url, project, name).await?;

    if packages.is_empty() {
        println!("No packages found.");
        return Ok(());
    }

    for pkg in &packages {
        println!("{:<8} {:<40} {:<20} {}", pkg.id, pkg.name, pkg.version, pkg.package_type);
    }

    Ok(())
}
