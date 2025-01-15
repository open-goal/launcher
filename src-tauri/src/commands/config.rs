use super::{util::is_avx_supported, CommandError};
use crate::config::LauncherConfig;
use semver::Version;
use serde_json::{json, Value};

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

#[tauri::command]
pub async fn update_setting_value(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  key: String,
  val: Value,
  game_name: Option<String>,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  match &config_lock.update_setting_value(&key, val, game_name) {
    Ok(()) => Ok(()),
    Err(e) => {
      log::error!("Unable to get setting directory: {:?}", e);
      Err(CommandError::Configuration(
        "Unable to update setting".to_owned(),
      ))
    }
  }
}

#[tauri::command]
pub async fn get_setting_value(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  key: String,
  game_name: Option<String>,
) -> Result<Value, CommandError> {
  let config_lock = config.lock().await;
  match &config_lock.get_setting_value(&key, game_name) {
    Ok(value) => Ok(json!(value)),
    Err(e) => {
      log::error!("Unable to get setting directory: {:?}", e);
      Err(CommandError::Configuration(
        "Unable to get setting".to_owned(),
      ))
    }
  }
}

#[tauri::command]
pub async fn update_mods_setting_value(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  key: String,
  game_name: String,
  source_name: Option<String>,
  version_name: Option<String>,
  mod_name: Option<String>,
  texture_packs: Option<Vec<String>>,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  match &config_lock.update_mods_setting_value(
    &key,
    game_name,
    source_name,
    version_name,
    mod_name,
    texture_packs,
  ) {
    Ok(()) => Ok(()),
    Err(e) => {
      log::error!("Unable to get setting directory: {:?}", e);
      Err(CommandError::Configuration(
        "Unable to update setting".to_owned(),
      ))
    }
  }
}

#[tauri::command]
pub async fn set_install_directory(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  new_dir: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.set_install_directory(new_dir).map_err(|err| {
    log::error!("Unable to persist installation directory: {:?}", err);
    CommandError::Configuration("Unable to persist installation directory".to_owned())
  })
}

#[tauri::command]
pub async fn is_avx_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<bool, CommandError> {
  let mut config_lock = config.lock().await;
  if config_lock.requirements.bypass_requirements {
    log::warn!("Bypassing the AVX requirements check!");
    Ok(true)
  } else {
    let _ = config_lock.update_setting_value("avx", is_avx_supported().await.into(), None);
    Ok(config_lock.requirements.avx)
  }
}

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
    log::warn!("Bypassing the OpenGL requirements check!");
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
    log::warn!(
      "We no longer check for OpenGL support via heuristics, assuming they meet the requirement"
    );
    return Ok(true);
  }
  // Do it the new way!
  log::info!("Checking for OpenGL support via `gk`");
  let test_result = crate::util::game_tests::run_game_gpu_test(&config_lock, &app_handle).await?;
  config_lock
    .update_setting_value("opengl_requirements_met", test_result.success.into(), None)
    .map_err(|_| {
      CommandError::Configuration("Unable to persist opengl requirement change".to_owned())
    })?;
  Ok(test_result.success)
}

#[tauri::command]
pub async fn cleanup_enabled_texture_packs(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
  cleanup_list: Vec<String>,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .cleanup_game_enabled_texture_packs(&game_name, cleanup_list)
    .map_err(|_| {
      CommandError::Configuration("Unable to cleanup enabled texture packs".to_owned())
    })?;
  Ok(())
}

#[tauri::command]
pub async fn does_active_tooling_version_support_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  match &config_lock.active_version {
    Some(version) => {
      // If we can't determine the version, assume its our first release
      let tooling_version = Version::parse(version.strip_prefix('v').unwrap_or(version))
        .unwrap_or(Version::new(0, 0, 1));
      match game_name.as_str() {
        "jak1" => Ok(true),
        "jak2" => Ok(tooling_version.minor > 1 || tooling_version.patch >= 44),
        _ => Ok(false),
      }
    }
    None => {
      log::warn!("No active tooling version set, can't check the game supports it!");
      Ok(false)
    }
  }
}

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
    log::warn!("No active tooling version set, can't check if the minimum!");
    Ok(false)
  }
}
