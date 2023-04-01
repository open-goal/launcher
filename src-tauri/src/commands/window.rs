use std::path::Path;

use tauri::Manager;

use super::CommandError;

#[tauri::command]
pub async fn open_main_window(handle: tauri::AppHandle) {
  // NOTE:
  // When you create multiple static windows (inside the conf file)
  // they are actually all running in the background
  //
  // This seemed to sometimes create a race condition where the app was not fully setup
  // and when a panic hook was added that exited the process, the app would crash.
  //
  // So instead we make the main window at runtime, and close the splashscreen

  // Create main window
  // {
  //   "title": "OpenGOAL Launcher",
  //   "label": "main",
  //   "width": 800,
  //   "height": 600,
  //   "resizable": false,
  //   "fullscreen": false,
  //   "visible": false,
  //   "center": true,
  //   "decorations": false
  // },
  tauri::WindowBuilder::new(
    &handle,
    "main", /* the unique window label */
    tauri::WindowUrl::App("index.html".parse().unwrap()),
  )
  .title("OpenGOAL Launcher")
  .resizable(false)
  .fullscreen(false)
  .visible(true)
  .center()
  .decorations(false)
  .build()
  .unwrap();
  // Close splashscreen
  if let Some(splashscreen) = handle.app_handle().get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
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
