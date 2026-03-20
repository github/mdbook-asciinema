use crate::types::*;
use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{Write, copy};
use std::iter::Iterator;
use tracing::info;

impl GitHubResponse for serde_json::Value {
    fn get_asset_download_url(&self, asset_name: &str) -> Result<String> {
        let value = self
            .as_object()
            .ok_or_else(|| anyhow!("result should be json object"))?
            .get("assets")
            .ok_or_else(|| anyhow!("result should specify assets"))?
            .as_array()
            .ok_or_else(|| anyhow!("assets should be json array"))?
            .iter()
            .find(|&value| {
                let Some(asset) = value.as_object() else {
                    return false;
                };

                let Some(value_name) = asset.get("name") else {
                    return false;
                };

                let Some(name) = value_name.as_str() else {
                    return false;
                };

                name == asset_name
            })
            .ok_or_else(|| anyhow!("{asset_name} is missing"))?
            .as_object()
            .ok_or_else(|| anyhow!("asset should be json object"))?
            .get("browser_download_url")
            .ok_or_else(|| anyhow!("download url for {asset_name} is missing"))?
            .as_str()
            .ok_or_else(|| anyhow!("download url should be a string"))?
            .to_string();
        Ok(value)
    }
}

pub(crate) fn fetch_and_save_binary(
    url: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("download {url} to {file_path}");
    let mut response = ureq::get(url).call()?;
    let mut file = File::create(file_path)?;
    file.write_all(crate::LICENSE_HEADER.as_bytes())?;
    copy(&mut response.body_mut().as_reader(), &mut file)?;
    Ok(())
}
