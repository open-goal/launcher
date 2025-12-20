#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
  fs,
  io::ErrorKind,
  path::{Path, PathBuf},
  process::Stdio,
};

use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tokio::{io::AsyncWriteExt, process::Command};

use crate::{
  cache::{LauncherCache, ModInfo},
  commands::{CommandError, binaries::InstallStepOutput, cache::get_mod_sources_data},
  config::{ExecutableLocation, LauncherConfig, SupportedGame},
  util::{
    file::{create_dir, delete_dir, to_image_base64},
    network::download_file,
    process::{create_log_file, create_std_log_file, watch_process},
    tar::{extract_and_delete_tar_ball, extract_tar_ball},
    zip::{extract_and_delete_zip_file, extract_zip_file},
  },
};

fn strip_tar_gz(path: &String) -> String {
  if path.ends_with(".tar") {
    return path.strip_suffix(".tar").unwrap().to_string();
  }
  return path.to_string();
}

#[tauri::command]
pub async fn extract_new_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  bundle_path: String,
  mod_source: String,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };

  // The name of the zip becomes the folder, if one already exists it will be deleted!
  let bundle_path_buf = PathBuf::from(bundle_path);
  let mut mod_name = match bundle_path_buf.file_stem() {
    Some(name) => name.to_string_lossy().to_string(),
    None => {
      return Err(CommandError::GameFeatures(
        "Unable to get mod name from zip file path".to_string(),
      ));
    }
  };
  // .tar.gz files will only get the .gz portion stripped
  mod_name = strip_tar_gz(&mod_name);
  let destination_dir = &install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(mod_source)
    .join(&mod_name);
  delete_dir(destination_dir)?;
  create_dir(destination_dir).map_err(|err| {
    log::error!("Unable to create directory for mod: {}", err);
    CommandError::GameFeatures(format!("Unable to create directory for mod: {}", err))
  })?;
  if cfg!(windows) {
    extract_zip_file(&bundle_path_buf, destination_dir, false).map_err(|err| {
      log::error!("Unable to extract mod: {}", err);
      CommandError::GameFeatures(format!("Unable to extract mod: {}", err))
    })?;
  } else if cfg!(unix) {
    extract_tar_ball(&bundle_path_buf, destination_dir).map_err(|err| {
      log::error!("Unable to extract mod: {}", err);
      CommandError::GameFeatures(format!("Unable to extract mod: {}", err))
    })?;
  } else {
    Err(CommandError::VersionManagement(
      "Unknown operating system, unable to download and extract mod".to_owned(),
    ))?;
  }
  Ok(InstallStepOutput {
    success: true,
    msg: None,
  })
}

#[tauri::command]
pub async fn download_and_extract_new_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  cache: tauri::State<'_, tokio::sync::Mutex<LauncherCache>>,
  game_name: SupportedGame,
  download_url: String,
  mod_name: String,
  source_name: String,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't download and extract mod".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };

  // Download the file
  let parent_path = &install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(&source_name)
    .join(&mod_name);
  let download_path = &parent_path.join(format!("{mod_name}.zip"));

  delete_dir(parent_path)?;
  create_dir(parent_path).map_err(|err| {
    log::error!("Unable to create directory for mod: {}", err);
    CommandError::GameFeatures(format!("Unable to create directory for mod: {}", err))
  })?;
  download_file(&download_url, download_path)
    .await
    .map_err(|err| {
      CommandError::GameFeatures(
        format!(
          "Unable to successfully download mod version from {} to {}, error: {}",
          download_url,
          download_path.to_string_lossy(),
          err
        )
        .to_owned(),
      )
    })?;

  if cfg!(windows) {
    extract_and_delete_zip_file(download_path, parent_path, false).map_err(|err| {
      log::error!("Unable to extract mod: {}", err);
      CommandError::GameFeatures(format!("Unable to extract mod: {}", err))
    })?;
  } else if cfg!(unix) {
    extract_and_delete_tar_ball(download_path, parent_path).map_err(|err| {
      log::error!("Unable to extract mod: {}", err);
      CommandError::GameFeatures(format!("Unable to extract mod: {}", err))
    })?;
  } else {
    Err(CommandError::VersionManagement(
      "Unknown operating system, unable to download and extract mod".to_owned(),
    ))?;
  }

  // Persist the info about the mod to the disk in the event that the mod source is removed / etc
  let mod_source_data = get_mod_sources_data(cache).await;
  match mod_source_data {
    Ok(mod_source_data) => {
      let relevant_mod_source = mod_source_data
        .iter()
        .find(|(_mod_source_url, mod_source_data)| mod_source_data.source_name == source_name);
      if let Some(found_mode_source) = relevant_mod_source {
        if let Some(mod_info) = found_mode_source.1.mods.get(&mod_name) {
          let metadata_path = parent_path.join("_metadata.json");
          log::info!("saving mod info to: {metadata_path:?}");
          create_dir(&metadata_path.parent().unwrap())?;
          let file = fs::File::create(metadata_path)?;
          if let Err(err) = serde_json::to_writer_pretty(file, mod_info) {
            log::error!("Unable to save _metadata.json file: {err:?}")
          }
        }
      }
    }
    Err(err) => {
      log::error!(
        "Unable to fetch mod source data, so unable to persist metadata to disk: {err:?}"
      );
    }
  }

  Ok(InstallStepOutput {
    success: true,
    msg: None,
  })
}

