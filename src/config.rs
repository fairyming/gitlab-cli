use anyhow::Context;
use reqwest::Client;

#[derive(Clone, PartialEq)]
pub enum TokenKind {
    Private,
    Job,
}

pub struct GitLabConfig {
    pub api_url: String,
    pub token: String,
    pub token_kind: TokenKind,
}

pub fn load_config() -> anyhow::Result<GitLabConfig> {
    let api_url = std::env::var("GITLAB_API_URL")
        .context("GITLAB_API_URL environment variable is not set")?;

    if let Ok(token) = std::env::var("GITLAB_PRIVATE_TOKEN") {
        return Ok(GitLabConfig {
            api_url,
            token,
            token_kind: TokenKind::Private,
        });
    }

    if let Ok(token) = std::env::var("GITLAB_TOKEN") {
        return Ok(GitLabConfig {
            api_url,
            token,
            token_kind: TokenKind::Private,
        });
    }

    if let Ok(token) = std::env::var("CI_JOB_TOKEN") {
        return Ok(GitLabConfig {
            api_url,
            token,
            token_kind: TokenKind::Job,
        });
    }

    anyhow::bail!(
        "No token found. Set one of: GITLAB_PRIVATE_TOKEN, GITLAB_TOKEN, or CI_JOB_TOKEN"
    );
}

pub fn build_client(config: &GitLabConfig, insecure: bool) -> anyhow::Result<Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    let header_name = match config.token_kind {
        TokenKind::Private => "PRIVATE-TOKEN",
        TokenKind::Job => "JOB-TOKEN",
    };
    headers.insert(
        header_name,
        reqwest::header::HeaderValue::from_str(&config.token)?,
    );
    let mut builder = Client::builder().default_headers(headers);
    if insecure {
        builder = builder.danger_accept_invalid_certs(true);
    }
    Ok(builder.build()?)
}
