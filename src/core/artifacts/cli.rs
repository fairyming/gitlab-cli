use std::path::PathBuf;

use clap::Subcommand;

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

        /// Extract the zip archive after downloading
        #[arg(long)]
        extract: bool,
    },
}
