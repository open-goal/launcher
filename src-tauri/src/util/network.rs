use anyhow::{Context, Result};
use std::path::Path;
use tokio;
use tokio::io::AsyncWriteExt;

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  NetworkRequest(#[from] reqwest::Error),
  #[error("{0}")]
  Message(String),
}

pub async fn download_file(url: &str, destination: impl AsRef<Path>) -> Result<()> {
  let destination = destination.as_ref();
  let mut response = reqwest::get(url)
    .await
    .with_context(|| format!("Failed to download file from: {url}"))?
    .error_for_status()
    .with_context(|| format!("Server returned error for {url}"))?;

  let mut file = tokio::fs::File::create(&destination)
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

pub async fn download_json(url: &str) -> Result<String, NetworkError> {
  let res = reqwest::get(url).await?;
  if res.status().is_success() {
    return Ok(res.text().await?);
  }
  Err(NetworkError::Message(format!(
    "Unable to download json from {url}, status: {}",
    res.status()
  )))
}
