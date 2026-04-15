use std::collections::HashMap;

use anyhow::{bail, Context};

use super::cli::PipelineAction;
use crate::{api, client::GitLabClient};

fn parse_variables(env_list: &[String]) -> anyhow::Result<HashMap<String, String>> {
    let mut vars = HashMap::new();
    for item in env_list {
        let (key, value) =
            item.split_once('=').with_context(|| format!("Invalid variable format '{}', expected KEY=VALUE", item))?;
        vars.insert(key.to_string(), value.to_string());
    }
    Ok(vars)
}

fn trigger_job(
    gitlab: &GitLabClient,
    project: &str,
    r#ref: &str,
    is_branch: bool,
    job_name: &str,
    variables: &HashMap<String, String>,
    inputs: &HashMap<String, String>,
) -> anyhow::Result<()> {
    let ref_label = if is_branch { "branch" } else { "commit" };
    println!("Looking for pipeline by {} '{}'...", ref_label, r#ref);

    let pipeline = if is_branch {
        api::get_pipeline_by_ref(gitlab, project, r#ref)?
    } else {
        api::get_pipeline_by_sha(gitlab, project, r#ref)?
    };
    println!("Found pipeline: {}", pipeline.id);

    println!("Searching for job '{}'...", job_name);
    let job = api::find_job(gitlab, project, pipeline.id, job_name)?;
    println!("Found job: {} (id={}, status={})", job.name, job.id, job.status);

    api::check_job_status(&job.status)?;

    // 存在一点小问题，目前不支持 retry 时清空所有 inputs 或 variables
    let triggered = match job.status.as_str() {
        "manual" => {
            println!("Playing manual job...");
            api::play_job(gitlab, project, job.id, variables, inputs)?
        }
        _ => {
            if !variables.is_empty() {
                println!("Retrying job with custom variables via GraphQL...");
                api::retry_job_with_variables(gitlab, job.id, variables, inputs)?
            } else {
                println!("Retrying job...");
                api::retry_job(gitlab, project, job.id, inputs)?
            }
        }
    };

    println!("Job triggered successfully, new job id: {}", triggered.id);
    Ok(())
}

pub fn run(gitlab: &GitLabClient, action: PipelineAction) -> anyhow::Result<()> {
    match action {
        PipelineAction::Run { project, branch, commit, job, variables, inputs } => {
            let vars = parse_variables(&variables)?;
            let inp = parse_variables(&inputs)?;
            match (branch.as_deref(), commit.as_deref()) {
                (Some(b), None) => trigger_job(gitlab, &project, b, true, &job, &vars, &inp),
                (None, Some(c)) => trigger_job(gitlab, &project, c, false, &job, &vars, &inp),
                _ => bail!("Must specify exactly one of --branch or --commit"),
            }
        }
    }
}
