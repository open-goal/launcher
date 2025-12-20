use std::path::Path;
use tokio;

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  NetworkRequest(#[from] reqwest::Error),
  #[error("{0}")]
  Message(String),
}

pub async fn download_file(url: &str, destination: &Path) -> Result<(), NetworkError> {
  let res = reqwest::get(url).await?;
  let bytes = res.bytes().await?;
  tokio::fs::write(destination, &bytes).await?;
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
