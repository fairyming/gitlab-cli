mod api;
mod client;
mod core;

use core::{
    artifacts::{run as artifacts_run, ArtifactsAction},
    packages::{run as packages_run, PackageAction},
    pipeline::{run as pipeline_run, PipelineAction},
};

use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use client::GitLabClient;

/// GitLab client connection settings
#[derive(Args, Clone)]
#[command(next_help_heading = "Client")]
struct ClientArgs {
    /// GitLab API URL (e.g. https://gitlab.com/api/v4)
    #[arg(long, env = "CI_API_V4_URL")]
    api_url: Option<String>,

    /// Use PRIVATE-TOKEN (from --private-token or GITLAB_PRIVATE_TOKEN)
    #[arg(long)]
    use_private_token: bool,

    /// Personal access token (overrides env GITLAB_PRIVATE_TOKEN)
    #[arg(long, env = "GITLAB_PRIVATE_TOKEN")]
    private_token: Option<String>,

    /// CI job token (overrides env CI_JOB_TOKEN)
    #[arg(long, env = "CI_JOB_TOKEN")]
    job_token: Option<String>,

    /// Skip TLS certificate verification
    #[arg(long)]
    insecure: bool,

    /// Log level for HTTP tracing (trace, debug, info, warn, error)
    #[arg(long, default_value = "warn")]
    log_level: String,
}

#[derive(Parser)]
#[command(name = "gitlab-cli", version = env!("GIT_VERSION"), about = "GitLab CLI tool")]
struct Cli {
    #[command(flatten)]
    client: ClientArgs,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download CI/CD artifacts
    Artifacts {
        #[command(subcommand)]
        action: ArtifactsAction,
    },

    /// Manage Package Registry
    Package {
        #[command(subcommand)]
        action: PackageAction,
    },

    /// Manage CI/CD pipelines
    Pipeline {
        #[command(subcommand)]
        action: PipelineAction,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let log_level: tracing::Level = cli.client.log_level.parse().map_err(|_| {
        anyhow::anyhow!("Invalid log level '{}'. Valid: trace, debug, info, warn, error", cli.client.log_level)
    })?;
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(log_level.into())
                .from_env_lossy()
                .add_directive(format!("reqwest={}", log_level).parse()?)
                .add_directive(format!("hickory_dns={}", log_level).parse()?),
        )
        .with_target(false)
        .init();

    let api_url = cli.client.api_url.context("--api-url or CI_API_V4_URL is required")?;
    let gitlab = GitLabClient::new(
        api_url,
        cli.client.use_private_token,
        cli.client.private_token.as_deref(),
        cli.client.job_token.as_deref(),
        cli.client.insecure,
    )?;

    match cli.command {
        Commands::Artifacts { action } => artifacts_run(&gitlab, action)?,
        Commands::Package { action } => packages_run(&gitlab, action)?,
        Commands::Pipeline { action } => pipeline_run(&gitlab, action)?,
    }

    Ok(())
}
