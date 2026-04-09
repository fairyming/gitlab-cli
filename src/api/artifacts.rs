use anyhow::Context;
use reqwest::blocking::Client;

use super::types::{encode_project, Job, Pipeline};

pub fn get_pipeline_by_ref(client: &Client, api_url: &str, project: &str, r#ref: &str) -> anyhow::Result<Pipeline> {
    let url = format!(
        "{}/projects/{}/pipelines?ref={}&per_page=1&order_by=id&sort=desc",
        api_url,
        encode_project(project),
        urlencoding::encode(r#ref),
    );

    let response = client.get(&url).send()?;
    let response = super::check_response(response, &url)?;

    let pipelines: Vec<Pipeline> = response.json()?;

    pipelines.into_iter().next().context(format!("No pipeline found for ref '{}'", r#ref))
}

pub fn get_pipeline_by_sha(client: &Client, api_url: &str, project: &str, sha: &str) -> anyhow::Result<Pipeline> {
    let url = format!(
        "{}/projects/{}/pipelines?sha={}&per_page=1&order_by=id&sort=desc",
        api_url,
        encode_project(project),
        urlencoding::encode(sha),
    );

    let response = client.get(&url).send()?;
    let response = super::check_response(response, &url)?;

    let pipelines: Vec<Pipeline> = response.json()?;

    pipelines.into_iter().next().context(format!("No pipeline found for commit '{}'", sha))
}

pub fn find_job(
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

    let response = client.get(&url).send()?;
    let response = super::check_response(response, &url)?;

    let jobs: Vec<Job> = response.json()?;

    jobs.into_iter()
        .find(|j| j.name == job_name)
        .with_context(|| format!("Job '{}' not found in pipeline {}", job_name, pipeline_id))
}

pub fn download_artifacts(
    client: &Client,
    api_url: &str,
    project: &str,
    job_id: u64,
    output: &std::path::Path,
) -> anyhow::Result<()> {
    let url = format!("{}/projects/{}/jobs/{}/artifacts", api_url, encode_project(project), job_id);

    let response = client.get(&url).send()?;
    let response = super::check_response(response, &url)?;

    let bytes = response.bytes()?;

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output, &bytes)?;
    println!("Artifacts saved to {}", output.display());

    Ok(())
}
