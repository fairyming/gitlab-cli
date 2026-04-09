use anyhow::Context;
use reqwest::Client;

pub struct GitLabConfig {
    pub api_url: String,
    pub token: String,
}

pub fn load_config() -> anyhow::Result<GitLabConfig> {
    let api_url = std::env::var("GITLAB_API_URL")
        .context("GITLAB_API_URL environment variable is not set")?;

    let token = std::env::var("GITLAB_PRIVATE_TOKEN")
        .or_else(|_| std::env::var("GITLAB_TOKEN"))
        .or_else(|_| std::env::var("CI_JOB_TOKEN"))
        .context(
            "No token found. Set one of: GITLAB_PRIVATE_TOKEN, GITLAB_TOKEN, or CI_JOB_TOKEN",
        )?;

    Ok(GitLabConfig { api_url, token })
}

pub fn build_client(config: &GitLabConfig, insecure: bool) -> anyhow::Result<Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "PRIVATE-TOKEN",
        reqwest::header::HeaderValue::from_str(&config.token)?,
    );
    let mut builder = Client::builder().default_headers(headers);
    if insecure {
        builder = builder.danger_accept_invalid_certs(true);
    }
    Ok(builder.build()?)
}
