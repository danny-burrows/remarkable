use std::path::PathBuf;

/// The configuration parameters for the remarkable server.
///
/// These can either be passed on the command line, or in some cases, pulled from environment variables.
#[derive(Clone, clap::Parser)]
pub struct Config {
    /// Path to the directory structure containing markdown files to serve.
    #[clap(long)]
    pub(crate) markdown_dir: PathBuf,
}
