use reqwest::blocking::Client;

#[derive(Clone, PartialEq)]
enum TokenKind {
    Private,
    Job,
}

pub struct GitLabClient {
    pub api_url: String,
    pub client: Client,
}

impl GitLabClient {
    pub fn new(api_url: String, cli_token: Option<&str>, insecure: bool) -> anyhow::Result<Self> {
        let (token, token_kind) = if let Some(token) = cli_token {
            (token.to_string(), TokenKind::Private)
        } else if let Ok(token) = std::env::var("GITLAB_PRIVATE_TOKEN") {
            (token, TokenKind::Private)
        } else if let Ok(token) = std::env::var("CI_JOB_TOKEN") {
            (token, TokenKind::Job)
        } else {
            anyhow::bail!("No token found. Set one of: GITLAB_PRIVATE_TOKEN, CI_JOB_TOKEN, or use --token");
        };

        let mut headers = reqwest::header::HeaderMap::new();
        let header_name = match token_kind {
            TokenKind::Private => "PRIVATE-TOKEN",
            TokenKind::Job => "JOB-TOKEN",
        };
        headers.insert(header_name, reqwest::header::HeaderValue::from_str(&token)?);

        let mut builder = Client::builder().default_headers(headers);
        if insecure {
            builder = builder.danger_accept_invalid_certs(true);
        }

        Ok(GitLabClient { api_url, client: builder.build()? })
    }
}
