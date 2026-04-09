mod api;
mod cli;
mod cmd;
mod config;

use clap::Parser;
use cli::{Cli, Commands, ArtifactsAction, PackageAction};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Artifacts { action } => match action {
            ArtifactsAction::Download {
                project,
                branch,
                commit,
                job,
                output,
                insecure,
            } => {
                cmd::handle_download(
                    &project,
                    branch.as_deref(),
                    commit.as_deref(),
                    &job,
                    &output,
                    insecure,
                )
                .await?;
            }
        },

        Commands::Package { action } => match action {
            PackageAction::Upload {
                project,
                name,
                version,
                file,
                insecure,
            } => {
                cmd::handle_package_upload(&project, &name, &version, &file, insecure).await?;
            }

            PackageAction::Download {
                project,
                name,
                version,
                file,
                output,
                insecure,
            } => {
                cmd::handle_package_download(&project, &name, &version, &file, &output, insecure)
                    .await?;
            }

            PackageAction::Delete {
                project,
                id,
                insecure,
            } => {
                cmd::handle_package_delete(&project, id, insecure).await?;
            }

            PackageAction::List {
                project,
                name,
                insecure,
            } => {
                cmd::handle_package_list(&project, name.as_deref(), insecure).await?;
            }
        },
    }

    Ok(())
}
