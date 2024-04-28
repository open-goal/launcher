use std::path::PathBuf;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  NetworkRequest(#[from] reqwest::Error),
}

pub async fn download_file(url: &String, destination: &PathBuf) -> Result<(), NetworkError> {
  let client = reqwest::Client::new();
  let req = client.get(url);
  let res = req.send().await?;

  let mut file = File::create(destination).await?;
  let resp_bytes = res.bytes().await?;
  file.write_all(&resp_bytes).await?;
  Ok(())
}

pub async fn download_json(url: &String) -> Result<String, NetworkError> {
  let client = reqwest::Client::new();
  let req = client.get(url);
  let resp = req.send().await?;
  Ok(resp.text().await?)
}
