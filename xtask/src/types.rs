use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "xtask")]
#[command(version = option_env!("APP_VERSION").as_deref().unwrap_or("dev"))]
#[command(about = "Maintenance tasks for mdbook-asciinema")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Update embeded assets.
    Update {
        /// Version of asset to download.
        #[arg(long, default_value = "latest")]
        version: String,
    },
}

pub(crate) trait GitHubResponse {
    fn get_asset_download_url(&self, asset_name: &str) -> Result<String>;
}
