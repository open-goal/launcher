#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
  collections::HashMap,
  path::{Path, PathBuf},
  process::Command
};

use log::info;
use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::{
  commands::binaries::InstallStepOutput,
  config::LauncherConfig,
  util::{
    file::{create_dir, delete_dir, overwrite_dir, to_image_base64},
    zip::{check_if_zip_contains_top_level_dir, extract_and_delete_zip_file, extract_zip_file},
  },
};

use super::{download::download_file, CommandError};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TexturePackInfo {
  #[serde(skip_deserializing)]
  file_list: Vec<String>,
  #[serde(skip_deserializing)]
  has_metadata: bool,
  #[serde(skip_deserializing)]
  cover_image_path: Option<String>,
  name: String,
  version: String,
  author: String,
  release_date: String,
  supported_games: Vec<String>,
  description: String,
  tags: Vec<String>,
}

#[tauri::command]
pub async fn list_extracted_texture_pack_info(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<HashMap<String, TexturePackInfo>, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => return Ok(HashMap::new()),
    Some(path) => Path::new(path),
  };

  let expected_path = Path::new(install_path)
    .join("features")
    .join(&game_name)
    .join("texture-packs");
  if !expected_path.exists() || !expected_path.is_dir() {
    log::info!(
      "No {} folder found, returning no texture packs",
      expected_path.display()
    );
    return Ok(HashMap::new());
  }

  let entries = std::fs::read_dir(&expected_path).map_err(|_| {
    CommandError::GameFeatures(format!(
      "Unable to read texture packs from {}",
      expected_path.display()
    ))
  })?;

  let mut package_map = HashMap::new();

  for entry in entries {
    let entry = entry?;
    let entry_path = entry.path();
    if entry_path.is_dir() {
      let directory_name = entry_path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .map(String::from)
        .ok_or_else(|| {
          CommandError::GameFeatures(format!("Unable to get directory name for {:?}", entry_path))
        })?;
      // Get a list of all texture files for this pack
      log::info!("Texture pack dir name: {}", directory_name);
      let mut file_list = Vec::new();
      for entry in glob::glob(
        &entry_path
          .join("custom_assets")
          .join(&game_name)
          .join("texture_replacements/**/*.png")
          .to_string_lossy(),
      )
      .expect("Failed to read glob pattern")
      {
        match entry {
          Ok(path) => {
            let relative_path = path
              .strip_prefix(
                &entry_path
                  .join("custom_assets")
                  .join(&game_name)
                  .join("texture_replacements"),
              )
              .map_err(|_| {
                CommandError::GameFeatures(format!(
                  "Unable to read texture packs from {}",
                  expected_path.display()
                ))
              })?;
            file_list.push(relative_path.display().to_string().replace("\\", "/"));
          }
          Err(e) => println!("{:?}", e),
        }
      }
      let cover_image_path = match entry_path.join("cover.png").exists() {
        true => Some(entry_path.join("cover.png").to_string_lossy().to_string()),
        false => None,
      };
      let mut pack_info = TexturePackInfo {
        file_list,
        has_metadata: false,
        cover_image_path,
        name: directory_name.to_owned(),
        version: "Unknown Version".to_string(),
        author: "Unknown Author".to_string(),
        release_date: "Unknown Release Date".to_string(),
        supported_games: vec![game_name.clone()], // if no info, assume it's supported
        description: "Unknown Description".to_string(),
        tags: vec![],
      };
      // Read metadata if it's available
      match entry_path.join("metadata.json").exists() {
        true => {
          match std::fs::read_to_string(entry_path.join("metadata.json")) {
            Ok(content) => {
              // Serialize from json
              match serde_json::from_str::<TexturePackInfo>(&content) {
                Ok(pack_metadata) => {
                  pack_info.name = pack_metadata.name;
                  pack_info.version = pack_metadata.version;
                  pack_info.author = pack_metadata.author;
                  pack_info.release_date = pack_metadata.release_date;
                  pack_info.description = pack_metadata.description;
                  pack_info.tags = pack_metadata.tags;
                }
                Err(err) => {
                  log::error!("Unable to parse {}: {}", &content, err);
                }
              }
            }
            Err(err) => {
              log::error!(
                "Unable to read {}: {}",
                entry_path.join("metadata.json").display(),
                err
              );
            }
          };
        }
        false => {}
      }
      package_map.insert(directory_name, pack_info);
    }
  }

  Ok(package_map)
}

