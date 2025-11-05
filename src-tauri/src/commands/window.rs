use tauri::Manager;

use super::CommandError;

#[tauri::command]
pub async fn open_main_window(handle: tauri::AppHandle) -> Result<(), CommandError> {
  // NOTE:
  // When you create multiple static windows (inside the conf file)
  // they are actually all running in the background
  //
  // This seemed to sometimes create a race condition where the app was not fully setup
  // and when a panic hook was added that exited the process, the app would crash.
  //
  // So instead we make the main window at runtime, and close the splashscreen
  //
  // TODO LATER - consider removing the multi-window setup and just integrate the splash
  // into the main app.  This would probably make for a better experience on things like
  // the steamdeck
  log::info!("Creating main window");
  tauri::WebviewWindowBuilder::new(
    &handle,
    "main", /* the unique window label */
    tauri::WebviewUrl::App("index.html".parse().unwrap()),
  )
  .title("OpenGOAL Launcher")
  .resizable(true)
  .fullscreen(false)
  .visible(true)
  .center()
  .decorations(false)
  .inner_size(800.0, 600.0)
  .focused(true)
  .build()
  .map_err(|_| {
    CommandError::WindowManagement("Unable to create main launcher window".to_owned())
  })?;
  log::info!("Closing splash window");
  // Close splashscreen
  if let Some(splashscreen) = handle.app_handle().get_webview_window("splashscreen") {
    splashscreen
      .close()
      .map_err(|_| CommandError::WindowManagement("Unable to close splash window".to_owned()))?;
  }
  Ok(())
}