#[tauri::command]
pub async fn get_locally_persisted_mod_info(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<ModInfo, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't find mod metadata".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };
  let metadata_path = &install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(&source_name)
    .join(&mod_name)
    .join("_metadata.json");
  if metadata_path.exists() {
    let file = fs::File::open(metadata_path)?;
    let mod_info = serde_json::from_reader(file).map_err(|_| {
      CommandError::GameFeatures("Unable to deserialize local mod metadata".to_string())
    })?;
    return Ok(mod_info);
  }
  Err(CommandError::GameFeatures(
    "Locally persisted mod metadata does not exist".to_string(),
  ))
}

#[tauri::command]
pub async fn base_game_iso_exists(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };
  Ok(
    install_path
      .join("active")
      .join(game_name.to_string())
      .join("data")
      .join("iso_data")
      .join(game_name.to_string())
      .exists(),
  )
}

fn get_mod_exec_location(
  install_path: PathBuf,
  executable_name: &str,
  game_name: SupportedGame,
  mod_name: &str,
  source_name: &str,
) -> Result<ExecutableLocation, CommandError> {
  let exec_dir = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(source_name)
    .join(mod_name);
  let mut exec_path: PathBuf = exec_dir.join(executable_name);
  if cfg!(windows) {
    exec_path.set_extension("exe");
  }
  if !exec_path.exists() {
    log::error!(
      "Could not find the required binary '{}', can't perform operation",
      exec_path.to_string_lossy()
    );
    return Err(CommandError::BinaryExecution(format!(
      "Could not find the required binary '{}', can't perform operation",
      exec_path.to_string_lossy()
    )));
  }
  Ok(ExecutableLocation {
    executable_dir: exec_dir,
    executable_path: exec_path,
  })
}

#[derive(Debug, Serialize, Deserialize)]
struct LauncherErrorCode {
  msg: String,
}

