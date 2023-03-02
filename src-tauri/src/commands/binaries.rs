use std::io::{BufRead, BufReader};
use std::{
  path::{Path, PathBuf},
  process::Command,
};

use crate::{config::LauncherConfig, util::file::create_dir};

use super::CommandError;

// TODO - update data dir command

fn bin_ext(filename: &str) -> String {
  if cfg!(windows) {
    return format!("{}.exe", filename);
  }
  return filename.to_string();
}

struct CommonConfigData {
  install_path: std::path::PathBuf,
  active_version: String,
  active_version_folder: String,
}

fn common_prelude(
  config: &tokio::sync::MutexGuard<LauncherConfig>,
) -> Result<CommonConfigData, CommandError> {
  let install_path = match &config.installation_dir {
    None => {
      return Err(CommandError::BinaryExecution(format!(
        "No installation directory set, can't perform operation"
      )))
    }
    Some(path) => Path::new(path),
  };

  let active_version = config
    .active_version
    .as_ref()
    .ok_or(CommandError::BinaryExecution(format!(
      "No active version set, can't perform operation"
    )))?;

  let active_version_folder =
    config
      .active_version_folder
      .as_ref()
      .ok_or(CommandError::BinaryExecution(format!(
        "No active version folder set, can't perform operation"
      )))?;

  Ok(CommonConfigData {
    install_path: install_path.to_path_buf(),
    active_version: active_version.clone(),
    active_version_folder: active_version_folder.clone(),
  })
}

fn get_data_dir(
  config_info: &CommonConfigData,
  game_name: &String,
) -> Result<PathBuf, CommandError> {
  let data_folder = config_info
    .install_path
    .join("active")
    .join(game_name)
    .join("data");
  if !data_folder.exists() {
    return Err(CommandError::BinaryExecution(format!(
      "Could not locate relevant data directory '{}', can't perform operation",
      data_folder.to_string_lossy()
    )));
  }
  Ok(data_folder)
}

struct ExecutableLocation {
  executable_dir: PathBuf,
  executable_path: PathBuf,
}

fn get_exec_location(
  config_info: &CommonConfigData,
  executable_name: &str,
) -> Result<ExecutableLocation, CommandError> {
  let exec_dir = config_info
    .install_path
    .join("versions")
    .join(&config_info.active_version_folder)
    .join(&config_info.active_version);
  let exec_path = exec_dir.join(bin_ext(executable_name));
  if !exec_path.exists() {
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
      return Err(CommandError::Installation(format!(
        "Could not determine path to save installation logs"
      )))
    }
    Some(path) => path.clone(),
  };
  create_dir(&log_path)?;
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

#[tauri::command]
pub async fn extract_and_validate_iso(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: String,
  game_name: String,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  let data_folder = get_data_dir(&config_info, &game_name)?;
  let exec_info = get_exec_location(&config_info, "extractor")?;

  let mut args = vec![
    path_to_iso.clone(),
    "--extract".to_string(),
    "--validate".to_string(),
    "--proj-path".to_string(),
    data_folder.to_string_lossy().into_owned(),
  ];
  if Path::new(&path_to_iso.clone()).is_dir() {
    args.push("--folder".to_string());
  }

  // This is the first install step, reset the file
  let log_file = create_log_file(&app_handle, "extractor.log", false)?;

  // TODO - exit codes
  let output = Command::new(exec_info.executable_path)
    .args(args)
    .current_dir(exec_info.executable_dir)
    .stdout(log_file.try_clone().unwrap())
    .stderr(log_file)
    .output()?;
  Ok(())
}

#[tauri::command]
pub async fn run_decompiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: String,
  game_name: String,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  let data_folder = get_data_dir(&config_info, &game_name)?;
  let exec_info = get_exec_location(&config_info, "extractor")?;

  let mut source_path = path_to_iso;
  if source_path.is_empty() {
    source_path = data_folder
      .join("iso_data")
      .join(game_name)
      .to_string_lossy()
      .to_string();
  }

  // TODO - handle error codes
  let log_file = create_log_file(&app_handle, "extractor.log", true)?;
  let output = Command::new(&exec_info.executable_path)
    .args([
      source_path,
      "--decompile".to_string(),
      "--proj-path".to_string(),
      data_folder.to_string_lossy().into_owned(),
    ])
    .stdout(log_file.try_clone().unwrap())
    .stderr(log_file)
    .current_dir(exec_info.executable_dir)
    .output()?;
  Ok(())
}

#[tauri::command]
pub async fn run_compiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: String,
  game_name: String,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  let data_folder = get_data_dir(&config_info, &game_name)?;
  let exec_info = get_exec_location(&config_info, "extractor")?;

  let mut source_path = path_to_iso;
  if source_path.is_empty() {
    source_path = data_folder
      .join("iso_data")
      .join(game_name)
      .to_string_lossy()
      .to_string();
  }

  // TODO - handle error codes
  let log_file = create_log_file(&app_handle, "extractor.log", true)?;
  let output = Command::new(&exec_info.executable_path)
    .args([
      source_path,
      "--compile".to_string(),
      "--proj-path".to_string(),
      data_folder.to_string_lossy().into_owned(),
    ])
    .stdout(log_file.try_clone().unwrap())
    .stderr(log_file)
    .current_dir(exec_info.executable_dir)
    .output()?;
  Ok(())
}

#[tauri::command]
pub async fn open_repl(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<(), CommandError> {
  // TODO - explore a linux option though this is very annoying because without doing a ton of research
  // we seem to have to handle various terminals.  Which honestly we should probably do on windows too
  //
  // So maybe we can make a menu where the user will specify what terminal to use / what launch-options to use
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  let data_folder = get_data_dir(&config_info, &game_name)?;
  let exec_info = get_exec_location(&config_info, "goalc")?;
  // TODO - handle error codes
  let output = Command::new("cmd")
    .args([
      "/K",
      "start",
      &bin_ext("goalc"),
      "--proj-path",
      &data_folder.to_string_lossy().into_owned(),
    ])
    .current_dir(exec_info.executable_dir)
    .spawn()?;
  Ok(())
}

#[tauri::command]
pub async fn launch_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: String,
  in_debug: bool,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  let data_folder = get_data_dir(&config_info, &game_name)?;
  let exec_info = get_exec_location(&config_info, "gk")?;

  let mut args = vec!["-boot".to_string(), "-fakeiso".to_string()];
  // TODO - order unfortunately matters for gk args, this will be fixed eventually...
  if in_debug {
    args.push("-debug".to_string());
  }
  args.push("-proj-path".to_string());
  args.push(data_folder.to_string_lossy().into_owned());
  // TODO - handle error codes
  let log_file = create_log_file(&app_handle, "game.log", false)?;
  let output = Command::new(exec_info.executable_path)
    .args(args)
    .stdout(log_file.try_clone().unwrap())
    .stderr(log_file)
    .current_dir(exec_info.executable_dir)
    .spawn()?;
  Ok(())
}
