mod types;
mod utils;

use crate::{types::*, utils::*};
use anyhow::{Result, anyhow};
use clap::Parser;
use tracing::{error, info};

const LICENSE_HEADER: &str = r#"/* Licensed to Marcin Kulik under the Apache License, Version 2.0. */
/* For license information please see https://github.com/asciinema/asciinema-player/blob/develop/LICENSE */
"#;

const ASCIINEMA_FILES: &[&str] = &["asciinema-player.css", "asciinema-player.min.js"];

fn main() {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .without_time()
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_writer(std::io::stderr)
        .init();

    let result = match cli.command {
        Command::Update { version } => update(version),
    };

    if let Err(err) = result {
        error!("{err}");
        std::process::exit(1);
    }
}

fn update(version: String) -> Result<()> {
    let release_url = match version.as_str() {
        "latest" => {
            "https://api.github.com/repos/asciinema/asciinema-player/releases/latest".to_string()
        }
        _ => format!(
            "https://api.github.com/repos/asciinema/asciinema-player/releases/tags/{version}"
        ),
    };

    info!("fetch release information for asciinema - {version}");

    let release = ureq::get(release_url)
        .call()
        .map_err(|_| anyhow!("release does not exist"))?
        .body_mut()
        .read_json::<serde_json::Value>()
        .map_err(|_| anyhow!("invalid response, not json"))?;

    for &asset_name in ASCIINEMA_FILES {
        let download_url = release.get_asset_download_url(asset_name)?;
        let file_path = format!("mdbook-asciinema/assets/{asset_name}");
        fetch_and_save_binary(&download_url, &file_path)
            .map_err(|_| anyhow!("unable to fetch asset"))?;
    }

    Ok(())
}
