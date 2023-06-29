use std::path::Path;

use tauri::{api::path::config_dir, Manager};

use crate::config::LauncherConfig;

use super::CommandError;

#[tauri::command]
pub async fn uninstall_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;

  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameManagement(format!(
        "No installation directory set, can't perform uninstallation"
      )))
    }
    Some(path) => Path::new(path),
  };

  let data_folder = Path::new(install_path)
    .join("active")
    .join(&game_name)
    .join("data");

  std::fs::remove_dir_all(data_folder.join("decompiler_out"))?;
  std::fs::remove_dir_all(data_folder.join("iso_data"))?;
  std::fs::remove_dir_all(data_folder.join("out"))?;

  config_lock
    .update_installed_game_version(&game_name, false)
    .map_err(|_| {
      CommandError::GameManagement(format!("Unable to persist game installation status"))
    })?;
  app_handle.emit_all("gameUninstalled", {})?;
  Ok(())
}

#[tauri::command]
pub async fn reset_game_settings(game_name: String) -> Result<(), CommandError> {
  let config_dir = match config_dir() {
    None => {
      return Err(CommandError::GameManagement(format!(
        "Could not determine game config directory"
      )))
    }
    Some(path) => path,
  };

  let path_to_settings = config_dir
    .join("OpenGOAL")
    .join(game_name)
    .join("settings")
    .join("pc-settings.gc");
  if path_to_settings.exists() {
    let mut backup_file = path_to_settings.clone();
    backup_file.set_file_name("pc-settings.old.gc");
    std::fs::rename(path_to_settings, backup_file)?;
    Ok(())
  } else {
    return Err(CommandError::GameManagement(format!(
      "Game config directory does not exist, cannot reset settings"
    )));
  }
}
