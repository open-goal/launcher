use crate::{config::LauncherConfig, util::file::delete_dir};
use tauri::Manager;

use super::CommandError;

#[tauri::command]
pub async fn has_old_data_directory(app_handle: tauri::AppHandle) -> Result<bool, CommandError> {
  match &app_handle.path_resolver().app_config_dir() {
    None => Ok(false),
    Some(dir) => Ok(dir.join("data").join("iso_data").exists()),
  }
}

#[tauri::command]
pub async fn delete_old_data_directory(app_handle: tauri::AppHandle) -> Result<(), CommandError> {
  match &app_handle.path_resolver().app_config_dir() {
    None => Ok(()),
    Some(dir) => Ok(delete_dir(&dir.join("data"))?),
  }
}

#[tauri::command]
pub async fn reset_to_defaults(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.reset_to_defaults().map_err(|_| {
    CommandError::Configuration(format!("Unable to reset configuration to defaults"))
  })?;
  Ok(())
}

#[tauri::command]
pub async fn get_install_directory(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<String>, CommandError> {
  let config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Ok(None),
    Some(dir) => Ok(Some(dir.to_string())),
  }
}

#[tauri::command]
pub async fn set_install_directory(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  new_dir: String,
) -> Result<Option<String>, CommandError> {
  let mut config_lock = config.lock().await;
  Ok(config_lock.set_install_directory(new_dir).map_err(|_| {
    CommandError::Configuration(format!("Unable to persist installation directory"))
  })?)
}

#[cfg(any(target_arch = "x86"))]
#[tauri::command]
pub async fn is_avx_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<bool, CommandError> {
  let mut config_lock = config.lock().await;
  match config_lock.requirements.avx {
    None => {
      if is_x86_feature_detected!("avx") || is_x86_feature_detected!("avx2") {
        config_lock.requirements.avx = Some(true);
      } else {
        config_lock.requirements.avx = Some(false);
      }
      config_lock.save_config().map_err(|err| {
        log::error!("Unable to persist avx requirement change {}", err);
        CommandError::Configuration(format!("Unable to persist avx requirement change"))
      })?;
      Ok(config_lock.requirements.avx.unwrap_or(false))
    }
    Some(val) => Ok(val),
  }
}

#[cfg(any(target_arch = "aarch64"))]
#[tauri::command]
pub async fn is_avx_requirement_met() -> bool {
  false
}

// TODO - investigate moving the OpenGL check into the rust layer via `wgpu`
// for now, we return potentially undefined so the frontend can update the value via sidecar
#[tauri::command]
pub async fn is_opengl_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<bool>, CommandError> {
  let config_lock = config.lock().await;
  match config_lock.requirements.opengl {
    None => Ok(None),
    Some(_) => Ok(config_lock.requirements.opengl),
  }
}

#[tauri::command]
pub async fn set_opengl_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  requirement_met: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .set_opengl_requirement_met(requirement_met)
    .map_err(|_| {
      CommandError::Configuration(format!("Unable to persist opengl requirement change"))
    })?;
  Ok(())
}

#[tauri::command]
pub async fn finalize_installation(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .update_installed_game_version(&game_name, true)
    .map_err(|_| {
      CommandError::Configuration(format!("Unable to persist game installation status"))
    })?;
  app_handle.emit_all("gameInstalled", {})?;
  Ok(())
}

#[tauri::command]
pub async fn is_game_installed(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<bool, CommandError> {
  let mut config_lock = config.lock().await;

  if !config_lock.is_game_installed(&game_name) {
    return Ok(false);
  }

  // Check that the version and version folder config field is set properly as well
  let version = config_lock.game_install_version(&game_name);
  let version_folder = config_lock.game_install_version_folder(&game_name);

  if version.is_empty() || version_folder.is_empty() {
    config_lock
      .update_installed_game_version(&game_name, false)
      .map_err(|err| {
        log::error!(
          "Unable to mark partially installed game as uninstalled {}",
          err
        );
        CommandError::Configuration(format!(
          "Unable to mark partially installed game as uninstalled"
        ))
      })?;
    return Ok(false);
  }

  Ok(true)
}

#[tauri::command]
pub async fn get_installed_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.game_install_version(&game_name))
}

#[tauri::command]
pub async fn get_installed_version_folder(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.game_install_version_folder(&game_name))
}

#[tauri::command]
pub async fn save_active_version_change(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version_folder: String,
  new_active_version: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .set_active_version_folder(version_folder)
    .map_err(|_| {
      CommandError::Configuration(format!("Unable to persist active version folder change"))
    })?;
  config_lock
    .set_active_version(new_active_version)
    .map_err(|_| CommandError::Configuration(format!("Unable to persist active version change")))?;
  Ok(())
}

#[tauri::command]
pub async fn get_active_tooling_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<String>, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.active_version.clone())
}

#[tauri::command]
pub async fn get_active_tooling_version_folder(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<String>, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.active_version_folder.clone())
}

#[tauri::command]
pub async fn get_background_disabled(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<bool>, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.background_disabled.clone())
}

#[tauri::command]
pub async fn set_background_disabled(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  disabled: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  Ok(
    config_lock
      .set_background_disabled(disabled)
      .map_err(|_| CommandError::Configuration(format!("Unable to persist background disabled")))?,
  )
}
