use std::path::PathBuf;

use clap::Subcommand;

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
    },

    /// Delete a package by ID
    Delete {
        /// Project ID or path
        #[arg(long)]
        project: String,

        /// Package ID
        #[arg(long)]
        id: u64,
    },

    /// List packages in the project
    List {
        /// Project ID or path
        #[arg(long)]
        project: String,

        /// Filter by package name
        #[arg(long)]
        name: Option<String>,
    },
}
