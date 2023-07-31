use crate::{config::LauncherConfig, util::file::delete_dir};
use semver::Version;
use tauri::Manager;
use wgpu::InstanceDescriptor;

use super::CommandError;

#[tauri::command]
pub async fn has_old_data_directory(app_handle: tauri::AppHandle) -> Result<bool, CommandError> {
  match &app_handle.path_resolver().app_config_dir() {
    None => Ok(false),
    Some(dir) => Ok(dir.join("data").join("iso_data").exists()),
  }
}

#[tauri::command]
pub async fn delete_old_data_directory(app_handle: tauri::AppHandle) -> Result<(), CommandError> {
  match &app_handle.path_resolver().app_config_dir() {
    None => Ok(()),
    Some(dir) => Ok(delete_dir(&dir.join("data"))?),
  }
}

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
pub async fn get_install_directory(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<String>, CommandError> {
  let config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Ok(None),
    Some(dir) => Ok(Some(dir.to_string())),
  }
}

#[tauri::command]
pub async fn set_install_directory(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  new_dir: String,
) -> Result<Option<String>, CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.set_install_directory(new_dir).map_err(|err| {
    log::error!("Unable to persist installation directory: {:?}", err);
    CommandError::Configuration("Unable to persist installation directory".to_owned())
  })
}

#[tauri::command]
pub async fn is_avx_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  force: bool,
) -> Result<bool, CommandError> {
  let mut config_lock = config.lock().await;
  if force {
    config_lock.requirements.avx = None;
  }
  if let Some(bypass) = config_lock.requirements.bypass_requirements {
    if bypass {
      log::warn!("Bypassing the AVX requirements check!");
      return Ok(true);
    }
  }
  match config_lock.requirements.avx {
    None => {
      if is_x86_feature_detected!("avx") || is_x86_feature_detected!("avx2") {
        config_lock.requirements.avx = Some(true);
      } else {
        config_lock.requirements.avx = Some(false);
      }
      config_lock.save_config().map_err(|err| {
        log::error!("Unable to persist avx requirement change {}", err);
        CommandError::Configuration("Unable to persist avx requirement change".to_owned())
      })?;
      Ok(config_lock.requirements.avx.unwrap_or(false))
    }
    Some(val) => Ok(val),
  }
}

async fn check_opengl_via_heuristic(
  mut config_lock: tokio::sync::MutexGuard<'_, LauncherConfig>,
) -> Result<Option<bool>, CommandError> {
  let instance = wgpu::Instance::new(InstanceDescriptor {
    backends: wgpu::Backends::all(),
    dx12_shader_compiler: wgpu::Dx12Compiler::default(),
  });
  let adapter = match instance
    .request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      force_fallback_adapter: false,
      compatible_surface: None,
    })
    .await
  {
    None => {
      config_lock.set_opengl_requirement_met(None).map_err(|_| {
        CommandError::Configuration("Unable to persist opengl requirement change".to_owned())
      })?;
      return Err(CommandError::Configuration(
        "Unable to request GPU adapter to check for OpenGL support".to_owned(),
      ));
    }
    Some(instance) => instance,
  };

  match adapter
    .request_device(
      &wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: wgpu::Limits {
          // These are OpenGL 4.3 minimums where these values
          // were the maximum (not inclusive) for 4.2
          max_texture_dimension_1d: 16384,
          max_texture_dimension_2d: 16384,
          max_texture_dimension_3d: 2048,
          ..wgpu::Limits::default()
        },
        label: None,
      },
      None,
    )
    .await
  {
    Err(err) => {
      config_lock
        .set_opengl_requirement_met(Some(false))
        .map_err(|_| {
          CommandError::Configuration("Unable to persist opengl requirement change".to_owned())
        })?;
      return Err(CommandError::Configuration(format!(
        "Unable to request GPU device with adequate OpenGL support - {err:?}",
      )));
    }
    Ok(device) => device,
  };

  // If we didn't support the above limits, we would have returned an error already
  config_lock
    .set_opengl_requirement_met(Some(true))
    .map_err(|_| {
      CommandError::Configuration("Unable to persist opengl requirement change".to_owned())
    })?;
  Ok(Some(true))
}

