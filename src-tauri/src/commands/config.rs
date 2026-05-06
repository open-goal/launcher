use std::path::PathBuf;

use super::{CommandError, util::is_avx_supported};
use crate::config::{LauncherConfig, SupportedGame};
use semver::Version;
use tauri::Emitter;
use tracing::instrument;

#[instrument(skip(config))]
#[tauri::command]
pub async fn reset_to_defaults(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.reset_to_defaults().map_err(|_| {
    CommandError::Configuration("Unable to reset configuration to defaults".to_owned())
  })?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_game_installed(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  installed: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.set_game_installed(game_name, installed)?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_active_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.set_active_version(Some(version))?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn get_launcher_config(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<LauncherConfig, CommandError> {
  Ok(config.lock().await.clone())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_texture_packs(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  texture_packs: Vec<String>,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .set_texture_packs(game_name, texture_packs)
    .map_err(|err| CommandError::Configuration(format!("Unable to set texture packs: {}", err)))
}

#[instrument(skip(config, app_handle))]
#[tauri::command]
pub async fn set_install_directory(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  new_dir: PathBuf,
  app_handle: tauri::AppHandle,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.set_install_directory(new_dir).map_err(|err| {
    tracing::error!("Unable to persist installation directory: {:?}", err);
    CommandError::Configuration("Unable to persist installation directory".to_owned())
  })?;
  app_handle.emit("config:saved", ())?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn is_avx_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<bool, CommandError> {
  let mut config_lock = config.lock().await;
  if config_lock.requirements.bypass_requirements {
    tracing::warn!("Bypassing the AVX requirements check!");
    Ok(true)
  } else {
    let avx_supported = is_avx_supported().await;
    config_lock.requirements.set_avx(avx_supported);
    Ok(avx_supported)
  }
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_bypass_requirements(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  bypass: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.requirements.set_bypass_requirements(bypass);
  config_lock.save_config()?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_locale(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  locale: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.set_locale(locale)?;
  config_lock.save_config()?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn update_mod_sources(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  source: String,
  add: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.update_mod_sources(source, add)?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_auto_update_games(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  auto_update: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.set_auto_update_games(auto_update)?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_check_for_latest_mod_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  check: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.set_check_for_latest_mod_version(check)?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_delete_previous_versions(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  delete: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.set_delete_previous_versions(delete)?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_rip_levels(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  enabled: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .decompiler_settings
    .set_rip_levels_enabled(enabled);
  config_lock.save_config()?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_rip_collision(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  enabled: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .decompiler_settings
    .set_rip_collision_enabled(enabled);
  config_lock.save_config()?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_rip_textures(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  enabled: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .decompiler_settings
    .set_rip_textures_enabled(enabled);
  config_lock.save_config()?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn set_rip_streamed_audio(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  enabled: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .decompiler_settings
    .set_rip_streamed_audio_enabled(enabled);
  config_lock.save_config()?;
  Ok(())
}

#[instrument(skip(config, app_handle))]
#[tauri::command]
pub async fn is_opengl_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  force: bool,
) -> Result<bool, CommandError> {
  let mut config_lock = config.lock().await;
  if force {
    config_lock.requirements.opengl = false;
  }
  if config_lock.requirements.bypass_requirements {
    tracing::warn!("Bypassing the OpenGL requirements check!");
    return Ok(true);
  }
  // If the value is already set, just return it
  if config_lock.requirements.opengl {
    return Ok(config_lock.requirements.opengl);
  }

  // Check the active tooling version, if it's above 0.1.38 we can use the new
  // built-in gpu testing feature
  // If not, we have to use the spotty heuristic
  let active_version = config_lock
    .active_version
    .as_ref()
    .ok_or(CommandError::Configuration(
      "No active version set, can't perform operation".to_owned(),
    ))?;
  // If we can't determine the version, assume it's too old
  let tooling_version = Version::parse(active_version.strip_prefix('v').unwrap_or(active_version))
    .unwrap_or(Version::new(0, 1, 37));
  if tooling_version.major == 0 && tooling_version.minor <= 1 && tooling_version.patch < 38 {
    // Assume it's fine
    tracing::warn!(
      "We no longer check for OpenGL support via heuristics, assuming they meet the requirement"
    );
    return Ok(true);
  }
  // Do it the new way!
  tracing::info!("Checking for OpenGL support via `gk`");
  // Note: if the minimum vcc runtime requirement (windows only) isn't met then run_game_gpu_test will fail right here with a non zero error code
  // what looks like a GPU test failure is actually just the game not launching
  let test_result = crate::util::game_tests::run_game_gpu_test(&config_lock, &app_handle).await?;
  config_lock.requirements.set_opengl(test_result.success);
  Ok(test_result.success)
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn cleanup_enabled_texture_packs(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  cleanup_list: Vec<String>,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .cleanup_game_enabled_texture_packs(game_name, cleanup_list)
    .map_err(|_| {
      CommandError::Configuration("Unable to cleanup enabled texture packs".to_owned())
    })?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn does_active_tooling_version_support_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  let version_str = if let Some(v) = &config_lock.active_version {
    v.strip_prefix('v').unwrap_or(v)
  } else {
    tracing::warn!("No active tooling version set, can't check the game supports it!");
    return Ok(false);
  };

  let tooling_version = Version::parse(version_str).unwrap_or_else(|_| Version::new(0, 0, 1));
  let supported = match game_name {
    SupportedGame::Jak1 => true,
    SupportedGame::Jak2 => tooling_version.minor >= 2,
    SupportedGame::Jak3 => tooling_version.minor >= 3,
    SupportedGame::JakX => false,
  };

  return Ok(supported);
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn does_active_tooling_version_meet_minimum(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  minimum_patch: u64,
  minimum_minor: u64,
  minimum_major: u64,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  if let Some(version) = &config_lock.active_version {
    let tooling_version = Version::parse(version.strip_prefix('v').unwrap_or(version))
      .unwrap_or_else(|_| Version::new(0, 0, 0));
    let compare_version = Version::new(minimum_major, minimum_minor, minimum_patch);
    Ok(tooling_version >= compare_version)
  } else {
    tracing::warn!("No active tooling version set, can't check if the minimum!");
    Ok(false)
  }
}
