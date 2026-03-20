mod types;

use anyhow::{anyhow, Result};
use clap::Parser;
use crate::types::*;
use mdbook_asciinema::PlaceholderPreprocessor;
use mdbook_preprocessor::Preprocessor;
use tracing::error;
use std::io;

fn main() {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .without_time()
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_writer(std::io::stderr)
        .init();

    let result = match cli.command {
        Some(Command::Supports { renderer }) => supports(renderer.as_str()),
        None => preprocessor(),
    };

    if let Err(err) = result {
        error!("{err}");
        std::process::exit(1);
    }
}

fn supports(renderer: &str) -> Result<()> {
    match PlaceholderPreprocessor::new().supports_renderer(renderer) {
        Ok(true) => Ok(()),
        Ok(false) => Err(anyhow!("mdbook-asciinema does not support {renderer}")),
        Err(err) => Err(err)
    }
}

fn preprocessor() -> Result<()> {
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    let preprocessor = PlaceholderPreprocessor::new();
    let processed_book = preprocessor.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
