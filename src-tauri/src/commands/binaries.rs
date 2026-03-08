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
use tokio::process::Command;

use log::{info, warn};
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

#[derive(Clone, serde::Serialize)]
struct ToastPayload {
  toast: String,
  level: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LauncherErrorCode {
  msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallStepOutput {
  pub success: bool,
  pub msg: Option<String>,
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
    .unwrap_or_else(|| format!("Unexpected error occurred with code {code}"))
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
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;

  copy_data_dir(&config_info, game_name)?;

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
  game_name: SupportedGame,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;

  let data_folder = get_data_dir(&config_info, game_name, true)?;
  log::info!(
    "extracting using data folder: {}",
    data_folder.to_string_lossy()
  );
  let exec_info = match config_info.get_exec_location("extractor") {
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
    return Ok(InstallStepOutput {
      success: true,
      msg: None,
    });
  }

  if let Some(code) = status.code() {
    let message = get_error_code_message(&config_info, game_name, code);
    log::error!("extraction and validation was not successful. Code {code}");
    return Ok(InstallStepOutput {
      success: false,
      msg: Some(message),
    });
  }

  log::error!("extraction and validation was not successful. No status code");
  Ok(InstallStepOutput {
    success: false,
    msg: Some("Unexpected error occurred".to_owned()),
  })
}

#[tauri::command]
pub async fn run_decompiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: Option<String>,
  game_name: SupportedGame,
  truncate_logs: bool,
  use_decomp_settings: bool,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;

  let data_folder = get_data_dir(&config_info, game_name, false)?;
  log::info!(
    "decompiling using data folder: {}",
    data_folder.to_string_lossy()
  );
  let exec_info = match config_info.get_exec_location("extractor") {
    Ok(exec_info) => exec_info,
    Err(_) => {
      log::error!("extractor executable not found");
      return Ok(InstallStepOutput {
        success: false,
        msg: Some("Tooling appears to be missing critical files. This may be caused by antivirus software. You will need to redownload the version and try again.".to_string()),
      });
    }
  };

  let source_path = path_to_iso.unwrap_or_else(|| {
    data_folder
      .join("iso_data")
      .join(game_name.to_string())
      .to_string_lossy()
      .into_owned()
  });

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
  if config_info.tooling_version >= Version::new(0, 1, 44) {
    args.push("--game".to_string());
    args.push(game_name.to_string());
  }

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

  let status = watch_process(&mut log_file, &mut child, &app_handle).await?;
  if status.success() {
    log::info!("decompilation was successful");
    return Ok(InstallStepOutput {
      success: true,
      msg: None,
    });
  }

  if let Some(code) = status.code() {
    let message = get_error_code_message(&config_info, game_name, code);
    log::error!("decompilation was not successful. Code {code}");
    return Ok(InstallStepOutput {
      success: false,
      msg: Some(message),
    });
  }

  log::error!("decompilation was not successful. No status code");
  Ok(InstallStepOutput {
    success: false,
    msg: Some("Unexpected error occurred".to_owned()),
  })
}

#[tauri::command]
pub async fn run_compiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  path_to_iso: Option<String>,
  game_name: SupportedGame,
  truncate_logs: bool,
) -> Result<InstallStepOutput, CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;

  let data_folder = get_data_dir(&config_info, game_name, false)?;
  log::info!(
    "compiling using data folder: {}",
    data_folder.to_string_lossy()
  );
  let exec_info = match config_info.get_exec_location("extractor") {
    Ok(exec_info) => exec_info,
    Err(_) => {
      return Ok(InstallStepOutput {
        success: false,
        msg: Some("Tooling appears to be missing critical files. This may be caused by antivirus software. You will need to redownload the version and try again.".to_string()),
      })
    }
  };

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
    return Ok(InstallStepOutput {
      success: true,
      msg: None,
    });
  }

  if let Some(code) = status.code() {
    let message = get_error_code_message(&config_info, game_name, code);
    log::error!("compilation was not successful. Code {code}");
    return Ok(InstallStepOutput {
      success: false,
      msg: Some(message),
    });
  }

  log::error!("compilation was not successful. No status code");
  Ok(InstallStepOutput {
    success: false,
    msg: Some("Unexpected error occurred".to_owned()),
  })
}

#[tauri::command]
pub async fn open_repl(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
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
    exec_info.executable_path.display(),
    args.join(" ")
  ))
}

#[tauri::command]
pub async fn launch_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  in_debug: bool,
  executable_location: Option<String>,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let config_info = config_lock.common_prelude()?;

  let mut exec_info = config_info.get_exec_location("gk")?;
  if let Some(custom_exec_location) = executable_location {
    match PathBuf::from_str(custom_exec_location.as_str()) {
      Ok(exec_path) => {
        let path_copy = exec_path.clone();
        if path_copy.parent().is_none() {
          return Err(CommandError::BinaryExecution(
            "Failed to resolve custom binary parent directory".to_string(),
          ));
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

  let mut command = std::process::Command::new(exec_info.executable_path);
  command
    .args(args)
    .stdout(log_file)
    .stderr(log_file_err)
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
        if status_code.code().is_none() || status_code.code().unwrap() != 0 {
          let _ = app_handle.emit(
            "toast_msg",
            ToastPayload {
              toast: "Game crashed unexpectedly!".to_string(),
              level: "error".to_string(),
            },
          );
        }
      }
      Err(err) => {
        log::error!("Error occurred when waiting for game to exit: {}", err);
        return;
      }
    }
    // once the game exits pass the time the game started to the track_playtine function
    if let Err(err) = track_playtime(start_time, game_name).await {
      log::error!("Failed to track playtime: {err}");
    }
  });
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