#[tauri::command]
pub async fn extract_iso_for_mod_install(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
  path_to_iso: String,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };
  let exec_info = match get_mod_exec_location(
    install_path.to_path_buf(),
    "extractor",
    game_name,
    &mod_name,
    &source_name,
  ) {
    Ok(exec_info) => exec_info,
    Err(_) => {
      log::error!("extractor executable not found");
      return Ok(InstallStepOutput {
        success: false,
        msg: Some("Tooling appears to be missing critical files. This may be caused by antivirus software. You will need to redownload the version and try again.".to_string()),
      });
    }
  };

  let iso_extraction_dir = install_path
    .join("active")
    .join(game_name.to_string())
    .join("data")
    .join("iso_data")
    .to_path_buf();

  create_dir(&iso_extraction_dir)?;

  let args = vec![
    path_to_iso.clone(),
    "--extract".to_string(),
    "--validate".to_string(),
    "--extract-path".to_string(),
    iso_extraction_dir.to_string_lossy().into_owned(),
    "--game".to_string(),
    game_name.to_string(),
  ];

  log::info!("Running extractor with args: {:?}", args);

  let mut command = Command::new(exec_info.executable_path);
  command
    .args(args)
    .current_dir(exec_info.executable_dir)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let mut child = command.spawn()?;

  // This is the first install step, reset the file
  let mut log_file = create_log_file(
    &app_handle,
    format!("extractor-{game_name}-{mod_name}.log"),
    true,
  )
  .await?;

  let process_status = watch_process(&mut log_file, &mut child, &app_handle).await?;
  match process_status.code() {
    Some(code) => {
      if code == 0 {
        log::info!("extraction and validation was successful");
        return Ok(InstallStepOutput {
          success: true,
          msg: None,
        });
      }
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      log::error!("extraction and validation was not successful. Code {code}");
      Ok(InstallStepOutput {
        success: false,
        msg: Some(default_error.msg.clone()),
      })
    }
    None => {
      log::error!("extraction and validation was not successful. No status code");
      Ok(InstallStepOutput {
        success: false,
        msg: Some("Unexpected error occurred".to_owned()),
      })
    }
  }
}

#[tauri::command]
pub async fn decompile_for_mod_install(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };
  let exec_info = match get_mod_exec_location(
    install_path.to_path_buf(),
    "extractor",
    game_name,
    &mod_name,
    &source_name,
  ) {
    Ok(exec_info) => exec_info,
    Err(_) => {
      log::error!("extractor executable not found");
      return Ok(InstallStepOutput {
        success: false,
        msg: Some("Tooling appears to be missing critical files. This may be caused by antivirus software. You will need to redownload the version and try again.".to_string()),
      });
    }
  };

  let iso_dir = install_path
    .join("active")
    .join(game_name.to_string())
    .join("data")
    .join("iso_data")
    .join(game_name.to_string())
    .to_path_buf();

  let args = vec![
    iso_dir.clone().to_string_lossy().into_owned(),
    "--folder".to_string(),
    "--decompile".to_string(),
    "--game".to_string(),
    game_name.to_string(),
  ];

  log::info!("Running extractor with args: {:?}", args);

  let mut command = Command::new(exec_info.executable_path);
  command
    .args(args)
    .current_dir(exec_info.executable_dir)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let mut child = command.spawn()?;

  let mut log_file =
    create_log_file(&app_handle, format!("extractor-{game_name}.log"), false).await?;

  let process_status = watch_process(&mut log_file, &mut child, &app_handle).await?;

  // Ensure all remaining data is flushed to the file
  log_file.flush().await?;
  match process_status.code() {
    Some(code) => {
      if code == 0 {
        log::info!("decompilation was successful");
        return Ok(InstallStepOutput {
          success: true,
          msg: None,
        });
      }
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      log::error!("decompilation was not successful. Code {code}");
      Ok(InstallStepOutput {
        success: false,
        msg: Some(default_error.msg.clone()),
      })
    }
    None => {
      log::error!("decompilation was not successful. No status code");
      Ok(InstallStepOutput {
        success: false,
        msg: Some("Unexpected error occurred".to_owned()),
      })
    }
  }
}

