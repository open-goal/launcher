use std::path::{Path, PathBuf};

use crate::util::{file::create_dir, network};

use super::CommandError;

#[tauri::command]
pub async fn download_file(url: String, destination: String) -> Result<(), CommandError> {
  let download_path = PathBuf::from(&destination);
  match download_path.parent() {
    Some(parent) => {
      if parent == Path::new("") {
        return Err(CommandError::OSOperation(
          "Unable to successfully download file".to_owned(),
        ));
      } else {
        create_dir(&parent.to_path_buf()).map_err(|_| {
          CommandError::VersionManagement(format!(
            "Unable to prepare destination folder '{}' for download",
            parent.display()
          ))
        })?;
      }
    }
    None => {
      return Err(CommandError::OSOperation(
        "Unable to successfully download file".to_owned(),
      ));
    }
  }
  network::download_file(&url, &download_path)
    .await
    .map_err(|_| CommandError::OSOperation("Unable to successfully download file".to_owned()))?;
  Ok(())
}
