#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
  collections::HashMap,
  fs::File,
  io::prelude::*,
  path::{Path, PathBuf},
  process::Command,
  thread,
  time::Instant,
};

use log::{info, warn};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{api::path, Manager};

use crate::{
  config::LauncherConfig,
  util::file::{create_dir, overwrite_dir, read_last_lines_from_file},
};

use super::CommandError;

fn bin_ext(filename: &str) -> String {
  if cfg!(windows) {
    return format!("{filename}.exe");
  }
  filename.to_string()
}

struct CommonConfigData {
  install_path: std::path::PathBuf,
  active_version: String,
  active_version_folder: String,
  tooling_version: Version,
}

fn common_prelude(
  config: &tokio::sync::MutexGuard<LauncherConfig>,
) -> Result<CommonConfigData, CommandError> {
  let install_path = match &config.installation_dir {
    None => {
      return Err(CommandError::BinaryExecution(
        "No installation directory set, can't perform operation".to_owned(),
      ))
    }
    Some(path) => Path::new(path),
  };

  let active_version = config
    .active_version
    .as_ref()
    .ok_or(CommandError::BinaryExecution(
      "No active version set, can't perform operation".to_owned(),
    ))?;

  let active_version_folder =
    config
      .active_version_folder
      .as_ref()
      .ok_or(CommandError::BinaryExecution(
        "No active version folder set, can't perform operation".to_owned(),
      ))?;

  let tooling_version = Version::parse(active_version.strip_prefix('v').unwrap_or(&active_version))
    .unwrap_or(Version::new(0, 1, 35)); // assume new format if none can be found

  Ok(CommonConfigData {
    install_path: install_path.to_path_buf(),
    active_version: active_version.clone(),
    active_version_folder: active_version_folder.clone(),
    tooling_version: tooling_version,
  })
}

#[derive(Debug, Serialize, Deserialize)]
struct LauncherErrorCode {
  msg: String,
}

fn get_error_codes(
  config: &CommonConfigData,
  game_name: &String,
) -> HashMap<i32, LauncherErrorCode> {
  let json_file = config
    .install_path
    .join("active")
    .join(game_name)
    .join("data")
    .join("launcher")
    .join("error-code-metadata.json");
  if !json_file.exists() {
    warn!("couldn't locate error code file at {}", json_file.display());
    return HashMap::new();
  }
  let file_contents = match std::fs::read_to_string(&json_file) {
    Ok(content) => content,
    Err(_err) => {
      warn!("couldn't read error code file at {}", &json_file.display());
      return HashMap::new();
    }
  };
  let json: Value = match serde_json::from_str(&file_contents) {
    Ok(json) => json,
    Err(_err) => {
      warn!("couldn't parse error code file at {}", &json_file.display());
      return HashMap::new();
    }
  };

  if let Value::Object(map) = json {
    let mut result: HashMap<i32, LauncherErrorCode> = HashMap::new();
    for (key, value) in map {
      let Ok(error_code) = serde_json::from_value(value) else {
        continue;
      };
      let Ok(code) = key.parse::<i32>() else {
        continue;
      };
      result.insert(code, error_code);
    }
    return result;
  }

  warn!(
    "couldn't convert error code file at {}",
    &json_file.display()
  );

  HashMap::new()
}

fn copy_data_dir(config_info: &CommonConfigData, game_name: &String) -> Result<(), CommandError> {
  let src_dir = config_info
    .install_path
    .join("versions")
    .join(&config_info.active_version_folder)
    .join(&config_info.active_version)
    .join("data");

  let dst_dir = config_info
    .install_path
    .join("active")
    .join(game_name)
    .join("data");

  info!("Copying {} into {}", src_dir.display(), dst_dir.display());

  overwrite_dir(&src_dir, &dst_dir).map_err(|err| {
    CommandError::Installation(format!("Unable to copy data directory: '{err}'",))
  })?;
  Ok(())
}

