use clap::Subcommand;

#[derive(Subcommand)]
pub enum PipelineAction {
    /// Trigger a specific job in a pipeline
    Run {
        /// Project ID or path
        #[arg(long)]
        project: String,

        /// Branch name to find pipeline
        #[arg(long, group = "ref")]
        branch: Option<String>,

        /// Commit SHA to find pipeline
        #[arg(long, group = "ref")]
        commit: Option<String>,

        /// Job name to trigger
        #[arg(long)]
        job: String,

        /// Environment variables (e.g. --env KEY=VALUE), can be specified multiple times
        #[arg(long = "env", value_name = "KEY=VALUE")]
        variables: Vec<String>,
    },
}
