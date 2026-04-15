use std::collections::HashMap;

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

use super::types::{encode_project, GraphQlResponse, JobDetail, Pipeline, TriggeredJob};
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

pub fn find_job(gitlab: &GitLabClient, project: &str, pipeline_id: u64, job_name: &str) -> anyhow::Result<JobDetail> {
    let url =
        format!("{}/projects/{}/pipelines/{}/jobs?per_page=100", gitlab.api_url, encode_project(project), pipeline_id);

    let body = gitlab.get(&url)?;
    let jobs: Vec<JobDetail> = serde_json::from_slice(&body)?;
    jobs.into_iter()
        .find(|j| j.name == job_name)
        .with_context(|| format!("Job '{}' not found in pipeline {}", job_name, pipeline_id))
}

pub fn play_job(
    gitlab: &GitLabClient,
    project: &str,
    job_id: u64,
    variables: &HashMap<String, String>,
    inputs: &HashMap<String, String>,
) -> anyhow::Result<TriggeredJob> {
    let url = format!("{}/projects/{}/jobs/{}/play", gitlab.api_url, encode_project(project), job_id);

    let body = build_play_body(variables, inputs);
    let res = gitlab.post_json(&url, &body)?;
    let job: TriggeredJob = serde_json::from_slice(&res)?;
    Ok(job)
}

pub fn retry_job_with_variables(
    gitlab: &GitLabClient,
    job_id: u64,
    variables: &HashMap<String, String>,
    inputs: &HashMap<String, String>,
) -> anyhow::Result<TriggeredJob> {
    let vars: Vec<GraphQlVariable> = variables.iter().map(|(k, v)| GraphQlVariable { key: k, value: v }).collect();
    let inp: Vec<GraphQlInput> = inputs.iter().map(|(k, v)| GraphQlInput { name: k, value: v }).collect();

    let request = TriggerJobArgs {
        operation_name: "retryJobWithVariables",
        query: "mutation retryJobWithVariables($id: CiProcessableID!, $variables: [CiVariableInput!], $inputs: [CiInputsInput!]) {\n  jobRetry(input: {id: $id, variables: $variables, inputs: $inputs}) {\n    job {\n      ...BaseCiJob\n      webPath\n      __typename\n    }\n    errors\n    __typename\n  }\n}\n\nfragment BaseCiJob on CiJob {\n  id\n  manualVariables {\n    nodes {\n      ...ManualCiVariable\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n\nfragment ManualCiVariable on CiVariable {\n  id\n  key\n  value\n  __typename\n}",
        variables: TriggerJobVariables {
            id: format!("gid://gitlab/Ci::Build/{}", job_id),
            variables: vars,
            inputs: inp,
        },
    };

    let res = gitlab.graphql(&request)?;
    let response: GraphQlResponse<RetryJobData> = serde_json::from_slice(&res)?;

    if let Some(errors) = &response.errors {
        let messages: Vec<&str> = errors.iter().map(|e| e.message.as_str()).collect();
        bail!("GraphQL errors: {}", messages.join(", "));
    }

    let data = response.data.context("GraphQL response missing data")?;
    let new_job_id = parse_gid_id(&data.job_retry.job.id)?;
    Ok(TriggeredJob { id: new_job_id })
}

pub fn retry_job(
    gitlab: &GitLabClient,
    project: &str,
    job_id: u64,
    inputs: &HashMap<String, String>,
) -> anyhow::Result<TriggeredJob> {
    let url = format!("{}/projects/{}/jobs/{}/retry", gitlab.api_url, encode_project(project), job_id);
    if inputs.is_empty() {
        let body = gitlab.post(&url)?;
        let job: TriggeredJob = serde_json::from_slice(&body)?;
        Ok(job)
    } else {
        // 实际参数为 inputs: https://gitlab.com/gitlab-org/gitlab/-/blob/master/lib/api/ci/jobs.rb?ref_type=heads#L168
        let body = RetryBody { inputs };
        let res = gitlab.post_json(&url, &body)?;
        let job: TriggeredJob = serde_json::from_slice(&res)?;
        Ok(job)
    }
}

// --- GraphQL request types ---

#[derive(Serialize)]
struct TriggerJobArgs<'a> {
    #[serde(rename = "operationName")]
    operation_name: &'a str,
    query: &'a str,
    variables: TriggerJobVariables<'a>,
}

#[derive(Serialize)]
struct TriggerJobVariables<'a> {
    id: String,
    variables: Vec<GraphQlVariable<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    inputs: Vec<GraphQlInput<'a>>,
}

#[derive(Serialize)]
struct GraphQlVariable<'a> {
    key: &'a str,
    value: &'a str,
}

#[derive(Serialize)]
struct GraphQlInput<'a> {
    name: &'a str,
    value: &'a str,
}

// --- GraphQL response types ---

#[derive(Deserialize)]
struct RetryJobData {
    #[serde(rename = "jobRetry")]
    job_retry: RetryJobPayload,
}

#[derive(Deserialize)]
struct RetryJobPayload {
    job: RetryJobNode,
}

#[derive(Deserialize)]
struct RetryJobNode {
    id: String,
}

fn parse_gid_id(gid: &str) -> anyhow::Result<u64> {
    let suffix = gid.rsplit('/').next().context("Invalid GID format")?;
    suffix.parse::<u64>().context("Invalid job ID in GID")
}

// --- play job body ---

#[derive(Serialize)]
struct PlayBody<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    job_variables_attributes: Vec<PlayVariable<'a>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    job_inputs: &'a HashMap<String, String>,
}

#[derive(Serialize)]
struct RetryBody<'a> {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    inputs: &'a HashMap<String, String>,
}

#[derive(Serialize)]
struct PlayVariable<'a> {
    key: &'a str,
    value: &'a str,
}

fn build_play_body<'a>(variables: &'a HashMap<String, String>, inputs: &'a HashMap<String, String>) -> PlayBody<'a> {
    let attrs: Vec<PlayVariable<'a>> = variables.iter().map(|(k, v)| PlayVariable { key: k, value: v }).collect();
    PlayBody { job_variables_attributes: attrs, job_inputs: inputs }
}

pub fn check_job_status(status: &str) -> anyhow::Result<()> {
    match status {
        "running" | "pending" => bail!("Job is currently '{}' and cannot be triggered", status),
        _ => Ok(()),
    }
}