fn get_data_dir(
  config_info: &CommonConfigData,
  game_name: &String,
  copy_directory: bool,
) -> Result<PathBuf, CommandError> {
  let data_folder = config_info
    .install_path
    .join("active")
    .join(game_name)
    .join("data");
  if !data_folder.exists() && !copy_directory {
    return Err(CommandError::BinaryExecution(format!(
      "Could not locate relevant data directory '{}', can't perform operation",
      data_folder.to_string_lossy()
    )));
  } else if copy_directory {
    copy_data_dir(config_info, game_name)?;
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
#[serde(rename_all = "camelCase")]
pub struct InstallStepOutput {
  pub success: bool,
  pub msg: Option<String>,
}

#[tauri::command]
pub async fn update_data_directory(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  copy_data_dir(&config_info, &game_name)?;

  Ok(InstallStepOutput {
    success: true,
    msg: None,
  })
}

#[tauri::command]
pub async fn get_end_of_logs(app_handle: tauri::AppHandle) -> Result<String, CommandError> {
  Ok(read_last_lines_from_file(
    &app_handle
      .path_resolver()
      .app_log_dir()
      .unwrap()
      .join("extractor.log"),
    250,
  )?)
}

#[tauri::command]
pub async fn extract_and_validate_iso(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: String,
  game_name: String,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  let data_folder = get_data_dir(&config_info, &game_name, true)?;
  log::info!(
    "extracting using data folder: {}",
    data_folder.to_string_lossy()
  );
  let exec_info = match get_exec_location(&config_info, "extractor") {
    Ok(exec_info) => exec_info,
    Err(_) => {
      log::error!("extractor executable not found");
      return Ok(InstallStepOutput {
        success: false,
        msg: Some("Tooling appears to be missing critical files. This may be caused by antivirus software. You will need to redownload the version and try again.".to_string()),
      });
    }
  };

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
  // Add new --game argument
  if config_info.tooling_version.minor >= 1 && config_info.tooling_version.patch >= 44 {
    args.push("--game".to_string());
    args.push(game_name.clone());
  }

  // This is the first install step, reset the file
  let log_file = create_log_file(&app_handle, "extractor.log", false)?;

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
      let error_code_map = get_error_codes(&config_info, &game_name);
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      let message = error_code_map.get(&code).unwrap_or(&default_error);
      log::error!("extraction and validation was not successful. Code {code}");
      log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      log::error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
      Ok(InstallStepOutput {
        success: false,
        msg: Some(message.msg.clone()),
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
pub async fn run_decompiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: String,
  game_name: String,
  truncate_logs: bool,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  let data_folder = get_data_dir(&config_info, &game_name, false)?;
  log::info!(
    "decompiling using data folder: {}",
    data_folder.to_string_lossy()
  );
  let exec_info = match get_exec_location(&config_info, "extractor") {
    Ok(exec_info) => exec_info,
    Err(_) => {
      log::error!("extractor executable not found");
      return Ok(InstallStepOutput {
        success: false,
        msg: Some("Tooling appears to be missing critical files. This may be caused by antivirus software. You will need to redownload the version and try again.".to_string()),
      });
    }
  };

  let mut source_path = path_to_iso;
  if source_path.is_empty() {
    source_path = data_folder
      .join("iso_data")
      .join(&game_name)
      .to_string_lossy()
      .to_string();
  }

  let log_file = create_log_file(&app_handle, "extractor.log", !truncate_logs)?;
  let mut command = Command::new(exec_info.executable_path);

  let mut args = vec![
    source_path,
    "--decompile".to_string(),
    "--proj-path".to_string(),
    data_folder.to_string_lossy().into_owned(),
  ];
  // Add new --game argument
  if config_info.tooling_version.minor >= 1 && config_info.tooling_version.patch >= 44 {
    args.push("--game".to_string());
    args.push(game_name.clone());
  }

  command
    .args(args)
    .stdout(log_file.try_clone()?)
    .stderr(log_file)
    .current_dir(exec_info.executable_dir);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let output = command.output()?;
  match output.status.code() {
    Some(code) => {
      if code == 0 {
        log::info!("decompilation was successful");
        return Ok(InstallStepOutput {
          success: true,
          msg: None,
        });
      }
      let error_code_map = get_error_codes(&config_info, &game_name);
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      let message = error_code_map.get(&code).unwrap_or(&default_error);
      log::error!("decompilation was not successful. Code {code}");
      log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      log::error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
      Ok(InstallStepOutput {
        success: false,
        msg: Some(message.msg.clone()),
      })
    }
    None => {
      log::error!("decompilation was not successful. No status code");
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
pub async fn run_compiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: String,
  game_name: String,
  truncate_logs: bool,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  let data_folder = get_data_dir(&config_info, &game_name, false)?;
  log::info!(
    "compiling using data folder: {}",
    data_folder.to_string_lossy()
  );
  let exec_info = match get_exec_location(&config_info, "extractor") {
    Ok(exec_info) => exec_info,
    Err(_) => {
      return Ok(InstallStepOutput {
        success: false,
        msg: Some("Tooling appears to be missing critical files. This may be caused by antivirus software. You will need to redownload the version and try again.".to_string()),
      })
    }
  };

  let mut source_path = path_to_iso;
  if source_path.is_empty() {
    source_path = data_folder
      .join("iso_data")
      .join(&game_name)
      .to_string_lossy()
      .to_string();
  }

  let log_file = create_log_file(&app_handle, "extractor.log", !truncate_logs)?;
  let mut args = vec![
    source_path,
    "--compile".to_string(),
    "--proj-path".to_string(),
    data_folder.to_string_lossy().into_owned(),
  ];
  // Add new --game argument
  if config_info.tooling_version.minor >= 1 && config_info.tooling_version.patch >= 44 {
    args.push("--game".to_string());
    args.push(game_name.clone());
  }
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
  let output = command.output()?;
  match output.status.code() {
    Some(code) => {
      if code == 0 {
        log::info!("compilation was successful");
        return Ok(InstallStepOutput {
          success: true,
          msg: None,
        });
      }
      let error_code_map = get_error_codes(&config_info, &game_name);
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      let message = error_code_map.get(&code).unwrap_or(&default_error);
      log::error!("compilation was not successful. Code {code}");
      log::error!("stderr: {}", String::from_utf8_lossy(&output.stderr));
      log::error!("stdout: {}", String::from_utf8_lossy(&output.stdout));
      Ok(InstallStepOutput {
        success: false,
        msg: Some(message.msg.clone()),
      })
    }
    None => {
      log::error!("compilation was not successful. No status code");
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

  let data_folder = get_data_dir(&config_info, &game_name, false)?;
  let exec_info = get_exec_location(&config_info, "goalc")?;
  let mut command = Command::new("cmd");
  command
    .args([
      "/K",
      "start",
      &bin_ext("goalc"),
      "--proj-path",
      &data_folder.to_string_lossy(),
    ])
    .current_dir(exec_info.executable_dir);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  command.spawn()?;
  Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GPUTestOutput {
  error: String,
  error_cause: String,
  success: bool,
}

pub async fn run_game_gpu_test(
  config_lock: &tokio::sync::MutexGuard<'_, LauncherConfig>,
  app_handle: tauri::AppHandle,
) -> Result<Option<bool>, CommandError> {
  let config_info = common_prelude(config_lock)?;

  let exec_info = get_exec_location(&config_info, "gk")?;
  let gpu_test_result_path = &match app_handle.path_resolver().app_data_dir() {
    None => {
      return Err(CommandError::BinaryExecution(
        "Could not determine path to save GPU test results".to_owned(),
      ))
    }
    Some(path) => path,
  };
  create_dir(gpu_test_result_path)?;
  let gpu_test_result_path = &gpu_test_result_path.join("gpu-test-result.json");

  log::info!(
    "Running GPU test on game version {:?} and storing in folder: {:?}",
    &config_info.active_version,
    gpu_test_result_path
  );

  let mut command = Command::new(exec_info.executable_path);
  command
    .args([
      "-v".to_string(),
      "--gpu-test".to_string(),
      "opengl".to_string(),
      "--gpu-test-out-path".to_string(),
      gpu_test_result_path.to_string_lossy().into_owned(),
    ])
    .current_dir(exec_info.executable_dir);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let output = command.output()?;
  match output.status.code() {
    Some(code) => {
      if code == 0 {
        // Parse the JSON file
        // Read the file
        let content = match std::fs::read_to_string(gpu_test_result_path) {
          Ok(content) => content,
          Err(err) => {
            log::error!("Unable to read {}: {}", gpu_test_result_path.display(), err);
            return Ok(None);
          }
        };

        // Serialize from json
        match serde_json::from_str::<GPUTestOutput>(&content) {
          Ok(test_results) => {
            return Ok(Some(test_results.success));
          }
          Err(err) => {
            log::error!("Unable to parse {}: {}", &content, err);
            return Ok(None);
          }
        }
      } else {
        return Ok(None);
      }
    }
    None => {
      return Ok(None);
    }
  }
}

fn generate_launch_game_string(
  config_info: &CommonConfigData,
  game_name: String,
  in_debug: bool,
) -> Result<Vec<String>, CommandError> {
  let data_folder = get_data_dir(&config_info, &game_name, false)?;

  let mut args;
  // NOTE - order unfortunately matters for gk args
  if config_info.tooling_version.major == 0
    && config_info.tooling_version.minor <= 1
    && config_info.tooling_version.patch < 35
  {
    // old argument format
    args = vec![
      "-boot".to_string(),
      "-fakeiso".to_string(),
      "-proj-path".to_string(),
      data_folder.to_string_lossy().into_owned(),
    ];
    if in_debug {
      args.push("-debug".to_string());
    }
  } else {
    args = vec![
      "-v".to_string(),
      "--proj-path".to_string(),
      data_folder.to_string_lossy().into_owned(),
    ];
    // Add new --game argument
    if config_info.tooling_version.minor >= 1 && config_info.tooling_version.patch >= 44 {
      args.push("--game".to_string());
      args.push(game_name.clone());
    }
    // passthru args
    args.push("--".to_string());
    args.push("-boot".to_string());
    args.push("-fakeiso".to_string());
    if in_debug {
      args.push("-debug".to_string());
    }
  }
  Ok(args)
}

#[tauri::command]
pub async fn get_launch_game_string(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  let exec_info = get_exec_location(&config_info, "gk")?;
  let args = generate_launch_game_string(&config_info, game_name, false)?;

  Ok(format!(
    "{} {}",
    exec_info.executable_path.display(),
    args.join(" ")
  ))
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

  let exec_info = get_exec_location(&config_info, "gk")?;
  let args = generate_launch_game_string(&config_info, game_name, in_debug)?;

  log::info!(
    "Launching game version {:?} -> {:?} with args: {:?}",
    &config_info.active_version,
    &config_info.tooling_version,
    args
  );

  // TODO - log rotation here would be nice too, and for it to be game specific
  let log_file = create_log_file(&app_handle, "game.log", false)?;
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

  let start_time = Instant::now(); // get the start time of the game

  if let Ok(mut child) = command.spawn() {
    // move all playtime tracking to a separate thread
    thread::spawn(move || {
      // start waiting for the game to exit
      if let Err(err) = child.wait() {
        log::error!("Error occured when waiting for game to exit: {}", err);
        return;
      }

      // once the game exits pass the time the game started to the track_playtine function
      if let Err(err) = track_playtime(start_time, app_handle) {
        log::error!("Error occured when tracking playtime: {}", err);
        return;
      }
    });
  }
  Ok(())
}

fn track_playtime(
  start_time: std::time::Instant,
  app_handle: tauri::AppHandle,
) -> Result<Option<bool>, CommandError> {
  // get the playtime of the session
  let mut elapsed_time = start_time.elapsed().as_secs();

  let config_dir = match path::config_dir() {
    None => {
      log::error!("Couldn't determine application config directory");
      return Err(CommandError::BinaryExecution(
        "Couldn't determine application config directory".to_owned(),
      ));
    }
    Some(path) => path,
  };

  // save the playtime in the config directory when the game is closed
  let playtime_path = config_dir.join("OpenGOAL-Launcher").join("playtime.txt");

  // initialise the playtime integer
  let mut existing_playtime = 0;

  // if playtime.txt exists, read the playtime.txt file and get the existing value
  if let Ok(mut file) = File::open(&playtime_path) {
    let mut contents = String::new();

    if let Err(err) = file.read_to_string(&mut contents) {
      log::error!("Could not read playtime.txt: {}", err);
      return Err(CommandError::BinaryExecution(format!(
        "Could not read playtime.txt: {}",
        err
      )));
    }

    // if there is a int parse error and set the existing playtime to 0
    existing_playtime = match contents.trim().parse::<u64>() {
      Ok(playtime) => playtime,
      Err(_) => 0,
    };
  }

  // add the times together
  elapsed_time = existing_playtime + elapsed_time;

  // create the playtime file
  let mut file = File::create(playtime_path)?;

  // add the new value to the playtime file
  if let Err(err) = file.write_all(elapsed_time.to_string().as_bytes()) {
    log::error!("Could not write playtime to file: {}", err);
    return Err(CommandError::BinaryExecution(format!(
      "Could not write playtime to file: {}",
      err
    )));
  }

  // send an event to the front end so that it can refresh the playtime on screen
  if let Err(err) = app_handle.emit_all("playtimeUpdated", ()) {
    log::error!("Failed to emit playtimeUpdated event: {}", err);
    return Err(CommandError::BinaryExecution(format!(
      "Failed to emit playtimeUpdated event: {}",
      err
    )));
  }

  Ok(None)
}
