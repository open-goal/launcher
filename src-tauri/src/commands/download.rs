use std::path::PathBuf;

use crate::util::network;

use super::CommandError;

#[tauri::command]
pub async fn download_file(url: String, destination: PathBuf) -> Result<(), CommandError> {
  network::download_file(&url, &destination).await?;
  Ok(())
}