#[tauri::command]
pub async fn compile_for_mod_install(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };
  let exec_info = match get_mod_exec_location(
    install_path.to_path_buf(),
    "extractor",
    game_name,
    &mod_name,
    &source_name,
  ) {
    Ok(exec_info) => exec_info,
    Err(_) => {
      log::error!("extractor executable not found");
      return Ok(InstallStepOutput {
        success: false,
        msg: Some("Tooling appears to be missing critical files. This may be caused by antivirus software. You will need to redownload the version and try again.".to_string()),
      });
    }
  };

  let iso_dir = install_path
    .join("active")
    .join(game_name.to_string())
    .join("data")
    .join("iso_data")
    .join(game_name.to_string())
    .to_path_buf();

  let args = vec![
    iso_dir.clone().to_string_lossy().into_owned(),
    "--folder".to_string(),
    "--compile".to_string(),
    "--game".to_string(),
    game_name.to_string(),
  ];

  log::info!("Running extractor with args: {:?}", args);

  let mut command = Command::new(exec_info.executable_path);
  command
    .args(args)
    .current_dir(exec_info.executable_dir)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let mut child = command.spawn()?;

  let mut log_file =
    create_log_file(&app_handle, format!("extractor-{game_name}.log"), false).await?;

  let process_status = watch_process(&mut log_file, &mut child, &app_handle).await?;
  log_file.flush().await?;
  match process_status.code() {
    Some(code) => {
      if code == 0 {
        log::info!("compilation was successful");
        return Ok(InstallStepOutput {
          success: true,
          msg: None,
        });
      }
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      log::error!("compilation was not successful. Code {code}");
      Ok(InstallStepOutput {
        success: false,
        msg: Some(default_error.msg.clone()),
      })
    }
    None => {
      log::error!("compilation was not successful. No status code");
      Ok(InstallStepOutput {
        success: false,
        msg: Some("Unexpected error occurred".to_owned()),
      })
    }
  }
}

#[tauri::command]
pub async fn save_mod_install_info(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
  version_name: String,
) -> Result<InstallStepOutput, CommandError> {
  let mut config_lock = config.lock().await;
  log::info!(
    "Saving mod install info {}, {}, {}, {}",
    game_name.to_string(),
    mod_name,
    source_name,
    version_name
  );
  config_lock
    .update_mods_setting_value(
      "add_mod",
      game_name,
      Some(source_name),
      Some(version_name),
      Some(mod_name),
      None,
    )
    .map_err(|err| {
      log::error!("Unable to remove mod source: {:?}", err);
      CommandError::Configuration("Unable to remove mod source".to_owned())
    })?;
  Ok(InstallStepOutput {
    success: true,
    msg: None,
  })
}

fn generate_launch_mod_args(
  game_name: SupportedGame,
  in_debug: bool,
  config_dir: PathBuf,
  quote_project_path: bool,
) -> Result<Vec<String>, CommandError> {
  let config_dir_adjusted = if quote_project_path {
    format!("\"{}\"", config_dir.to_string_lossy().into_owned())
  } else {
    config_dir.to_string_lossy().into_owned()
  };

  let mut args = vec![
    "-v".to_string(),
    "--game".to_string(),
    game_name.to_string(),
    "--config-path".to_string(),
    config_dir_adjusted,
    "--".to_string(),
    "-boot".to_string(),
    "-fakeiso".to_string(),
  ];
  if in_debug {
    args.push("-debug".to_string());
  }

  Ok(args)
}

#[tauri::command]
pub async fn launch_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  in_debug: bool,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };
  let config_dir = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(&source_name)
    .join("_settings")
    .join(&mod_name);
  let exec_info = get_mod_exec_location(
    install_path.to_path_buf(),
    "gk",
    game_name,
    &mod_name,
    &source_name,
  )?;
  let args = generate_launch_mod_args(game_name, in_debug, config_dir, false)?;

  log::info!("Launching gk args: {:?}", args);

  let log_file = create_std_log_file(
    &app_handle,
    format!("game-{game_name}-{mod_name}.log"),
    false,
  )?;

  // TODO - log rotation here would be nice too
  let mut command = Command::new(exec_info.executable_path);
  command
    .args(args)
    .stdout(log_file.try_clone().unwrap())
    .stderr(log_file)
    .current_dir(exec_info.executable_dir);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  // Start the process here so if there is an error, we can return immediately
  let _child = command.spawn()?;
  Ok(())
}

#[tauri::command]
pub async fn get_local_mod_thumbnail_base64(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => return Ok("".to_string()),
    Some(path) => Path::new(path),
  };

  let cover_path = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join("_local")
    .join(mod_name)
    .join("thumbnail.png");
  if cover_path.exists() {
    return Ok(to_image_base64(cover_path.to_string_lossy().as_ref()));
  }
  Ok("".to_string())
}

