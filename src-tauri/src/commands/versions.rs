use crate::{
  config::LauncherConfig,
  util::{file::delete_dir, network::download_file, tar::extract_and_delete_archive},
};

use super::CommandError;

#[tauri::command]
pub async fn list_downloaded_versions(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version_folder: String,
) -> Result<Vec<String>, CommandError> {
  let config = config.lock().await;
  Ok(config.list_downloaded_versions(&version_folder)?)
}

#[tauri::command]
pub async fn download_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version: String,
  version_folder: String,
  url: String,
) -> Result<(), CommandError> {
  let versions_dir = config
    .lock()
    .await
    .install_dir()?
    .join("versions")
    .join(&version_folder);
  let dest_dir = versions_dir.join(&version);

  #[cfg(windows)]
  let download_path = versions_dir.join(format!("{version}.zip"));

  #[cfg(unix)]
  let download_path = versions_dir.join(format!("{version}.tar.gz"));

  delete_dir(&dest_dir)?;
  download_file(&url, &download_path).await?;
  extract_and_delete_archive(&download_path, &dest_dir, true)?;
  Ok(())
}

#[tauri::command]
pub async fn remove_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  Ok(config_lock.remove_version(&version)?)
}

#[tauri::command]
pub async fn ensure_active_version_still_exists(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<bool, CommandError> {
  let mut config_lock = config.lock().await;
  Ok(config_lock.ensure_active_version_still_exists()?)
}
