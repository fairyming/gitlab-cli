use anyhow::Context;
use tracing::debug;

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

    /// Stream download: send GET request, check status, then stream response body to file.
    pub fn download(&self, url: &str, path: &std::path::Path) -> anyhow::Result<()> {
        use std::io::{Read, Write};

        log_request("GET", url);
        let mut response = self.client.get(url).send()?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().unwrap_or_else(|_| "<failed to read response body>".to_string());
            let truncated = &body[..body.len().min(1024)];
            debug!("response body: {}", truncated);
            anyhow::bail!("API request failed\n  URL: {url}\n  Status: {status}\n  Response: {truncated}")
        }

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = std::fs::File::create(path)?;
        let mut buf = [0u8; 8 * 1024];
        let mut first = true;
        loop {
            let n = response.read(&mut buf)?;
            if n == 0 {
                break;
            }
            if first {
                debug!("response body: {}", String::from_utf8_lossy(&buf[..n.min(1024)]));
                first = false;
            }
            file.write_all(&buf[..n])?;
        }
        Ok(())
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

    /// POST request (no body): log + send + status check + read body.
    pub fn post(&self, url: &str) -> anyhow::Result<Vec<u8>> {
        log_request("POST", url);
        let response = self.client.post(url).send()?;
        read_response(response, url)
    }

    /// POST request with JSON body: log + send + status check + read body.
    pub fn post_json(&self, url: &str, body: &impl serde::Serialize) -> anyhow::Result<Vec<u8>> {
        log_request("POST", url);
        debug!("POST JSON body: {:?}", serde_json::to_string(body)?);
        let response = self.client.post(url).json(body).send()?;
        read_response(response, url)
    }

    /// GraphQL request: POST JSON body to /api/graphql.
    pub fn graphql(&self, body: &impl serde::Serialize) -> anyhow::Result<Vec<u8>> {
        let url = format!("{}/../graphql", self.api_url);
        debug!("GraphQL request body: {:?}", serde_json::to_string(body)?);
        log_request("POST", &url);
        let response = self.client.post(&url).json(body).send()?;
        read_response(response, &url)
    }
}
