pub mod artifacts;
pub mod packages;
pub mod types;

pub use artifacts::*;
pub use packages::*;

/// Check response status and return error with status code + response body on failure.
fn check_response(response: reqwest::blocking::Response, url: &str) -> anyhow::Result<reqwest::blocking::Response> {
    if response.status().is_success() {
        return Ok(response);
    }

    let status = response.status();
    let body = response.text().unwrap_or_else(|_| "<failed to read response body>".to_string());

    let truncated = &body[..body.len().min(512)];
    anyhow::bail!("API request failed\n  URL: {url}\n  Status: {status}\n  Response: {truncated}")
}
