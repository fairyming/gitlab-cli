mod api;
mod client;
mod core;

use core::{
    artifacts::{run as artifacts_run, ArtifactsAction},
    packages::{run as packages_run, PackageAction},
};

use clap::{Args, Parser, Subcommand};
use client::GitLabClient;

/// GitLab client connection settings
#[derive(Args, Clone)]
#[command(next_help_heading = "Client")]
struct ClientArgs {
    /// GitLab API URL (e.g. https://gitlab.com/api/v4)
    #[arg(long, env = "GITLAB_API_URL")]
    api_url: Option<String>,

    /// Access token (overrides env GITLAB_PRIVATE_TOKEN)
    #[arg(long)]
    token: Option<String>,

    /// Skip TLS certificate verification
    #[arg(long)]
    insecure: bool,
}

#[derive(Parser)]
#[command(name = "gitlab-cli", version, about = "GitLab CLI tool")]
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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let gitlab =
        GitLabClient::new(cli.client.api_url.unwrap_or_default(), cli.client.token.as_deref(), cli.client.insecure)?;

    match cli.command {
        Commands::Artifacts { action } => artifacts_run(&gitlab, action)?,
        Commands::Package { action } => packages_run(&gitlab, action)?,
    }

    Ok(())
}
