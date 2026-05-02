#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
  fs,
  io::ErrorKind,
  path::{Path, PathBuf},
  process::Stdio,
};

use anyhow::Context;
use tauri::Emitter;
use tokio::process::Command;
use tracing::instrument;

use crate::{
  cache::{ModCache, ModInfo},
  commands::{CommandError, binaries::format_exit_code},
  config::{ExecutableLocation, LauncherConfig, SupportedGame},
  util::{
    file::{delete_dir, to_image_base64},
    network::download_file,
    process::{create_log_file, create_std_log_file, watch_process},
    tar::{extract_and_delete_archive, extract_archive},
  },
};

#[instrument(skip(config))]
#[tauri::command]
pub async fn extract_new_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  bundle_path: PathBuf,
  mod_source: String,
) -> Result<(), CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
  };

  // The name of the zip becomes the folder, if one already exists it will be deleted!
  let mod_name = bundle_path
    .file_stem()
    .and_then(|stem| stem.to_str())
    .map(|s| s.strip_suffix(".tar").unwrap_or(s))
    .ok_or_else(|| anyhow::anyhow!("Unable to get mod name from archive path"))?;

  let destination_dir = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(mod_source)
    .join(&mod_name);

  delete_dir(&destination_dir)?;
  extract_archive(&bundle_path, &destination_dir)?;

  Ok(())
}

#[instrument(skip(config, cache))]
#[tauri::command]
pub async fn download_and_extract_new_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  cache: tauri::State<'_, tokio::sync::Mutex<ModCache>>,
  game_name: SupportedGame,
  download_url: String,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
  };

  // Download the file
  let destination_dir = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(&source_name)
    .join(&mod_name);

  let filename = download_url
    .rsplit('/')
    .next()
    .filter(|s| !s.is_empty())
    .context("Invalid URL: missing filename")?;

  let download_path = &destination_dir.join(filename);

  delete_dir(&destination_dir)?;
  download_file(&download_url, &download_path).await?;
  extract_and_delete_archive(&download_path, &destination_dir, false)?;

  // Persist the info about the mod to the disk in the event that the mod source is removed / etc
  let mod_info = {
    let cache_lock = cache.lock().await;
    cache_lock
      .mod_sources
      .iter()
      .find(|(_, data)| data.source_name == source_name)
      .and_then(|(_, source)| source.mods.get(&mod_name))
      .cloned()
      .ok_or_else(|| anyhow::anyhow!("Unable to find mod {} in source {}", mod_name, source_name))?
  };

  let metadata_path = destination_dir.join("_metadata.json");
  let file = fs::File::create(&metadata_path)?;

  tracing::info!("saving mod info to: {}", &metadata_path.display());
  serde_json::to_writer_pretty(file, &mod_info)
    .map_err(|e| anyhow::anyhow!("Unable to save mod metadata: {}", e))?;

  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn get_locally_persisted_mod_info(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<ModInfo, CommandError> {
  if source_name == "_local" {
    return Ok(ModInfo {
      name: mod_name.clone(),
      display_name: mod_name,
      source: source_name,
      installed: true,
      supported_games: vec![game_name],
      tags: vec!["local".to_string()],
      ..Default::default()
    });
  }

  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
  };

  let metadata_path = &install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(&source_name)
    .join(&mod_name)
    .join("_metadata.json");

  let file = fs::File::open(metadata_path).map_err(|e| {
    anyhow::anyhow!(
      "Unable to open local mod metadata at {}: {}",
      metadata_path.display(),
      e
    )
  })?;
  let mod_info = serde_json::from_reader(file)
    .map_err(|e| anyhow::anyhow!("Unable to deserialize local mod metadata: {}", e))?;
  return Ok(mod_info);
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn base_game_iso_exists(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
) -> Result<bool, CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
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
  install_path: &Path,
  executable_name: &str,
  game_name: SupportedGame,
  mod_name: &str,
  source_name: &str,
) -> ExecutableLocation {
  let exec_dir = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(source_name)
    .join(mod_name);

  #[cfg(windows)]
  let exec_path = exec_dir.join(executable_name).with_extension("exe");

  #[cfg(not(windows))]
  let exec_path = exec_dir.join(executable_name);

  ExecutableLocation {
    executable_dir: exec_dir,
    executable_path: exec_path,
  }
}

#[instrument(skip(config, app_handle))]
#[tauri::command]
pub async fn extract_iso_for_mod_install(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
  path_to_iso: String,
) -> Result<(), CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
  };

  let exec_info = get_mod_exec_location(
    &install_path,
    "extractor",
    game_name,
    &mod_name,
    &source_name,
  );

  let iso_extraction_dir = install_path
    .join("active")
    .join(game_name.to_string())
    .join("data")
    .join("iso_data");

  let args = vec![
    path_to_iso,
    "--extract".to_string(),
    "--validate".to_string(),
    "--extract-path".to_string(),
    iso_extraction_dir.to_string_lossy().into_owned(),
    "--game".to_string(),
    game_name.to_string(),
  ];

  tracing::info!("Running extractor with args: {:?}", args);

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
  let mut child = command.spawn().context("Failed to spawn extractor")?;

  // This is the first install step, reset the file
  let mut log_file = create_log_file(
    &app_handle,
    format!("extractor-{game_name}-{mod_name}.log"),
    true,
  )
  .await?;

  let status = watch_process(&mut log_file, &mut child, &app_handle).await?;
  if status.success() {
    tracing::info!("extraction and validation was successful");
    return Ok(());
  }

  let msg = status
    .code()
    .map(|code| format_exit_code(code))
    .map(|code| format!("Unexpected error occurred with code {code}"))
    .unwrap_or_else(|| "Unexpected error occurred".to_owned());

  return Err(CommandError::GameFeatures(msg));
}

