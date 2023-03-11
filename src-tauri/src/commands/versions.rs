use std::path::Path;

use log::info;

use crate::{
  config::LauncherConfig,
  util::{
    file::{create_dir, delete_dir},
    network::download_file,
    os::open_dir_in_os,
    zip::extract_and_delete_zip_file,
  },
};

use super::CommandError;

#[tauri::command]
pub async fn list_downloaded_versions(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version_folder: String,
) -> Result<Vec<String>, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => return Ok(Vec::new()),
    Some(path) => Path::new(path),
  };

  let expected_path = Path::new(install_path)
    .join("versions")
    .join(version_folder);
  if !expected_path.exists() || !expected_path.is_dir() {
    log::info!(
      "No {} folder found, returning no releases",
      expected_path.display()
    );
    return Ok(Vec::new());
  }

  let entries = std::fs::read_dir(&expected_path).map_err(|_| {
    CommandError::VersionManagement(format!(
      "Unable to read versions from {}",
      expected_path.display()
    ))
  })?;
  Ok(
    entries
      .filter_map(|e| {
        e.ok().and_then(|d| {
          let p = d.path();
          if p.is_dir() {
            Some(
              p.file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or("".into()),
            )
          } else {
            None
          }
        })
      })
      .collect(),
  )
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
      return Err(CommandError::VersionManagement(format!(
        "Cannot install version, no installation directory set"
      )))
    }
    Some(path) => Path::new(path),
  };

  let dest_dir = install_path
    .join("versions")
    .join(&version_folder)
    .join(&version);

  // Delete the directory if it exists, and create it from scratch
  delete_dir(&dest_dir).map_err(|_| {
    CommandError::VersionManagement(format!(
      "Unable to prepare destination folder '{}' for download",
      dest_dir.display()
    ))
  })?;
  create_dir(&dest_dir).map_err(|_| {
    CommandError::VersionManagement(format!(
      "Unable to prepare destination folder '{}' for download",
      dest_dir.display()
    ))
  })?;

  let download_path = install_path
    .join("versions")
    .join(version_folder)
    .join(format!("{}.zip", version));

  // Download the file
  download_file(&url, &download_path).await.map_err(|_| {
    CommandError::VersionManagement(format!("Unable to successfully download version"))
  })?;

  // Extract the zip file
  extract_and_delete_zip_file(&download_path, &dest_dir).map_err(|_| {
    CommandError::VersionManagement(format!("Unable to successfully extract downloaded version"))
  })?;
  Ok(())
}

#[tauri::command]
pub async fn remove_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version: String,
  version_folder: String,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::VersionManagement(format!(
        "Cannot install version, no installation directory set"
      )))
    }
    Some(path) => Path::new(path),
  };

  info!("Deleting Version {}:{}", version_folder, version);

  let version_dir = install_path
    .join("versions")
    .join(&version_folder)
    .join(&version);

  delete_dir(&version_dir)?;

  Ok(())
}

#[tauri::command]
pub async fn go_to_version_folder(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version_folder: String,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::VersionManagement(format!(
        "Cannot go to version folder, no installation directory set"
      )))
    }
    Some(path) => Path::new(path),
  };

  let folder_path = Path::new(install_path)
    .join("versions")
    .join(version_folder);
  create_dir(&folder_path).map_err(|_| {
    CommandError::VersionManagement(format!(
      "Unable to go to create version folder '{}' in order to open it",
      folder_path.display()
    ))
  })?;

  open_dir_in_os(folder_path.to_string_lossy().into_owned())
    .map_err(|_| CommandError::VersionManagement(format!("Unable to go to open folder in OS")))?;
  Ok(())
}
