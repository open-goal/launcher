use crate::config::LauncherConfig;
use crate::util::file::delete_dir;
#[cfg(target_os = "macos")]
use log::error;
#[cfg(target_os = "macos")]
use log::info;
use serde_json::Value;
use std::path::Path;
#[cfg(target_os = "macos")]
use sysctl::Sysctl;
use sysinfo::Disks;
use tauri::Manager;

use super::CommandError;

#[tauri::command]
pub async fn path_exists(directory: String) -> Result<bool, CommandError> {
  Ok(Path::new(&directory).exists())
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

pub fn diskspace_threshold_for_fresh_install(game_name: &str) -> Result<u64, CommandError> {
  match game_name {
    "jak1" => Ok(4 * 1024 * 1024 * 1024),  // 4gb
    "jak2" => Ok(11 * 1024 * 1024 * 1024), // 11gb
    "jak3" => Ok(11 * 1024 * 1024 * 1024), // TODO! gb
    "jakx" => Ok(11 * 1024 * 1024 * 1024), // TODO! gb
    _ => Err(CommandError::UnknownGame(game_name.to_string())),
  }
}

#[tauri::command]
pub async fn is_diskspace_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<bool, CommandError> {
  // If the game is already installed, we assume they have enough drive space
  let config_lock = config.lock().await;
  if matches!(
    config_lock.get_setting_value("installed", Some(game_name.clone())),
    Ok(Value::Bool(true))
  ) {
    return Ok(true);
  }
  if config_lock.requirements.bypass_requirements {
    log::warn!("Bypassing the Disk Space requirements check!");
    return Ok(true);
  }

  let install_dir = match &config_lock.installation_dir {
    None => {
      log::error!("Can't check disk space, no install directory has been choosen!");
      return Err(CommandError::Configuration(
        "Can't check disk space, no install directory has been choosen!".to_owned(),
      ));
    }
    Some(dir) => Path::new(dir),
  };

  // Check the drive that the installation directory is set to
  let minimum_required_drive_space = diskspace_threshold_for_fresh_install(&game_name)?;
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

  log::error!("Unable to find relevant drive to check for space");
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
  // TODO - macOS check if on atleast sequoia and rosetta 2 is installed
  false
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn is_macos_version_15_or_above() -> Result<bool, CommandError> {
  return Ok(false);
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn is_macos_version_15_or_above() -> Result<bool, CommandError> {
  if let Ok(ctl) = sysctl::Ctl::new("kern.osproductversion") {
    if let Ok(ctl_val) = ctl.value_string() {
      info!("MacOS Version Number: {}", ctl_val);
      let mut stripped_ctl_val = ctl_val.as_str();
      if stripped_ctl_val.contains(".") {
        let first_value = stripped_ctl_val.split(".").next();
        if first_value.is_none() {
          error!("Unable to parse MacOS major version number");
          return Ok(false);
        }
        stripped_ctl_val = first_value.unwrap();
      }
      info!("Checking MacOS Version Number: {}", stripped_ctl_val);
      let version = stripped_ctl_val.parse::<f32>();
      if version.is_ok() {
        return Ok(version.unwrap() >= 15.0);
      }
    }
  }
  Ok(false)
}
