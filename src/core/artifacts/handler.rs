use std::path::Path;

use anyhow::bail;

use super::cli::ArtifactsAction;
use crate::{api, client::GitLabClient};

fn download_by_branch(
    gitlab: &GitLabClient,
    project: &str,
    branch: &str,
    job_name: &str,
    output: &Path,
) -> anyhow::Result<()> {
    println!("Looking for pipeline by ref '{}'...", branch);
    let pipeline = api::get_pipeline_by_ref(&gitlab.client, &gitlab.api_url, project, branch)?;
    println!("Found pipeline: {}", pipeline.id);

    println!("Searching for job '{}'...", job_name);
    let job = api::find_job(&gitlab.client, &gitlab.api_url, project, pipeline.id, job_name)?;
    println!("Found job: {} (id={})", job.name, job.id);

    println!("Downloading artifacts...");
    api::download_artifacts(&gitlab.client, &gitlab.api_url, project, job.id, output)?;

    Ok(())
}

fn download_by_commit(
    gitlab: &GitLabClient,
    project: &str,
    sha: &str,
    job_name: &str,
    output: &Path,
) -> anyhow::Result<()> {
    println!("Looking for pipeline by commit '{}'...", sha);
    let pipeline = api::get_pipeline_by_sha(&gitlab.client, &gitlab.api_url, project, sha)?;
    println!("Found pipeline: {}", pipeline.id);

    println!("Searching for job '{}'...", job_name);
    let job = api::find_job(&gitlab.client, &gitlab.api_url, project, pipeline.id, job_name)?;
    println!("Found job: {} (id={})", job.name, job.id);

    println!("Downloading artifacts...");
    api::download_artifacts(&gitlab.client, &gitlab.api_url, project, job.id, output)?;

    Ok(())
}

pub fn run(gitlab: &GitLabClient, action: ArtifactsAction) -> anyhow::Result<()> {
    match action {
        ArtifactsAction::Download { project, branch, commit, job, output } => {
            match (branch.as_deref(), commit.as_deref()) {
                (Some(b), None) => download_by_branch(gitlab, &project, b, &job, &output),
                (None, Some(c)) => download_by_commit(gitlab, &project, c, &job, &output),
                _ => bail!("Must specify exactly one of --branch or --commit"),
            }
        }
    }
}
