use tracing::{debug, info, span, trace, Level};

/// Log request details at appropriate trace levels.
pub fn log_request(method: &str, url: &str) {
    let span = span!(Level::TRACE, "http_request", method = %method, url = %url);
    let _enter = span.enter();
    info!("--> {method} {url}");
    trace!("request headers will be logged at trace level");
}

pub fn read_response(response: reqwest::blocking::Response, url: &str) -> anyhow::Result<Vec<u8>> {
    let status = response.status();
    let version = response.version();
    let headers = response.headers();

    debug!("<-- {} {:?}", status.as_u16(), version);
    trace!("response headers: {:?}", headers);

    if status.is_success() {
        info!("<-- {} {}", status.as_u16(), url);
        let bytes = response.bytes()?;
        let preview = String::from_utf8_lossy(&bytes[..bytes.len().min(1024)]);
        debug!("response body [{}]: {}", url, preview);
        Ok(bytes.to_vec())
    } else {
        let body = response.text().unwrap_or_else(|_| "<failed to read response body>".to_string());
        let truncated = &body[..body.len().min(1024)];
        debug!("response body: {}", truncated);
        anyhow::bail!("API request failed\n  URL: {url}\n  Status: {status}\n  Response: {truncated}")
    }
}
