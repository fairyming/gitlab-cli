use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gitlab-cli", version, about = "GitLab CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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

#[derive(Subcommand)]
pub enum ArtifactsAction {
    /// Download artifacts as a zip file
    Download {
        /// Project ID or path
        #[arg(long)]
        project: String,

        /// Branch name to find pipeline
        #[arg(long, group = "ref")]
        branch: Option<String>,

        /// Commit SHA to find pipeline
        #[arg(long, group = "ref")]
        commit: Option<String>,

        /// Job name to download artifacts from
        #[arg(long)]
        job: String,

        /// Output file path for the zip archive
        #[arg(long, default_value = "artifacts.zip")]
        output: PathBuf,

        /// Skip TLS certificate verification
        #[arg(long, global = true)]
        insecure: bool,
    },
}

#[derive(Subcommand)]
pub enum PackageAction {
    /// Upload a generic package
    Upload {
        /// Project ID or path
        #[arg(long)]
        project: String,

        /// Package name
        #[arg(long)]
        name: String,

        /// Package version
        #[arg(long)]
        version: String,

        /// File to upload
        #[arg(long)]
        file: PathBuf,

        /// Skip TLS certificate verification
        #[arg(long, global = true)]
        insecure: bool,
    },

    /// Download a generic package
    Download {
        /// Project ID or path
        #[arg(long)]
        project: String,

        /// Package name
        #[arg(long)]
        name: String,

        /// Package version
        #[arg(long)]
        version: String,

        /// File name in the package registry
        #[arg(long)]
        file: String,

        /// Output file path
        #[arg(long)]
        output: PathBuf,

        /// Skip TLS certificate verification
        #[arg(long, global = true)]
        insecure: bool,
    },

    /// Delete a package by ID
    Delete {
        /// Project ID or path
        #[arg(long)]
        project: String,

        /// Package ID
        #[arg(long)]
        id: u64,

        /// Skip TLS certificate verification
        #[arg(long, global = true)]
        insecure: bool,
    },

    /// List packages in the project
    List {
        /// Project ID or path
        #[arg(long)]
        project: String,

        /// Filter by package name
        #[arg(long)]
        name: Option<String>,

        /// Skip TLS certificate verification
        #[arg(long, global = true)]
        insecure: bool,
    },
}
