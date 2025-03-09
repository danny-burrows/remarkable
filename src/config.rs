use std::path::PathBuf;

/// The configuration parameters for the remarkable server.
///
/// These can either be passed on the command line, or in some cases, pulled from environment variables.
#[derive(Clone, clap::Parser)]
pub struct Config {
    /// Path to the directory structure containing markdown files to serve.
    #[clap(long)]
    pub(crate) markdown_dir: PathBuf,

    /// The name of the theme, represented by a directory of stylesheets in `./themes`.
    #[clap(long, default_value = "basic")]
    pub(crate) theme: String,

    /// The address for the server to bind to. (e.g. `localhost:3000`)
    #[clap(long, default_value = "localhost:3000")]
    pub(crate) bind_address: String,
}