#[tauri::command]
pub async fn extract_new_texture_pack(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
  zip_path: String,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract texture pack".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };

  // First, we'll check the zip file to make sure it has a `custom_assets/<game>/texture_replacements` folder before extracting
  let zip_path_buf = PathBuf::from(zip_path);
  let texture_pack_name = match zip_path_buf.file_stem() {
    Some(name) => name.to_string_lossy().to_string(),
    None => {
      return Err(CommandError::GameFeatures(
        "Unable to get texture pack name from zip file path".to_string(),
      ));
    }
  };
  let expected_top_level_dir = format!("custom_assets/{}/texture_replacements", &game_name);
  let valid_zip =
    check_if_zip_contains_top_level_dir(&zip_path_buf, expected_top_level_dir.clone()).map_err(
      |err| {
        log::error!("Unable to read texture replacement zip file: {}", err);
        CommandError::GameFeatures(format!("Unable to read texture replacement pack: {}", err))
      },
    )?;
  if !valid_zip {
    log::error!(
      "Invalid texture pack, no top-level `{}` folder in: {}",
      &expected_top_level_dir,
      zip_path_buf.display()
    );
    return Ok(false);
  }
  // It's valid, let's extract it.  The name of the zip becomes the folder, if one already exists it will be deleted!
  let destination_dir = &install_path
    .join("features")
    .join(game_name)
    .join("texture-packs")
    .join(&texture_pack_name);
  // TODO - delete it
  create_dir(destination_dir).map_err(|err| {
    log::error!("Unable to create directory for texture pack: {}", err);
    CommandError::GameFeatures(format!(
      "Unable to create directory for texture pack: {}",
      err
    ))
  })?;
  extract_zip_file(&zip_path_buf, &destination_dir, false).map_err(|err| {
    log::error!("Unable to extract replacement pack: {}", err);
    CommandError::GameFeatures(format!("Unable to extract texture pack: {}", err))
  })?;
  Ok(true)
}

// TODO -  remove duplication
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameJobStepOutput {
  pub success: bool,
  pub msg: Option<String>,
}

#[tauri::command]
pub async fn update_texture_pack_data(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<GameJobStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Ok(GameJobStepOutput {
        success: false,
        msg: Some("No installation directory set, can't extract texture pack".to_string()),
      });
    }
    Some(path) => Path::new(path),
  };

  let game_texture_pack_dir = install_path
    .join("active")
    .join(&game_name)
    .join("data")
    .join("custom_assets")
    .join(&game_name)
    .join("texture_replacements");
  // Reset texture replacement directory
  delete_dir(&game_texture_pack_dir)?;
  create_dir(&game_texture_pack_dir)?;
  for pack in config_lock
    .game_enabled_textured_packs(&game_name)
    .iter()
    .rev()
  {
    let texture_pack_dir = install_path
      .join("features")
      .join(&game_name)
      .join("texture-packs")
      .join(&pack)
      .join("custom_assets")
      .join(&game_name)
      .join("texture_replacements");
    log::info!("Appending textures from: {}", texture_pack_dir.display());
    match overwrite_dir(&texture_pack_dir, &game_texture_pack_dir) {
      Ok(_) => (),
      Err(err) => {
        log::error!("Unable to update texture replacements: {}", err);
        return Ok(GameJobStepOutput {
          success: false,
          msg: Some(format!("Unable to update texture replacements: {}", err)),
        });
      }
    }
  }

  Ok(GameJobStepOutput {
    success: true,
    msg: None,
  })
}

#[tauri::command]
pub async fn delete_texture_packs(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
  packs: Vec<String>,
) -> Result<GameJobStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Ok(GameJobStepOutput {
        success: false,
        msg: Some("No installation directory set, can't extract texture pack".to_string()),
      });
    }
    Some(path) => Path::new(path),
  };

  let texture_pack_dir = install_path
    .join("features")
    .join(&game_name)
    .join("texture-packs");

  for pack in packs {
    log::info!("Deleting texture pack: {}", pack);
    match delete_dir(&texture_pack_dir.join(&pack)) {
      Ok(_) => (),
      Err(err) => {
        log::error!("Unable to delete texture pack: {}", err);
        return Ok(GameJobStepOutput {
          success: false,
          msg: Some(format!("Unable to delete texture pack: {}", err)),
        });
      }
    }
  }

  Ok(GameJobStepOutput {
    success: true,
    msg: None,
  })
}

#[tauri::command]
pub async fn is_mod_support_enabled(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.mods_enabled.unwrap_or(false))
}

