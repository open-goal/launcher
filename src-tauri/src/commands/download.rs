use std::path::PathBuf;

use crate::util::{file::create_dir, network};

use super::CommandError;

// TODO: Finish refactoring this function to use anyhow result, after refactoring download_file
#[tauri::command]
pub async fn download_file(url: String, destination: PathBuf) -> Result<(), CommandError> {
  if let Some(parent) = destination.parent() {
    create_dir(parent)?;
    network::download_file(&url, &destination)
      .await
      .map_err(|_| CommandError::OSOperation("Unable to successfully download file".to_owned()))?;
    Ok(())
  } else {
    return Err(CommandError::OSOperation(
      "Destination path has no parent directory".to_owned(),
    ));
    // TODO: anyhow::bail!("Destination path has no parent directory")
  }
}