#[tauri::command]
pub async fn is_opengl_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  force: bool,
) -> Result<Option<bool>, CommandError> {
  let mut config_lock = config.lock().await;
  if force {
    config_lock.requirements.opengl = None;
  }
  if let Some(bypass) = config_lock.requirements.bypass_requirements {
    if bypass {
      log::warn!("Bypassing the OpenGL requirements check!");
      return Ok(Some(true));
    }
  }
  // If the value is already set, just return it
  if let Some(val) = config_lock.requirements.opengl {
    return Ok(Some(val));
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
  let tooling_version = Version::parse(active_version.strip_prefix('v').unwrap_or(&active_version))
    .unwrap_or(Version::new(0, 1, 37));
  if tooling_version.major == 0 && tooling_version.minor <= 1 && tooling_version.patch < 38 {
    // Do it the old way
    log::info!("Checking for OpenGL support via heuristic");
    return check_opengl_via_heuristic(config_lock).await;
  } else {
    // Do it the new way!
    log::info!("Checking for OpenGL support via `gk`");
    let test_result = super::binaries::run_game_gpu_test(&config_lock, app_handle).await?;
    match test_result {
      Some(result) => {
        config_lock
          .set_opengl_requirement_met(Some(result))
          .map_err(|_| {
            CommandError::Configuration("Unable to persist opengl requirement change".to_owned())
          })?;
        Ok(Some(result))
      }
      None => Ok(None),
    }
  }
}

#[tauri::command]
pub async fn finalize_installation(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .update_installed_game_version(&game_name, true)
    .map_err(|_| {
      CommandError::Configuration("Unable to persist game installation status".to_owned())
    })?;
  app_handle.emit_all("gameInstalled", {})?;
  Ok(())
}

#[tauri::command]
pub async fn is_game_installed(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<bool, CommandError> {
  let mut config_lock = config.lock().await;

  if !config_lock.is_game_installed(&game_name) {
    return Ok(false);
  }

  // Check that the version and version folder config field is set properly as well
  let version = config_lock.game_install_version(&game_name);
  let version_folder = config_lock.game_install_version_folder(&game_name);

  if version.is_empty() || version_folder.is_empty() {
    config_lock
      .update_installed_game_version(&game_name, false)
      .map_err(|err| {
        log::error!(
          "Unable to mark partially installed game as uninstalled {}",
          err
        );
        CommandError::Configuration(
          "Unable to mark partially installed game as uninstalled".to_owned(),
        )
      })?;
    return Ok(false);
  }

  Ok(true)
}

#[tauri::command]
pub async fn get_installed_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.game_install_version(&game_name))
}

#[tauri::command]
pub async fn get_installed_version_folder(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<String, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.game_install_version_folder(&game_name))
}

#[tauri::command]
pub async fn save_active_version_change(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version_folder: String,
  new_active_version: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .set_active_version_folder(version_folder)
    .map_err(|_| {
      CommandError::Configuration("Unable to persist active version folder change".to_owned())
    })?;
  config_lock
    .set_active_version(new_active_version)
    .map_err(|_| {
      CommandError::Configuration("Unable to persist active version change".to_owned())
    })?;
  Ok(())
}

#[tauri::command]
pub async fn get_active_tooling_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<String>, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.active_version.clone())
}

#[tauri::command]
pub async fn get_active_tooling_version_folder(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<String>, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.active_version_folder.clone())
}

#[tauri::command]
pub async fn get_locale(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<String>, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.locale.clone())
}

#[tauri::command]
pub async fn set_locale(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  locale: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .set_locale(locale)
    .map_err(|_| CommandError::Configuration("Unable to persist locale change".to_owned()))?;
  Ok(())
}

#[tauri::command]
pub async fn get_bypass_requirements(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  match config_lock.requirements.bypass_requirements {
    Some(val) => Ok(val),
    None => Ok(false),
  }
}

#[tauri::command]
pub async fn set_bypass_requirements(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  bypass: bool,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock.set_bypass_requirements(bypass).map_err(|_| {
    CommandError::Configuration("Unable to persist bypass requirements change".to_owned())
  })?;
  Ok(())
}

#[tauri::command]
pub async fn get_enabled_texture_packs(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<Vec<String>, CommandError> {
  let config_lock = config.lock().await;
  Ok(config_lock.game_enabled_textured_packs(&game_name))
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
pub async fn set_enabled_texture_packs(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
  packs: Vec<String>,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;
  config_lock
    .set_game_enabled_texture_packs(&game_name, packs)
    .map_err(|_| {
      CommandError::Configuration("Unable to persist change to enabled texture packs".to_owned())
    })?;
  Ok(())
}
