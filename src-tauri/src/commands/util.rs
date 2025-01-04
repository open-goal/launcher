use std::path::Path;
#[cfg(target_os = "macos")]
use sysctl::Sysctl;
use tauri::Manager;

use super::CommandError;

#[tauri::command]
pub async fn path_exists(directory: String) -> Result<bool, CommandError> {
  Ok(Path::new(&directory).exists())
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
      let version = ctl_val.parse::<f32>();
      if version.is_ok() {
        return Ok(version.unwrap() >= 15.0);
      }
    }
  }
  Ok(false)
}
