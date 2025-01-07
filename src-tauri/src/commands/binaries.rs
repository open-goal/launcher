#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
  collections::HashMap,
  io::ErrorKind,
  path::{Path, PathBuf},
  process::Stdio,
  str::FromStr,
  time::Instant,
};
use tokio::{io::AsyncWriteExt, process::Command};

use log::{info, warn};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::Manager;

use crate::{
  config::LauncherConfig,
  util::{
    file::overwrite_dir,
    process::{create_log_file, create_std_log_file, watch_process},
  },
  TAURI_APP,
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
  tooling_version: Version,
}

#[derive(Clone, serde::Serialize)]
struct ToastPayload {
  toast: String,
  level: String,
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

  let tooling_version = Version::parse(active_version.strip_prefix('v').unwrap_or(&active_version))
    .unwrap_or(Version::new(0, 1, 35)); // assume new format if none can be found

  Ok(CommonConfigData {
    install_path: install_path.to_path_buf(),
    active_version: active_version.clone(),
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
    .join("official")
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
    .join("official")
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
  if config_info.tooling_version.minor > 1 || config_info.tooling_version.patch >= 44 {
    args.push("--game".to_string());
    args.push(game_name.clone());
  }

  log::info!("Running extractor with args: {:?}", args);

  let mut command = Command::new(exec_info.executable_path);
  command
    .args(args)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .current_dir(exec_info.executable_dir);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let mut child = command.spawn()?;

  // This is the first install step, reset the file
  let mut log_file =
    create_log_file(&app_handle, format!("extractor-{game_name}.log"), true).await?;

  let process_status = watch_process(&mut log_file, &mut child, &app_handle).await?;
  log_file.flush().await?;
  match process_status.code() {
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
      Ok(InstallStepOutput {
        success: false,
        msg: Some(message.msg.clone()),
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
pub async fn run_decompiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: String,
  game_name: String,
  truncate_logs: bool,
  use_decomp_settings: bool,
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

  let mut command = Command::new(exec_info.executable_path);

  let mut decomp_config_overrides = vec![];
  if use_decomp_settings {
    let decomp_settings = &config_lock.decompiler_settings;
    if decomp_settings.rip_levels_enabled {
      decomp_config_overrides.push(format!(
        "\"rip_levels\": {}",
        decomp_settings.rip_levels_enabled
      ));
    }
    if decomp_settings.rip_collision_enabled {
      decomp_config_overrides.push(format!(
        "\"rip_collision\": {}",
        decomp_settings.rip_collision_enabled
      ));
    }
    if decomp_settings.rip_textures_enabled {
      decomp_config_overrides.push(format!(
        "\"save_texture_pngs\": {}",
        decomp_settings.rip_textures_enabled
      ));
    }
    if decomp_settings.rip_streamed_audio_enabled {
      decomp_config_overrides.push(format!(
        "\"rip_streamed_audio\": {}",
        decomp_settings.rip_streamed_audio_enabled
      ));
    }
  }

  let mut args = vec![
    source_path,
    "--decompile".to_string(),
    "--proj-path".to_string(),
    data_folder.to_string_lossy().into_owned(),
  ];

  // Add new --game argument
  if config_info.tooling_version.minor > 1 || config_info.tooling_version.patch >= 44 {
    args.push("--game".to_string());
    args.push(game_name.clone());
  }

  // TODO NOW - minimum
  if !decomp_config_overrides.is_empty() {
    args.push("--decomp-config-override".to_string());
    args.push(format!("{{{}}}", decomp_config_overrides.join(", ")));
  }

  log::info!("Running extractor with args: {:?}", args);

  command
    .args(args)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .current_dir(exec_info.executable_dir);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }

  let mut child = command.spawn()?;

  let mut log_file = create_log_file(
    &app_handle,
    format!("extractor-{game_name}.log"),
    !truncate_logs,
  )
  .await?;

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
      let error_code_map = get_error_codes(&config_info, &game_name);
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      let message = error_code_map.get(&code).unwrap_or(&default_error);
      log::error!("decompilation was not successful. Code {code}");
      Ok(InstallStepOutput {
        success: false,
        msg: Some(message.msg.clone()),
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

  let mut args = vec![
    source_path,
    "--compile".to_string(),
    "--proj-path".to_string(),
    data_folder.to_string_lossy().into_owned(),
  ];
  // Add new --game argument
  if config_info.tooling_version.minor > 1 || config_info.tooling_version.patch >= 44 {
    args.push("--game".to_string());
    args.push(game_name.clone());
  }

  log::info!("Running compiler with args: {:?}", args);

  let mut command = Command::new(exec_info.executable_path);
  command
    .args(args)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .current_dir(exec_info.executable_dir);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let mut child = command.spawn()?;

  let mut log_file = create_log_file(
    &app_handle,
    format!("extractor-{game_name}.log"),
    !truncate_logs,
  )
  .await?;

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
      let error_code_map = get_error_codes(&config_info, &game_name);
      let default_error = LauncherErrorCode {
        msg: format!("Unexpected error occured with code {code}"),
      };
      let message = error_code_map.get(&code).unwrap_or(&default_error);
      log::error!("compilation was not successful. Code {code}");
      Ok(InstallStepOutput {
        success: false,
        msg: Some(message.msg.clone()),
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
pub async fn open_repl(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: String,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;
  let data_folder = get_data_dir(&config_info, &game_name, false)?;
  let exec_info = get_exec_location(&config_info, "goalc")?;
  let mut command;
  #[cfg(windows)]
  {
    command = std::process::Command::new("cmd");
    command
      .args([
        "/C",
        "start",
        &bin_ext("goalc"),
        "--game",
        &game_name,
        "--proj-path",
        &data_folder.to_string_lossy(),
      ])
      .current_dir(exec_info.executable_dir)
      .creation_flags(0x08000000);
  }
  #[cfg(target_os = "linux")]
  {
    command = std::process::Command::new("xdg-terminal-exec");
    command
      .args(["./goalc", "--proj-path", &data_folder.to_string_lossy()])
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
        "--proj-path",
        &data_folder.to_string_lossy(),
      ])
      .current_dir(exec_info.executable_dir);
  }
  match command.spawn() {
    Ok(_) => Ok(()),
    Err(e) => {
      if let ErrorKind::NotFound = e.kind() {
        let _ = app_handle.emit_all(
          "toast_msg",
          ToastPayload {
            toast: format!("'{:?}' not found in PATH!", command.get_program()),
            level: "error".to_string(),
          },
        );
      }
      return Err(CommandError::BinaryExecution(
        "Unable to launch REPL".to_owned(),
      ));
    }
  }
}

fn generate_launch_game_string(
  config_info: &CommonConfigData,
  game_name: String,
  in_debug: bool,
  quote_project_path: bool,
) -> Result<Vec<String>, CommandError> {
  let data_folder = get_data_dir(&config_info, &game_name, false)?;

  let proj_path = if quote_project_path {
    format!("\"{}\"", data_folder.to_string_lossy().into_owned())
  } else {
    data_folder.to_string_lossy().into_owned()
  };
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
      proj_path,
    ];
    if in_debug {
      args.push("-debug".to_string());
    }
  } else {
    args = vec!["-v".to_string(), "--proj-path".to_string(), proj_path];
    // Add new --game argument
    if config_info.tooling_version.minor > 1 || config_info.tooling_version.patch >= 44 {
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
  let args = generate_launch_game_string(&config_info, game_name, false, true)?;

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
  executable_location: Option<String>,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = common_prelude(&config_lock)?;

  let mut exec_info = get_exec_location(&config_info, "gk")?;
  if let Some(custom_exec_location) = executable_location {
    match PathBuf::from_str(custom_exec_location.as_str()) {
      Ok(exec_path) => {
        let path_copy = exec_path.clone();
        if path_copy.parent().is_none() {
          return Err(CommandError::BinaryExecution(format!(
            "Failed to resolve custom binary parent directory"
          )));
        }
        exec_info = ExecutableLocation {
          executable_dir: exec_path.clone().parent().unwrap().to_path_buf(),
          executable_path: exec_path.clone(),
        };
      }
      Err(err) => {
        return Err(CommandError::BinaryExecution(format!(
          "Failed to resolve custom binary location {}",
          err
        )));
      }
    }
  }

  let args = generate_launch_game_string(&config_info, game_name.clone(), in_debug, false)?;

  log::info!(
    "Launching game version {:?} -> {:?} with args: {:?}. Working Directory: {:?}, Path: {:?}",
    &config_info.active_version,
    &config_info.tooling_version,
    args,
    exec_info.executable_dir,
    exec_info.executable_path,
  );

  let log_file = create_std_log_file(&app_handle, format!("game-{game_name}.log"), false)?;

  let mut command = std::process::Command::new(exec_info.executable_path);
  command
    .args(args)
    .stdout(log_file.try_clone().unwrap())
    .stderr(log_file)
    .current_dir(exec_info.executable_dir);
  #[cfg(windows)]
  {
    std::os::windows::process::CommandExt::creation_flags(&mut command, 0x08000000);
  }
  // Start the process here so if there is an error, we can return immediately
  let mut child = command.spawn()?;
  // if all goes well, we await the child to exit in the background (separate thread)
  tokio::spawn(async move {
    let start_time = Instant::now(); // get the start time of the game
                                     // start waiting for the game to exit
    match child.wait() {
      Ok(status_code) => {
        if !status_code.code().is_some() || status_code.code().unwrap() != 0 {
          let _ = app_handle.emit_all(
            "toast_msg",
            ToastPayload {
              toast: "Game crashed unexpectedly!".to_string(),
              level: "error".to_string(),
            },
          );
        }
      }
      Err(err) => {
        log::error!("Error occured when waiting for game to exit: {}", err);
        return;
      }
    }
    // once the game exits pass the time the game started to the track_playtine function
    if let Err(err) = track_playtime(start_time, game_name).await {
      log::error!("Error occured when tracking playtime: {}", err);
      return;
    }
  });
  Ok(())
}

async fn track_playtime(
  start_time: std::time::Instant,
  game_name: String,
) -> Result<(), CommandError> {
  let app_handle = TAURI_APP
    .get()
    .ok_or_else(|| {
      CommandError::BinaryExecution("Cannot access global app state to persist playtime".to_owned())
    })?
    .app_handle();
  let config = app_handle.state::<tokio::sync::Mutex<LauncherConfig>>();
  let mut config_lock = config.lock().await;

  // get the playtime of the session
  let elapsed_time = start_time.elapsed().as_secs().into();
  log::info!("elapsed time: {}", elapsed_time);

  config_lock
    .update_setting_value("seconds_played", elapsed_time, Some(game_name))
    .map_err(|_| CommandError::Configuration("Unable to persist time played".to_owned()))?;

  // send an event to the front end so that it can refresh the playtime on screen
  if let Err(err) = app_handle.emit_all("playtimeUpdated", ()) {
    log::error!("Failed to emit playtimeUpdated event: {}", err);
    return Err(CommandError::BinaryExecution(format!(
      "Failed to emit playtimeUpdated event: {}",
      err
    )));
  }

  Ok(())
}
