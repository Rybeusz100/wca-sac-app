use std::{io::Cursor, path::PathBuf};

use anyhow::anyhow;
use log::info;

pub async fn download_and_unzip(target_dir: &str) -> anyhow::Result<()> {
    info!("Downloading WCA export");
    let zip_bytes = download().await?;
    info!("Downloaded WCA export");

    info!("Extracting WCA export");
    zip_extract::extract(Cursor::new(zip_bytes), &PathBuf::from(target_dir), true)?;
    info!("Extracted WCA export");

    Ok(())
}

async fn download() -> anyhow::Result<Vec<u8>> {
    let client = reqwest::Client::new();
    let url = "https://www.worldcubeassociation.org/export/results/WCA_export.tsv";
    let mut response = client.get(url).header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36").send().await?;

    if response.status().is_success() {
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            bytes.extend_from_slice(&chunk);
        }
        Ok(bytes)
    } else {
        Err(anyhow!(
            "Server responded with status: {}",
            response.status()
        ))
    }
}