#[tauri::command]
pub async fn add_mod_source(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  url: String,
) -> Result<(), CommandError> {
  log::info!("Adding mod source {}", url);
  let mut config_lock = config.lock().await;
  config_lock.add_new_mod_source(&url).map_err(|err| {
    log::error!("Unable to persist new mod source: {:?}", err);
    CommandError::Configuration("Unable to persist new mod source".to_owned())
  })?;
  Ok(())
}

#[tauri::command]
pub async fn remove_mod_source(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  mod_source_index: i32,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;

  log::info!("Removing mod source at index {}", mod_source_index);
  config_lock
    .remove_mod_source(mod_source_index as usize)
    .map_err(|err| {
      log::error!("Unable to remove mod source: {:?}", err);
      CommandError::Configuration("Unable to remove mod source".to_owned())
    })?;
  Ok(())
}

#[tauri::command]
pub async fn get_mod_sources(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Vec<String>, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.get_mod_sources())
}

#[tauri::command]
pub async fn extract_new_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
  zip_path: String,
  mod_source: String,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };

  // The name of the zip becomes the folder, if one already exists it will be deleted!
  let zip_path_buf = PathBuf::from(zip_path);
  let mod_name = match zip_path_buf.file_stem() {
    Some(name) => name.to_string_lossy().to_string(),
    None => {
      return Err(CommandError::GameFeatures(
        "Unable to get mod name from zip file path".to_string(),
      ));
    }
  };
  let destination_dir = &install_path
    .join("features")
    .join(game_name)
    .join("mods")
    .join(mod_source)
    .join(&mod_name);
  delete_dir(destination_dir)?;
  create_dir(destination_dir).map_err(|err| {
    log::error!("Unable to create directory for mod: {}", err);
    CommandError::GameFeatures(format!("Unable to create directory for mod: {}", err))
  })?;
  extract_zip_file(&zip_path_buf, &destination_dir, false).map_err(|err| {
    log::error!("Unable to extract mod: {}", err);
    CommandError::GameFeatures(format!("Unable to extract mod: {}", err))
  })?;
  Ok(true)
}

#[tauri::command]
pub async fn download_and_extract_new_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
  download_url: String,
  mod_name: String,
  source_name: String,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't download and extract mod".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };

  // Download the file
  let download_path = &install_path
      .join("features")
      .join(game_name)
      .join("mods")
      .join(&source_name)
      .join(&mod_name)
      .join(format!("{mod_name}.zip"));
  // TODO now - safer
  delete_dir(download_path.parent().unwrap())?;

  download_file(download_url.clone(), download_path.to_string_lossy().to_string()).await.map_err(|_| {
    CommandError::GameFeatures("Unable to successfully download mod version".to_owned())
  })?;

  // TODO now - safer
  extract_and_delete_zip_file(&download_path, &download_path.parent().unwrap(), false).map_err(|err| {
    log::error!("Unable to extract mod: {}", err);
    CommandError::GameFeatures(format!("Unable to extract mod: {}", err))
  })?;
  Ok(true)
}

#[tauri::command]
pub async fn base_game_iso_exists(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };
  Ok(
    install_path
      .join("active")
      .join(&game_name)
      .join("data")
      .join("iso_data")
      .join(&game_name)
      .exists(),
  )
}

fn bin_ext(filename: &str) -> String {
  if cfg!(windows) {
    return format!("{filename}.exe");
  }
  filename.to_string()
}

struct ExecutableLocation {
  executable_dir: PathBuf,
  executable_path: PathBuf,
}

