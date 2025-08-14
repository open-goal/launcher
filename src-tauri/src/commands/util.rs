use crate::config::LauncherConfig;
use crate::config::SupportedGame;
use crate::util::file::delete_dir;
use log::error;
use std::path::Path;
use sysinfo::Disks;
#[cfg(target_os = "macos")]
use sysinfo::System;
use tauri::Manager;

use super::CommandError;

#[tauri::command]
pub async fn path_exists(directory: String) -> bool {
  Path::new(&directory).exists()
}

#[tauri::command]
pub async fn has_old_data_directory(app_handle: tauri::AppHandle) -> Result<bool, CommandError> {
  match &app_handle.path().app_config_dir() {
    Ok(dir) => Ok(dir.join("data").join("iso_data").exists()),
    Err(_) => Ok(false),
  }
}

#[tauri::command]
pub async fn delete_old_data_directory(app_handle: tauri::AppHandle) -> Result<(), CommandError> {
  match &app_handle.path().app_config_dir() {
    Ok(dir) => Ok(delete_dir(dir.join("data"))?),
    Err(_) => Ok(()),
  }
}

#[tauri::command]
pub async fn is_diskspace_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  if config_lock.games[&game_name].is_installed {
    return Ok(true);
  }
  if config_lock.requirements.bypass_requirements {
    log::warn!("Bypassing the Disk Space requirements check!");
    return Ok(true);
  }

  let install_dir = match &config_lock.installation_dir {
    None => {
      error!("Can't check disk space, no install directory has been choosen!");
      return Err(CommandError::Configuration(
        "Can't check disk space, no install directory has been choosen!".to_owned(),
      ));
    }
    Some(dir) => Path::new(dir),
  };

  // Check the drive that the installation directory is set to
  let minimum_required_drive_space = game_name.required_diskspace();
  for disk in Disks::new_with_refreshed_list().into_iter() {
    if install_dir.starts_with(disk.mount_point()) {
      if disk.available_space() < minimum_required_drive_space {
        log::warn!("Not enough space left on disk: {:?}", disk.name());
        return Ok(false);
      } else {
        return Ok(true);
      }
    }
  }

  error!("Unable to find relevant drive to check for space");
  Err(CommandError::Configuration(
    "Unable to find relevant drive to check for space".to_owned(),
  ))
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn is_minimum_vcc_runtime_installed() -> Result<bool, CommandError> {
  use crate::util::os::get_installed_vcc_runtime;

  let minimum_version = semver::Version::new(14, 40, 33810);
  let installed_vcc_runtime_version = get_installed_vcc_runtime();
  if installed_vcc_runtime_version.is_none() {
    Err(CommandError::Configuration(
      "Unable to check if VCC runtime is installed".to_owned(),
    ))
  } else {
    Ok(installed_vcc_runtime_version.unwrap() >= minimum_version)
  }
}

#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub async fn is_minimum_vcc_runtime_installed() -> Result<bool, CommandError> {
  return Ok(false);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub async fn is_avx_supported() -> bool {
  return is_x86_feature_detected!("avx") || is_x86_feature_detected!("avx2");
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub async fn is_avx_supported() -> bool {
  false
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn is_macos_version_15_or_above() -> bool {
  false
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn is_macos_version_15_or_above() -> bool {
  System::os_version()
    .and_then(|v| v.split('.').next()?.parse::<u32>().ok())
    .map(|major| major >= 15)
    .unwrap_or(false)
}

// TODO - macOS check if rosetta 2 is installed
// #[cfg(target_os = "macos")]
// #[tauri::command]
// pub async fn is_rosetta2_installed() -> bool {
//   todo!()
// }
