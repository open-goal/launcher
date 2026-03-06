use std::path::Path;

use crate::{
  config::LauncherConfig,
  util::{
    file::delete_dir, network::download_file, tar::extract_and_delete_tar_ball,
    zip::extract_and_delete_zip_file,
  },
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
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::VersionManagement(
        "Cannot install version, no installation directory set".to_owned(),
      ));
    }
    Some(path) => Path::new(path),
  };

  let dest_dir = install_path
    .join("versions")
    .join(&version_folder)
    .join(&version);

  // Delete the directory if it exists
  delete_dir(&dest_dir)?;

  if cfg!(windows) {
    let download_path = install_path
      .join("versions")
      .join(&version_folder)
      .join(format!("{version}.zip"));

    // Download the file
    download_file(&url, &download_path).await?;

    // Extract the zip file
    extract_and_delete_zip_file(&download_path, &dest_dir, true).map_err(|_| {
      CommandError::VersionManagement(
        "Unable to successfully extract downloaded version".to_owned(),
      )
    })?;

    // Verify that the extracted files seem correct (look for extractor.exe)
    let expected_extractor_path = dest_dir.join("extractor.exe");
    if !expected_extractor_path.exists() {
      log::info!(
        "Version did not extract properly, {} is missing!",
        expected_extractor_path.display()
      );
      delete_dir(&dest_dir)?;
      return Err(CommandError::VersionManagement(
        "Version did not extract properly, critical files are missing. An antivirus may have deleted the files!"
        .to_owned()
      ));
    }
    return Ok(());
  } else if cfg!(unix) {
    let download_path = install_path
      .join("versions")
      .join(&version_folder)
      .join(format!("{version}.tar.gz"));

    // Download the file
    download_file(&url, &download_path).await?;

    // Extract the tar file
    extract_and_delete_tar_ball(download_path, &dest_dir)?;

    // Verify that the extracted files seem correct (look for extractor.exe)
    let expected_extractor_path = dest_dir.join("extractor");
    if !expected_extractor_path.exists() {
      log::info!(
        "Version did not extract properly, {} is missing!",
        expected_extractor_path.display()
      );
      delete_dir(&dest_dir)?;
      return Err(CommandError::VersionManagement(
        "Version did not extract properly, critical files are missing. An antivirus may have deleted the files!"
        .to_owned()
      ));
    }
    return Ok(());
  }
  Err(CommandError::VersionManagement(
    "Unknown operating system, unable to download and extract correct release".to_owned(),
  ))
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
