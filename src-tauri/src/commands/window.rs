use std::path::Path;

use tauri::Manager;

use super::CommandError;

#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) {
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  // Show main window
  // TODO - cleanup this, return an error if we can't close it
  window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
pub async fn open_dir_in_os(directory: String) -> Result<(), CommandError> {
  let folder_path = Path::new(&directory);

  if !folder_path.exists() {
    return Err(CommandError::OSOperation(format!(
      "Can't open folder '{}', doesn't exist",
      folder_path.display()
    )));
  }

  crate::util::os::open_dir_in_os(folder_path.to_string_lossy().into_owned())
    .map_err(|_| CommandError::OSOperation(format!("Unable to go to open folder in OS")))?;
  Ok(())
}
