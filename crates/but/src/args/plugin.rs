//! Command-line argument definitions for the `but plugin` command.

#[derive(Debug, clap::Parser)]
pub struct Platform {
    #[clap(subcommand)]
    pub cmd: Option<Subcommands>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// List all external plugins found on PATH
    List,
    /// Show PATH directories searched for plugins
    Path,
}
