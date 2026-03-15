use crate::config::LauncherConfig;
use crate::config::SupportedGame;
use anyhow::anyhow;
use log::warn;
use sysinfo::Disks;

use super::CommandError;

#[tauri::command]
pub async fn is_diskspace_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
) -> Result<bool, CommandError> {
  let install_dir = {
    let config_lock = config.lock().await;
    if config_lock.games[&game_name].is_installed {
      return Ok(true);
    }
    if config_lock.requirements.bypass_requirements {
      warn!("Bypassing the Disk Space requirements check!");
      return Ok(true);
    }
    config_lock.install_dir()?
  };

  let minimum_required_drive_space = game_name.required_diskspace();

  Disks::new_with_refreshed_list()
    .iter()
    .filter(|d| install_dir.starts_with(d.mount_point()))
    .max_by_key(|d| d.mount_point().as_os_str().len())
    .map(|disk| disk.available_space() >= minimum_required_drive_space)
    .ok_or_else(|| {
      anyhow!(
        "No disk found for install directory {}",
        install_dir.display()
      )
    })
    .map_err(|e| anyhow!("Failed to find relevant drive to check for space: {e}"))
    .map_err(Into::into)
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn is_minimum_vcc_runtime_installed() -> Result<bool, CommandError> {
  use crate::util::os::get_installed_vcc_runtime;

  let minimum_version = semver::Version::new(14, 40, 33810);
  match get_installed_vcc_runtime() {
    Ok(installed_version) => Ok(installed_version >= minimum_version),
    Err(err) => Err(CommandError::Configuration(format!(
      "Unable to check if VCC runtime is installed: {:?}",
      err
    ))),
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

#[tauri::command]
pub async fn frontend_log(level: String, log: String) -> Result<(), ()> {
  match level.as_str() {
    "debug" => log::debug!("{}", log),
    "info" => log::info!("{}", log),
    "warn" => log::warn!("{}", log),
    "error" => log::error!("{}", log),
    _ => log::info!("{}", log),
  }
  Ok(())
}