fn get_mod_exec_location(
  install_path: std::path::PathBuf,
  executable_name: &str,
  game_name: &str,
  mod_name: &str,
  source_name: &str,
) -> Result<ExecutableLocation, CommandError> {
  let exec_dir = install_path
    .join("features")
    .join(game_name)
    .join("mods")
    .join(source_name)
    .join(mod_name);
  let exec_path = exec_dir.join(bin_ext(executable_name));
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

fn create_log_file(
  app_handle: &tauri::AppHandle,
  name: &str,
  append: bool,
) -> Result<std::fs::File, CommandError> {
  let log_path = &match app_handle.path_resolver().app_log_dir() {
    None => {
      return Err(CommandError::Installation(
        "Could not determine path to save installation logs".to_owned(),
      ))
    }
    Some(path) => path,
  };
  create_dir(log_path)?;
  let mut file_options = std::fs::OpenOptions::new();
  file_options.create(true);
  if append {
    file_options.append(true);
  } else {
    file_options.write(true).truncate(true);
  }
  let file = file_options.open(log_path.join(name))?;
  Ok(file)
}

#[derive(Debug, Serialize, Deserialize)]
struct LauncherErrorCode {
  msg: String,
}

#[tauri::command]
pub async fn extract_iso_for_mod_install(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: String,
  mod_name: String,
  source_name: String,
  path_to_iso: String,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };
  let exec_info = match get_mod_exec_location(
    install_path.to_path_buf(),
    "extractor",
    &game_name,
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
    .join(&game_name)
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
    game_name.clone(),
  ];

  // This is the first install step, reset the file
  let log_file = create_log_file(&app_handle, "extractor.log", false)?;

  log::info!("Running extractor with args: {:?}", args);

  let mut command = Command::new(exec_info.executable_path);
  command
    .args(args)
    .current_dir(exec_info.executable_dir)
    .stdout(log_file.try_clone()?)
    .stderr(log_file.try_clone()?);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let output = command.output()?;
  match output.status.code() {
    Some(code) => {
      if code == 0 {
        log::info!("extraction and validation was successful");
        return Ok(InstallStepOutput {
          success: true,
          msg: None,
        });
      }
      // TODO - implement error code checking if possible
      // let error_code_map = get_error_codes(&config_info, &game_name);
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      // let message = error_code_map.get(&code).unwrap_or(&default_error);
      log::error!("extraction and validation was not successful. Code {code}");
      log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      log::error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
      Ok(InstallStepOutput {
        success: false,
        msg: Some(default_error.msg.clone()),
      })
    }
    None => {
      log::error!("extraction and validation was not successful. No status code");
      log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      log::error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
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
  game_name: String,
  mod_name: String,
  source_name: String,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };
  let exec_info = match get_mod_exec_location(
    install_path.to_path_buf(),
    "extractor",
    &game_name,
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
    .join(&game_name)
    .join("data")
    .join("iso_data")
    .join(&game_name)
    .to_path_buf();

  let args = vec![
    iso_dir.clone().to_string_lossy().into_owned(),
    "--folder".to_string(),
    "--decompile".to_string(),
    "--game".to_string(),
    game_name.clone(),
  ];

  // This is the first install step, reset the file
  let log_file = create_log_file(&app_handle, "extractor.log", false)?;

  log::info!("Running extractor with args: {:?}", args);

  let mut command = Command::new(exec_info.executable_path);
  command
    .args(args)
    .current_dir(exec_info.executable_dir)
    .stdout(log_file.try_clone()?)
    .stderr(log_file.try_clone()?);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let output = command.output()?;
  match output.status.code() {
    Some(code) => {
      if code == 0 {
        log::info!("extraction and validation was successful");
        return Ok(InstallStepOutput {
          success: true,
          msg: None,
        });
      }
      // TODO - implement error code checking if possible
      // let error_code_map = get_error_codes(&config_info, &game_name);
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      // let message = error_code_map.get(&code).unwrap_or(&default_error);
      log::error!("extraction and validation was not successful. Code {code}");
      log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      log::error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
      Ok(InstallStepOutput {
        success: false,
        msg: Some(default_error.msg.clone()),
      })
    }
    None => {
      log::error!("extraction and validation was not successful. No status code");
      log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      log::error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
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
  game_name: String,
  mod_name: String,
  source_name: String,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };
  let exec_info = match get_mod_exec_location(
    install_path.to_path_buf(),
    "extractor",
    &game_name,
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
    .join(&game_name)
    .join("data")
    .join("iso_data")
    .join(&game_name)
    .to_path_buf();

  let args = vec![
    iso_dir.clone().to_string_lossy().into_owned(),
    "--folder".to_string(),
    "--compile".to_string(),
    "--game".to_string(),
    game_name.clone(),
  ];

  // This is the first install step, reset the file
  let log_file = create_log_file(&app_handle, "extractor.log", false)?;

  log::info!("Running extractor with args: {:?}", args);

  let mut command = Command::new(exec_info.executable_path);
  command
    .args(args)
    .current_dir(exec_info.executable_dir)
    .stdout(log_file.try_clone()?)
    .stderr(log_file.try_clone()?);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let output = command.output()?;
  match output.status.code() {
    Some(code) => {
      if code == 0 {
        log::info!("extraction and validation was successful");
        return Ok(InstallStepOutput {
          success: true,
          msg: None,
        });
      }
      // TODO - implement error code checking if possible
      // let error_code_map = get_error_codes(&config_info, &game_name);
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      // let message = error_code_map.get(&code).unwrap_or(&default_error);
      log::error!("extraction and validation was not successful. Code {code}");
      log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      log::error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
      Ok(InstallStepOutput {
        success: false,
        msg: Some(default_error.msg.clone()),
      })
    }
    None => {
      log::error!("extraction and validation was not successful. No status code");
      log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      log::error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
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
  game_name: String,
  mod_name: String,
  source_name: String,
  version_name: String,
) -> Result<InstallStepOutput, CommandError> {
  let mut config_lock = config.lock().await;
  log::info!(
    "Saving mod install info {}, {}, {}, {}",
    game_name,
    mod_name,
    source_name,
    version_name
  );
  config_lock
    .save_mod_install_info(game_name, mod_name, source_name, version_name)
    .map_err(|err| {
      log::error!("Unable to remove mod source: {:?}", err);
      CommandError::Configuration("Unable to remove mod source".to_owned())
    })?;
  Ok(InstallStepOutput {
    success: true,
    msg: None,
  })
}

#[tauri::command]
pub async fn get_installed_mods(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<HashMap<String, HashMap<String, String>>, CommandError> {
  let config_lock = config.lock().await;
  match config_lock.get_installed_mods(game_name) {
    Ok(result) => Ok(result),
    Err(err) => {
      log::error!("Unable to retrieve installed mods: {:?}", err);
      Err(CommandError::Configuration(
        "Unable to retrieve installed mods".to_owned(),
      ))
    }
  }
}

#[tauri::command]
pub async fn launch_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: String,
  in_debug: bool,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };
  let config_dir = install_path
    .join("features")
    .join(&game_name)
    .join("mods")
    .join(&source_name)
    .join("_settings")
    .join(&mod_name);
  let exec_info = get_mod_exec_location(
    install_path.to_path_buf(),
    "gk",
    &game_name,
    &mod_name,
    &source_name,
  )?;
  let mut args = vec![
    "-v".to_string(),
    "--game".to_string(),
    game_name.clone(),
    "--config-path".to_string(),
    config_dir.to_string_lossy().to_string(),
    "--".to_string(),
    "-boot".to_string(),
    "-fakeiso".to_string(),
  ];
  if in_debug {
    args.push("-debug".to_string());
  }

  log::info!("Launching gk args: {:?}", args);

  let log_file = create_log_file(&app_handle, "mod.log", false)?;

  // TODO - log rotation here would be nice too, and for it to be game/mod specific
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
  let mut child = command.spawn()?;
  // if all goes well, we await the child to exit in the background (separate thread)
  tokio::spawn(async move {
    // let start_time = Instant::now(); // get the start time of the game
    //                                  // start waiting for the game to exit
    // if let Err(err) = child.wait() {
    //   log::error!("Error occured when waiting for game to exit: {}", err);
    //   return;
    // }
    // // once the game exits pass the time the game started to the track_playtine function
    // if let Err(err) = track_playtime(start_time, game_name).await {
    //   log::error!("Error occured when tracking playtime: {}", err);
    //   return;
    // }
  });
  Ok(())
}

#[tauri::command]
pub async fn get_local_mod_thumbnail_base64(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
  mod_name: String,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => return Ok("".to_string()),
    Some(path) => Path::new(path),
  };

  let cover_path = install_path
    .join("features")
    .join(game_name)
    .join("mods")
    .join("_local")
    .join(mod_name)
    .join("thumbnail.png");
  if cover_path.exists() {
    // todo now - safer
    return Ok(to_image_base64(cover_path.to_str().unwrap()));
  }
  Ok("".to_string())
}

#[tauri::command]
pub async fn get_local_mod_cover_base64(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
  mod_name: String,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => return Ok("".to_string()),
    Some(path) => Path::new(path),
  };

  let cover_path = install_path
    .join("features")
    .join(game_name)
    .join("mods")
    .join("_local")
    .join(mod_name)
    .join("cover.png");
  if cover_path.exists() {
    // todo now - safer
    return Ok(to_image_base64(cover_path.to_str().unwrap()));
  }
  Ok("".to_string())
}

#[tauri::command]
pub async fn uninstall_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: String,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract mod".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };
  let mod_dir = install_path
    .join("features")
    .join(&game_name)
    .join("mods")
    .join(&source_name)
    .join(&mod_name);
  std::fs::remove_dir_all(mod_dir)?;
  config_lock
    .uninstall_mod(game_name, mod_name, source_name)
    .map_err(|_| {
      CommandError::GameFeatures("Unable to uninstall mod".to_owned())
    })?;
  Ok(())
}