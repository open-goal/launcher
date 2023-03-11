use futures_util::StreamExt;
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
  let mut stream = res.bytes_stream();

  while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    file.write_all(&chunk).await?;
  }
  Ok(())
}