#[tauri::command]
pub async fn get_local_mod_cover_base64(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => return Ok("".to_string()),
    Some(path) => Path::new(path),
  };

  let cover_path = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join("_local")
    .join(mod_name)
    .join("cover.png");
  if cover_path.exists() {
    return Ok(to_image_base64(cover_path.to_string_lossy().as_ref()));
  }
  Ok("".to_string())
}

#[tauri::command]
pub async fn uninstall_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };
  let mod_dir = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(&source_name)
    .join(&mod_name);
  if mod_dir.exists() {
    std::fs::remove_dir_all(mod_dir)?;
  }
  config_lock
    .update_mods_setting_value(
      "uninstall_mod",
      game_name,
      Some(source_name),
      None,
      Some(mod_name),
      None,
    )
    .map_err(|_| CommandError::GameFeatures("Unable to uninstall mod".to_owned()))?;
  Ok(())
}

#[tauri::command]
pub async fn reset_mod_settings(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't reset mod settings".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };
  let path_to_settings = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(&source_name)
    .join("_settings")
    .join(&mod_name)
    .join("OpenGOAL")
    .join(game_name.to_string())
    .join("settings")
    .join("pc-settings.gc");

  if path_to_settings.exists() {
    let mut backup_file = path_to_settings.clone();
    backup_file.set_file_name("pc-settings.old.gc");
    std::fs::rename(path_to_settings, backup_file)?;
    Ok(())
  } else {
    Err(CommandError::GameFeatures(
      "Game config directory does not exist, cannot reset settings".to_owned(),
    ))
  }
}

#[tauri::command]
pub async fn get_launch_mod_string(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };
  let exec_info = get_mod_exec_location(
    install_path.to_path_buf(),
    "gk",
    game_name,
    &mod_name,
    &source_name,
  )?;
  let config_dir = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(&source_name)
    .join("_settings")
    .join(&mod_name);
  let args = generate_launch_mod_args(game_name, false, config_dir, true)?;

  Ok(format!(
    "{} {}",
    exec_info.executable_path.display(),
    args.join(" ")
  ))
}

#[derive(Clone, serde::Serialize)]
struct ToastPayload {
  toast: String,
  level: String,
}

#[tauri::command]
pub async fn open_repl_for_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't open REPL for mod".to_string(),
      ));
    }
    Some(path) => Path::new(path),
  };
  let iso_dir = install_path
    .join("active")
    .join(game_name.to_string())
    .join("data")
    .join("iso_data")
    .join(game_name.to_string())
    .to_path_buf();
  let exec_info = get_mod_exec_location(
    install_path.to_path_buf(),
    "goalc",
    game_name,
    &mod_name,
    &source_name,
  )?;
  let mut command;
  #[cfg(windows)]
  {
    command = std::process::Command::new("cmd");
    command
      .args([
        "/C",
        "start",
        "goalc.exe",
        "--game",
        &game_name.to_string(),
        "--iso-path",
        &iso_dir.to_string_lossy(),
      ])
      .current_dir(exec_info.executable_dir)
      .creation_flags(0x08000000);
  }
  #[cfg(target_os = "linux")]
  {
    command = std::process::Command::new("xdg-terminal-exec");
    command
      .args(["./goalc", "--iso-path", &iso_dir.to_string_lossy()])
      .current_dir(exec_info.executable_dir);
  }
  #[cfg(target_os = "macos")]
  {
    command = std::process::Command::new("osascript");
    command
      .args([
        "-e",
        "'tell app \"Terminal\" to do script",
        format!("\"cd {:?}\" &&", exec_info.executable_dir).as_str(),
        "./goalc",
        "--iso-path",
        &iso_dir.to_string_lossy(),
      ])
      .current_dir(exec_info.executable_dir);
  }
  match command.spawn() {
    Ok(_) => Ok(()),
    Err(e) => {
      if let ErrorKind::NotFound = e.kind() {
        let _ = app_handle.emit(
          "toast_msg",
          ToastPayload {
            toast: format!("'{:?}' not found in PATH!", command.get_program()),
            level: "error".to_string(),
          },
        );
      }
      Err(CommandError::BinaryExecution(
        "Unable to launch REPL".to_owned(),
      ))
    }
  }
}
