use std::path::Path;
use tokio;

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  NetworkRequest(#[from] reqwest::Error),
}

pub async fn download_file(url: &str, destination: &Path) -> Result<(), NetworkError> {
  let res = reqwest::get(url).await?;
  let bytes = res.bytes().await?;
  tokio::fs::write(destination, &bytes).await?;
  Ok(())
}

pub async fn download_json(url: &str) -> Result<String, NetworkError> {
  let res = reqwest::get(url).await?;
  Ok(res.text().await?)
}
