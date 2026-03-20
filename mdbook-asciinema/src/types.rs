use clap::{Parser, Subcommand, crate_description, crate_name, crate_version};
use rust_embed::Embed;

#[derive(Debug, Parser)]
#[command(name = crate_name!())]
#[command(version = crate_version!())]
#[command(about = crate_description!())]
pub(super) struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub(super) enum Command {
    /// Check whether a renderer is supported by this preprocessor
    Supports {
        #[arg(required = true)]
        renderer: String,
    },
}

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/assets/"]
#[allow(dead_code)]
pub(super) struct Asset;
