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
  match &config_lock.installation_dir {
    None => Err(CommandError::InvalidPath(format!(
      "Can't uninstalled the game, no installation directory found"
    ))),
    Some(path) => {
      // TODO - cleanup
      let data_folder = Path::new(path).join("active").join(&game_name).join("data");
      std::fs::remove_dir_all(data_folder.join("decompiler_out"));
      std::fs::remove_dir_all(data_folder.join("iso_data"));
      std::fs::remove_dir_all(data_folder.join("out"));
      config_lock.update_installed_game_version(&game_name, false);
      app_handle.emit_all("gameUninstalled", {}).unwrap();
      Ok(())
    }
  }
}

#[tauri::command]
pub async fn reset_game_settings(game_name: String) -> Result<(), ()> {
  let config_dir = config_dir();
  match &config_dir {
    None => Ok(()),
    Some(path) => {
      let path_to_settings = path
        .join("OpenGOAL")
        .join(game_name)
        .join("settings")
        .join("pc-settings.gc");
      if path_to_settings.exists() {
        let mut backup_file = path_to_settings.clone();
        backup_file.set_file_name("pc-settings.old.gc");
        std::fs::rename(path_to_settings, backup_file);
        Ok(())
      } else {
        Ok(())
      }
    }
  }
}
