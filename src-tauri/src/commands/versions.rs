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

  #[cfg(target_os = "windows")]
  let (download_path, expected_extractor_path) = (
    versions_dir.join(format!("{version}.zip")),
    dest_dir.join("extractor.exe"),
  );

  #[cfg(unix)]
  let (download_path, expected_extractor_path) = (
    versions_dir.join(format!("{version}.tar.gz")),
    dest_dir.join("extractor"),
  );

  delete_dir(&dest_dir)?;
  download_file(&url, &download_path).await?;
  extract_and_delete_archive(&download_path, &dest_dir, true)?;

  if !expected_extractor_path.exists() {
    log::error!(
      "Version did not extract properly, {} is missing!",
      expected_extractor_path.display()
    );
    delete_dir(&dest_dir)?;
    return Err(CommandError::VersionManagement(
    "Version did not extract properly, critical files are missing. An antivirus may have deleted the files!"
      .to_owned(),
  ));
  }
  return Ok(());
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
