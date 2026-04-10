use anyhow::Context;

use super::check::{log_request, read_response};

#[derive(Clone, PartialEq)]
pub enum TokenKind {
    Private,
    Job,
}

pub struct GitLabClient {
    pub api_url: String,
    client: reqwest::blocking::Client,
}

impl GitLabClient {
    pub fn new(
        api_url: String,
        use_private_token: bool,
        private_token: Option<&str>,
        job_token: Option<&str>,
        insecure: bool,
    ) -> anyhow::Result<Self> {
        let (token, token_kind) = if use_private_token {
            let token = private_token
                .map(String::from)
                .or_else(|| std::env::var("GITLAB_PRIVATE_TOKEN").ok())
                .context("--use-private-token is set but no PRIVATE_TOKEN found (use --private-token or set GITLAB_PRIVATE_TOKEN)")?;
            (token, TokenKind::Private)
        } else {
            let token = job_token
                .map(String::from)
                .or_else(|| std::env::var("CI_JOB_TOKEN").ok())
                .context("No JOB_TOKEN found (use --job-token, set CI_JOB_TOKEN, or add --use-private-token)")?;
            (token, TokenKind::Job)
        };

        let mut headers = reqwest::header::HeaderMap::new();
        let header_name = match token_kind {
            TokenKind::Private => "PRIVATE-TOKEN",
            TokenKind::Job => "JOB-TOKEN",
        };
        headers.insert(header_name, reqwest::header::HeaderValue::from_str(&token)?);

        let mut builder = reqwest::blocking::Client::builder().default_headers(headers);
        if insecure {
            builder = builder.danger_accept_invalid_certs(true);
        }

        Ok(GitLabClient { api_url, client: builder.build()? })
    }

    /// GET request: log + send + status check + read body.
    pub fn get(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        log_request("GET", url);
        let response = self.client.get(url).send()?;
        read_response(response, url)
    }

    /// PUT request: log + send + status check + read body.
    pub fn put(&self, url: &str, body: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        log_request("PUT", url);
        let response = self.client.put(url).body(body).send()?;
        read_response(response, url)
    }

    /// DELETE request: log + send + status check + read body.
    pub fn delete(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        log_request("DELETE", url);
        let response = self.client.delete(url).send()?;
        read_response(response, url)
    }
}
