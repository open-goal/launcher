use anyhow::Context;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
  collections::HashMap,
  path::{Path, PathBuf},
  process::Stdio,
  time::Instant,
};
use tokio::process::Command;

use log::{error, info, warn};
use semver::Version;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager};

use crate::{
  TAURI_APP,
  config::{CommonConfigData, ExecutableLocation, LauncherConfig, SupportedGame},
  util::{
    file::overwrite_dir,
    process::{create_log_file, create_std_log_file, watch_process},
  },
};

use super::CommandError;

#[derive(Debug, Serialize, Deserialize)]
struct LauncherErrorCode {
  msg: String,
}

#[cfg(windows)]
pub fn format_exit_code(code: i32) -> String {
  format!("{code} ({:#010X})", code as u32)
}

#[cfg(not(windows))]
pub fn format_exit_code(code: i32) -> String {
  code.to_string()
}

fn get_error_code_message(
  config: &CommonConfigData,
  game_name: SupportedGame,
  code: i32,
) -> String {
  let json_file = config
    .install_path
    .join("active")
    .join(game_name.to_string())
    .join("data")
    .join("launcher")
    .join("error-code-metadata.json");

  std::fs::File::open(&json_file)
    .inspect_err(|e| warn!("{}", e))
    .ok()
    .and_then(|file| {
      serde_json::from_reader::<_, HashMap<i32, LauncherErrorCode>>(file)
        .inspect_err(|e| warn!("{}", e))
        .ok()
    })
    .and_then(|map| map.get(&code).map(|e| e.msg.clone()))
    .unwrap_or_else(|| {
      format!(
        "Unexpected error occurred with code {}",
        format_exit_code(code)
      )
    })
}

fn copy_data_dir(
  config_info: &CommonConfigData,
  game_name: SupportedGame,
) -> Result<(), CommandError> {
  let src_dir = config_info
    .install_path
    .join("versions")
    .join("official")
    .join(&config_info.active_version)
    .join("data");

  let dst_dir = config_info
    .install_path
    .join("active")
    .join(game_name.to_string())
    .join("data");

  info!("Copying {} into {}", src_dir.display(), dst_dir.display());

  overwrite_dir(&src_dir, &dst_dir)?;
  Ok(())
}

