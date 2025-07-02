use std::path::PathBuf;

use crate::util::{file::create_dir, network};

use super::CommandError;

#[tauri::command]
pub async fn download_file(url: String, destination: PathBuf) -> Result<(), CommandError> {
  let parent = destination.parent().ok_or_else(|| {
    CommandError::OSOperation("Destination path has no parent directory".to_owned())
  })?;

  if !parent.exists() {
    create_dir(parent).map_err(|_| {
      CommandError::OSOperation(format!(
        "Unable to prepare destination folder '{}'",
        parent.display()
      ))
    })?;
  }

  network::download_file(&url, &destination)
    .await
    .map_err(|_| CommandError::OSOperation("Unable to successfully download file".to_owned()))?;

  Ok(())
}
