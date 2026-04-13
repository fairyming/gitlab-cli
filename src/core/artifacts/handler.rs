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
    extract: bool,
) -> anyhow::Result<()> {
    println!("Looking for pipeline by ref '{}'...", branch);
    let pipeline = api::get_pipeline_by_ref(gitlab, project, branch)?;
    println!("Found pipeline: {}", pipeline.id);

    println!("Searching for job '{}'...", job_name);
    let job = api::find_job(gitlab, project, pipeline.id, job_name)?;
    println!("Found job: {} (id={})", job.name, job.id);

    println!("Downloading artifacts...");
    api::download_artifacts(gitlab, project, job.id, output, extract)?;

    Ok(())
}

fn download_by_commit(
    gitlab: &GitLabClient,
    project: &str,
    sha: &str,
    job_name: &str,
    output: &Path,
    extract: bool,
) -> anyhow::Result<()> {
    println!("Looking for pipeline by commit '{}'...", sha);
    let pipeline = api::get_pipeline_by_sha(gitlab, project, sha)?;
    println!("Found pipeline: {}", pipeline.id);

    println!("Searching for job '{}'...", job_name);
    let job = api::find_job(gitlab, project, pipeline.id, job_name)?;
    println!("Found job: {} (id={})", job.name, job.id);

    println!("Downloading artifacts...");
    api::download_artifacts(gitlab, project, job.id, output, extract)?;

    Ok(())
}

pub fn run(gitlab: &GitLabClient, action: ArtifactsAction) -> anyhow::Result<()> {
    match action {
        ArtifactsAction::Download { project, branch, commit, job, output, extract } => {
            match (branch.as_deref(), commit.as_deref()) {
                (Some(b), None) => download_by_branch(gitlab, &project, b, &job, &output, extract),
                (None, Some(c)) => download_by_commit(gitlab, &project, c, &job, &output, extract),
                _ => bail!("Must specify exactly one of --branch or --commit"),
            }
        }
    }
}