fn get_data_dir(
  config_info: &CommonConfigData,
  game_name: SupportedGame,
  copy_directory: bool,
) -> Result<PathBuf, CommandError> {
  let data_folder = config_info
    .install_path
    .join("active")
    .join(game_name.to_string())
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

#[tauri::command]
pub async fn update_data_directory(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;
  copy_data_dir(&config_info, game_name)?;
  Ok(())
}

#[tauri::command]
pub async fn extract_and_validate_iso(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: String,
  game_name: SupportedGame,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;
  let data_folder = get_data_dir(&config_info, game_name, true)?;
  let exec_info = config_info.get_exec_location("extractor")?;

  log::info!(
    "extracting using data folder: {}",
    data_folder.to_string_lossy()
  );

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
  if config_info.tooling_version >= Version::new(0, 1, 44) {
    args.push("--game".to_string());
    args.push(game_name.to_string());
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

  let status = watch_process(&mut log_file, &mut child, &app_handle).await?;
  if status.success() {
    log::info!("extraction and validation was successful");
    return Ok(());
  }

  if let Some(code) = status.code() {
    let message = get_error_code_message(&config_info, game_name, code);
    error!("extraction and validation was not successful. Code {code}");
    return Err(CommandError::BinaryExecution(message));
  }

  #[cfg(unix)]
  if let Some(signal) = status.signal() {
    error!("Extractor process terminated by signal {signal:?}");
    return Err(CommandError::BinaryExecution(format!(
      "Extractor process terminated by signal {signal:?}",
    )));
  }

  error!("extraction and validation was not successful. No status code");
  return Err(CommandError::BinaryExecution(
    "Unexpected error occurred".to_owned(),
  ));
}

#[tauri::command]
pub async fn run_decompiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: Option<String>,
  game_name: SupportedGame,
  truncate_logs: bool,
  use_decomp_settings: bool,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;
  let data_folder = get_data_dir(&config_info, game_name, false)?;
  let exec_info = config_info.get_exec_location("extractor")?;

  log::info!(
    "decompiling using data folder: {}",
    data_folder.to_string_lossy()
  );

  let source_path = path_to_iso.unwrap_or_else(|| {
    data_folder
      .join("iso_data")
      .join(game_name.to_string())
      .to_string_lossy()
      .into_owned()
  });

  let mut command = Command::new(exec_info.executable_path);

  let mut args = vec![
    source_path,
    "--decompile".to_string(),
    "--proj-path".to_string(),
    data_folder.to_string_lossy().into_owned(),
  ];

  // Add new --game argument
  if config_info.tooling_version >= Version::new(0, 1, 44) {
    args.push("--game".to_string());
    args.push(game_name.to_string());
  }

  if use_decomp_settings {
    let settings = &config_lock.decompiler_settings;
    let mut overrides = serde_json::Map::new();

    for (key, enabled) in [
      ("rip_levels", settings.rip_levels_enabled),
      ("rip_collision", settings.rip_collision_enabled),
      ("save_texture_pngs", settings.rip_textures_enabled),
      ("rip_streamed_audio", settings.rip_streamed_audio_enabled),
    ] {
      if enabled {
        overrides.insert(key.to_string(), true.into());
      }
    }

    if !overrides.is_empty() {
      args.push("--decomp-config-override".to_string());
      args.push(serde_json::Value::Object(overrides).to_string());
    }
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

  let status = watch_process(&mut log_file, &mut child, &app_handle).await?;
  if status.success() {
    log::info!("decompilation was successful");
    return Ok(());
  }

  if let Some(code) = status.code() {
    let message = get_error_code_message(&config_info, game_name, code);
    error!("decompilation was not successful. Code {code}");
    return Err(CommandError::BinaryExecution(message));
  }

  #[cfg(unix)]
  if let Some(signal) = status.signal() {
    error!("Decompiler process terminated by signal {signal:?}");
    return Err(CommandError::BinaryExecution(format!(
      "Decompiler process terminated by signal {signal:?}",
    )));
  }

  error!("decompilation was not successful. No status code.");
  return Err(CommandError::BinaryExecution(
    "Unexpected error occurred".to_owned(),
  ));
}

#[tauri::command]
pub async fn run_compiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: Option<String>,
  game_name: SupportedGame,
  truncate_logs: bool,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;
  let exec_info = config_info.get_exec_location("extractor")?;
  let data_folder = get_data_dir(&config_info, game_name, false)?;

  log::info!(
    "compiling using data folder: {}",
    data_folder.to_string_lossy()
  );

  let source_path = path_to_iso.unwrap_or_else(|| {
    data_folder
      .join("iso_data")
      .join(game_name.to_string())
      .to_string_lossy()
      .into_owned()
  });

  let mut args = vec![
    source_path,
    "--compile".to_string(),
    "--proj-path".to_string(),
    data_folder.to_string_lossy().into_owned(),
  ];
  // Add new --game argument
  if config_info.tooling_version >= Version::new(0, 1, 44) {
    args.push("--game".to_string());
    args.push(game_name.to_string());
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

  let status = watch_process(&mut log_file, &mut child, &app_handle).await?;
  if status.success() {
    log::info!("compilation was successful");
    return Ok(());
  }

  if let Some(code) = status.code() {
    let message = get_error_code_message(&config_info, game_name, code);
    error!("compilation was not successful. Code {code}");
    return Err(anyhow::anyhow!(message).into());
  }

  #[cfg(unix)]
  if let Some(signal) = status.signal() {
    error!("Compiler process terminated by signal {signal:?}");
    return Err(CommandError::BinaryExecution(format!(
      "Compiler process terminated by signal {signal:?}",
    )));
  }

  error!("compilation was not successful. No status code");
  return Err(anyhow::anyhow!("compilation was not successful. No status code").into());
}

#[tauri::command]
pub async fn open_repl(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;
  let data_folder = get_data_dir(&config_info, game_name, false)?;
  let exec_info = config_info.get_exec_location("goalc")?;
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

  command.spawn().context("Unable to launch REPL")?;
  Ok(())
}

fn generate_launch_game_string(
  config_info: &CommonConfigData,
  game_name: SupportedGame,
  in_debug: bool,
  quote_project_path: bool,
) -> Result<Vec<String>, CommandError> {
  let data_folder = get_data_dir(config_info, game_name, false)?;

  let proj_path = if quote_project_path {
    format!("\"{}\"", data_folder.to_string_lossy().into_owned())
  } else {
    data_folder.to_string_lossy().into_owned()
  };
  let mut args;
  // NOTE - we do this check because the order of arguments matters for gk
  if config_info.tooling_version < Version::new(0, 1, 35) {
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
    if config_info.tooling_version >= Version::new(0, 1, 44) {
      args.push("--game".to_string());
      args.push(game_name.to_string());
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
  game_name: SupportedGame,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;
  let exec_info = config_info.get_exec_location("gk")?;
  let args = generate_launch_game_string(&config_info, game_name, false, true)?;

  Ok(format!(
    "{} {}",
    format!("\"{}\"", exec_info.executable_path.display()),
    args.join(" ")
  ))
}

#[tauri::command]
pub async fn launch_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  in_debug: bool,
  executable_location: Option<PathBuf>,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;

  let exec_info = if let Some(exec_path) = executable_location {
    ExecutableLocation {
      executable_dir: exec_path.parent().unwrap().to_path_buf(),
      executable_path: exec_path,
    }
  } else {
    config_info.get_exec_location("gk")?
  };

  let args = generate_launch_game_string(&config_info, game_name, in_debug, false)?;

  log::info!(
    "Launching game version {:?} -> {:?} with args: {:?}. Working Directory: {:?}, Path: {:?}",
    &config_info.active_version,
    &config_info.tooling_version,
    args,
    exec_info.executable_dir,
    exec_info.executable_path,
  );

  let log_file = create_std_log_file(&app_handle, format!("game-{game_name}.log"), false)?;
  let log_file_err = log_file.try_clone()?;

  let mut command = tokio::process::Command::new(exec_info.executable_path);
  command
    .args(args)
    .stdout(log_file)
    .stderr(log_file_err)
    .current_dir(exec_info.executable_dir);

  #[cfg(windows)]
  {
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    command.creation_flags(CREATE_NO_WINDOW);
  }
  drop(config_lock);
  let mut child = command.spawn()?;
  let handle = tokio::spawn(async move {
    let start_time = Instant::now();
    let status = match child.wait().await {
      Ok(status) => status,
      Err(err) => {
        error!("Error occurred when waiting for game to exit: {err}");
        anyhow::bail!("Error occurred when waiting for game to exit: {err}");
      }
    };

    track_playtime(start_time, game_name)
      .await
      .map_err(|err| anyhow::anyhow!("Failed to track playtime: {err}"))?;

    if let Some(exit_code) = status.code()
      && exit_code != 0
    {
      error!("Game crashed with code: {}", format_exit_code(exit_code));
      anyhow::bail!("Game crashed with code: {}", format_exit_code(exit_code));
    }
    Ok(())
  });

  handle.await.map_err(|e| anyhow::anyhow!("{:#?}", e))??;
  Ok(())
}

async fn track_playtime(
  start_time: std::time::Instant,
  game_name: SupportedGame,
) -> anyhow::Result<()> {
  let app_handle = TAURI_APP
    .get()
    .expect("Can't access global app state")
    .app_handle();

  let elapsed_seconds = start_time.elapsed().as_secs().into();

  let config = app_handle.state::<tokio::sync::Mutex<LauncherConfig>>();
  let mut config_lock = config.lock().await;
  config_lock.update_setting_value("seconds_played", elapsed_seconds, Some(game_name))?;

  app_handle.emit("playtimeUpdated", ())?;
  Ok(())
}
