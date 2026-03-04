use anyhow::{Context, Result};
use std::path::Path;
use tokio;
use tokio::io::AsyncWriteExt;

pub async fn download_file(url: &str, destination: &Path) -> Result<()> {
  if let Some(parent) = destination.parent() {
    tokio::fs::create_dir_all(parent).await?;
  }

  let mut response = reqwest::get(url)
    .await
    .with_context(|| format!("Failed to download file from: {url}"))?
    .error_for_status()
    .with_context(|| format!("Server returned error for {url}"))?;

  let mut file = tokio::fs::File::create(destination)
    .await
    .with_context(|| {
      format!(
        "Failed to create destination file {}",
        destination.display()
      )
    })?;

  while let Some(chunk) = response.chunk().await? {
    file.write_all(&chunk).await?;
  }

  Ok(())
}

pub async fn download_json(url: &str) -> Result<String> {
  let res = reqwest::get(url)
    .await
    .with_context(|| format!("Failed to download json from {url}"))?
    .error_for_status()
    .with_context(|| format!("Server returned error status for {url}"))?;

  res
    .text()
    .await
    .with_context(|| format!("Failed to read response body from {url}"))
}
