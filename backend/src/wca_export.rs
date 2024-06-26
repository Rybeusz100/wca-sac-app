use std::{io::Cursor, path::PathBuf};

use anyhow::anyhow;
use log::info;

pub async fn download_and_unzip(target_dir: &str) -> anyhow::Result<()> {
    info!("Downloading WCA export");
    let zip_bytes = download().await?;

    info!("Extracting WCA export");
    zip_extract::extract(Cursor::new(zip_bytes), &PathBuf::from(target_dir), true)?;

    info!("Downloaded and extracted WCA export");
    Ok(())
}

async fn download() -> anyhow::Result<Vec<u8>> {
    let url = "https://www.worldcubeassociation.org/export/results/WCA_export.tsv";
    let mut response = reqwest::get(url).await?;

    if response.status().is_success() {
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            bytes.extend_from_slice(&chunk);
        }
        return Ok(bytes);
    }

    Err(anyhow!("Failed to download WCA export"))
}
