use std::path::Path;
use tauri::Manager;

use super::CommandError;

#[tauri::command]
pub async fn path_exists(directory: String) -> Result<bool, CommandError> {
  Ok(Path::new(&directory).exists())
}