#[instrument(skip(config, app_handle))]
#[tauri::command]
pub async fn decompile_for_mod_install(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
  };

  let exec_info = get_mod_exec_location(
    &install_path,
    "extractor",
    game_name,
    &mod_name,
    &source_name,
  );

  let iso_dir = install_path
    .join("active")
    .join(game_name.to_string())
    .join("data")
    .join("iso_data")
    .join(game_name.to_string());

  let args = vec![
    iso_dir.clone().to_string_lossy().into_owned(),
    "--folder".to_string(),
    "--decompile".to_string(),
    "--game".to_string(),
    game_name.to_string(),
  ];

  tracing::info!("Running extractor with args: {:?}", args);

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
  let mut child = command.spawn().context("Failed to spawn decompiler")?;

  let mut log_file =
    create_log_file(&app_handle, format!("extractor-{game_name}.log"), false).await?;

  let status = watch_process(&mut log_file, &mut child, &app_handle).await?;
  if status.success() {
    tracing::info!("decompilation was successful");
    return Ok(());
  }

  let msg = status
    .code()
    .map(|code| format_exit_code(code))
    .map(|code| format!("Unexpected error occurred with code {code}"))
    .unwrap_or_else(|| "Unexpected error occurred".to_owned());

  return Err(CommandError::GameFeatures(msg));
}

#[instrument(skip(config, app_handle))]
#[tauri::command]
pub async fn compile_for_mod_install(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
  };
  let exec_info = get_mod_exec_location(
    &install_path,
    "extractor",
    game_name,
    &mod_name,
    &source_name,
  );

  let iso_dir = install_path
    .join("active")
    .join(game_name.to_string())
    .join("data")
    .join("iso_data")
    .join(game_name.to_string());

  let args = vec![
    iso_dir.clone().to_string_lossy().into_owned(),
    "--folder".to_string(),
    "--compile".to_string(),
    "--game".to_string(),
    game_name.to_string(),
  ];

  tracing::info!("Running extractor with args: {:?}", args);

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
  let mut child = command.spawn().context("Failed to spawn compiler")?;

  let mut log_file =
    create_log_file(&app_handle, format!("extractor-{game_name}.log"), false).await?;

  let status = watch_process(&mut log_file, &mut child, &app_handle).await?;
  if status.success() {
    tracing::info!("compilation was successful");
    return Ok(());
  }

  let msg = status
    .code()
    .map(|code| format_exit_code(code))
    .map(|code| format!("Unexpected error occurred with code {code}"))
    .unwrap_or_else(|| "Unexpected error occurred".to_owned());

  return Err(CommandError::GameFeatures(msg));
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn save_mod_install_info(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
  version_name: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  tracing::info!(
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
      tracing::error!("Unable to remove mod source: {:?}", err);
      CommandError::Configuration("Unable to remove mod source".to_owned())
    })?;
  Ok(())
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

#[instrument(skip(config, app_handle))]
#[tauri::command]
pub async fn launch_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  in_debug: bool,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
  };
  let config_dir = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods")
    .join(&source_name)
    .join("_settings")
    .join(&mod_name);
  let exec_info = get_mod_exec_location(&install_path, "gk", game_name, &mod_name, &source_name);
  let args = generate_launch_mod_args(game_name, in_debug, config_dir, false)?;

  tracing::info!("Launching gk args: {:?}", args);

  let log_file = create_std_log_file(
    &app_handle,
    format!("game-{game_name}-{mod_name}.log"),
    false,
  )?;
  let log_file_err = log_file.try_clone()?;

  // TODO - log rotation here would be nice too
  let mut command = Command::new(exec_info.executable_path);
  command
    .args(args)
    .stdout(log_file)
    .stderr(log_file_err)
    .current_dir(exec_info.executable_dir);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  // Start the process here so if there is an error, we can return immediately
  let _child = command.spawn().context("Failed to spawn game")?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn get_local_mod_thumbnail_base64(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
) -> Result<String, CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
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

#[instrument(skip(config))]
#[tauri::command]
pub async fn uninstall_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  let install_path = config_lock.install_dir()?;
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

#[instrument(skip(config))]
#[tauri::command]
pub async fn reset_mod_settings(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
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

  if !path_to_settings.exists() {
    return Err(CommandError::GameManagement(
      "Game config directory does not exist, cannot reset settings".to_owned(),
    ));
  }

  let mut backup_file = path_to_settings.clone();
  backup_file.set_file_name("pc-settings.old.gc");
  std::fs::rename(path_to_settings, backup_file)?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn get_launch_mod_string(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<String, CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
  };
  let exec_info = get_mod_exec_location(&install_path, "gk", game_name, &mod_name, &source_name);
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
    format!("\"{}\"", exec_info.executable_path.display()),
    args.join(" ")
  ))
}

#[derive(Clone, serde::Serialize)]
struct ToastPayload {
  toast: String,
  level: String,
}

#[instrument(skip(config, app_handle))]
#[tauri::command]
pub async fn open_repl_for_mod(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
  mod_name: String,
  source_name: String,
) -> Result<(), CommandError> {
  let install_path = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
  };
  let iso_dir = install_path
    .join("active")
    .join(game_name.to_string())
    .join("data")
    .join("iso_data")
    .join(game_name.to_string());

  let exec_info = get_mod_exec_location(&install_path, "goalc", game_name, &mod_name, &source_name);
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
