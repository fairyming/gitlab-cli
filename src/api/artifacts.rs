use anyhow::Context;

use super::types::{encode_project, Job, Pipeline};
use crate::client::GitLabClient;

pub fn get_pipeline_by_ref(gitlab: &GitLabClient, project: &str, r#ref: &str) -> anyhow::Result<Pipeline> {
    let url = format!(
        "{}/projects/{}/pipelines?ref={}&per_page=1&order_by=id&sort=desc",
        gitlab.api_url,
        encode_project(project),
        urlencoding::encode(r#ref),
    );

    let body = gitlab.get(&url)?;
    let pipelines: Vec<Pipeline> = serde_json::from_slice(&body)?;
    pipelines.into_iter().next().context(format!("No pipeline found for ref '{}'", r#ref))
}

pub fn get_pipeline_by_sha(gitlab: &GitLabClient, project: &str, sha: &str) -> anyhow::Result<Pipeline> {
    let url = format!(
        "{}/projects/{}/pipelines?sha={}&per_page=1&order_by=id&sort=desc",
        gitlab.api_url,
        encode_project(project),
        urlencoding::encode(sha),
    );

    let body = gitlab.get(&url)?;
    let pipelines: Vec<Pipeline> = serde_json::from_slice(&body)?;
    pipelines.into_iter().next().context(format!("No pipeline found for commit '{}'", sha))
}

pub fn find_job(gitlab: &GitLabClient, project: &str, pipeline_id: u64, job_name: &str) -> anyhow::Result<Job> {
    let url = format!(
        "{}/projects/{}/pipelines/{}/jobs?scope[]=success&per_page=100",
        gitlab.api_url,
        encode_project(project),
        pipeline_id
    );

    let body = gitlab.get(&url)?;
    let jobs: Vec<Job> = serde_json::from_slice(&body)?;
    jobs.into_iter()
        .find(|j| j.name == job_name)
        .with_context(|| format!("Job '{}' not found in pipeline {}", job_name, pipeline_id))
}

pub fn download_artifacts(
    gitlab: &GitLabClient,
    project: &str,
    job_id: u64,
    output: &std::path::Path,
) -> anyhow::Result<()> {
    let url = format!("{}/projects/{}/jobs/{}/artifacts", gitlab.api_url, encode_project(project), job_id);

    let bytes = gitlab.get(&url)?;

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output, &bytes)?;
    println!("Artifacts saved to {}", output.display());

    Ok(())
}
