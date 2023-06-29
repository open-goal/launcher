use std::path::Path;

use log::info;

use crate::{
  config::LauncherConfig,
  util::{
    file::{create_dir, delete_dir},
    network::download_file,
    os::open_dir_in_os,
    tar::extract_and_delete_tar_ball,
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
      return Err(CommandError::VersionManagement(
        "Cannot install version, no installation directory set".to_owned(),
      ))
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

  if cfg!(windows) {
    let download_path = install_path
      .join("versions")
      .join(&version_folder)
      .join(format!("{version}.zip"));

    // Download the file
    download_file(&url, &download_path).await.map_err(|_| {
      CommandError::VersionManagement("Unable to successfully download version".to_owned())
    })?;

    // Extract the zip file
    extract_and_delete_zip_file(&download_path, &dest_dir).map_err(|_| {
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
      delete_dir(&dest_dir).map_err(|_| {
        CommandError::VersionManagement(format!(
          "Unable to prepare destination folder '{}' for download",
          dest_dir.display()
        ))
      })?;
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
    download_file(&url, &download_path).await.map_err(|_| {
      CommandError::VersionManagement("Unable to successfully download version".to_owned())
    })?;

    // Extract the zip file
    extract_and_delete_tar_ball(&download_path, &dest_dir).map_err(|err| {
      log::error!("unable to extract and delete version tar.gz file {}", err);
      CommandError::VersionManagement(
        "Unable to successfully extract downloaded version".to_owned(),
      )
    })?;

    // Verify that the extracted files seem correct (look for extractor.exe)
    let expected_extractor_path = dest_dir.join("extractor");
    if !expected_extractor_path.exists() {
      log::info!(
        "Version did not extract properly, {} is missing!",
        expected_extractor_path.display()
      );
      delete_dir(&dest_dir).map_err(|_| {
        CommandError::VersionManagement(format!(
          "Unable to prepare destination folder '{}' for download",
          dest_dir.display()
        ))
      })?;
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
  version_folder: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::VersionManagement(
        "Cannot install version, no installation directory set".to_owned(),
      ))
    }
    Some(path) => Path::new(path),
  };

  info!("Deleting Version {}:{}", version_folder, version);

  let version_dir = install_path
    .join("versions")
    .join(&version_folder)
    .join(&version);

  delete_dir(&version_dir)?;

  // If it's the active version, we should clean that up in the settings file
  if let (Some(config_version_folder), Some(config_version)) = (
    &config_lock.active_version_folder,
    &config_lock.active_version,
  ) {
    if (version_folder == *config_version_folder) && (version == *config_version) {
      config_lock.clear_active_version().map_err(|_| {
        CommandError::VersionManagement(
          "Unable to clear active version after it was removed".to_owned(),
        )
      })?;
    }
  }

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
      return Err(CommandError::VersionManagement(
        "Cannot go to version folder, no installation directory set".to_owned(),
      ))
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
    .map_err(|_| CommandError::VersionManagement("Unable to open folder in OS".to_owned()))?;
  Ok(())
}

#[tauri::command]
pub async fn ensure_active_version_still_exists(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<bool, CommandError> {
  let mut config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::VersionManagement(
        "Cannot install version, no installation directory set".to_owned(),
      ))
    }
    Some(path) => Path::new(path),
  };

  info!(
    "Checking if active version still exists {:?}:{:?}",
    config_lock.active_version_folder, config_lock.active_version
  );

  match (
    &config_lock.active_version_folder,
    &config_lock.active_version,
  ) {
    (Some(config_version_folder), Some(config_version)) => {
      let version_dir = install_path
        .join("versions")
        .join(&config_version_folder)
        .join(&config_version);
      if !version_dir.exists() {
        // Clear active version if it's no longer available
        config_lock.clear_active_version().map_err(|_| {
          CommandError::VersionManagement(
            "Unable to clear active version after it was found to be missing".to_owned(),
          )
        })?;
      }
      Ok(version_dir.exists())
    }
    (_, _) => Ok(false),
  }
}
