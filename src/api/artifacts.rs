use super::types::encode_project;
use crate::client::GitLabClient;

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
