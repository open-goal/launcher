use crate::{config::LauncherConfig, util::file::delete_dir};
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
    CommandError::Configuration(format!("Unable to reset configuration to defaults"))
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
  Ok(config_lock.set_install_directory(new_dir).map_err(|_| {
    CommandError::Configuration(format!("Unable to persist installation directory"))
  })?)
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
  match config_lock.requirements.avx {
    None => {
      if is_x86_feature_detected!("avx") || is_x86_feature_detected!("avx2") {
        config_lock.requirements.avx = Some(true);
      } else {
        config_lock.requirements.avx = Some(false);
      }
      config_lock.save_config().map_err(|err| {
        log::error!("Unable to persist avx requirement change {}", err);
        CommandError::Configuration(format!("Unable to persist avx requirement change"))
      })?;
      Ok(config_lock.requirements.avx.unwrap_or(false))
    }
    Some(val) => Ok(val),
  }
}

// NOTE - this is somewhat of a hack, instead of checking the actual specific version
// of OpenGL, we just check if the system supports 3D textures and large uniform buffers
// that match OpenGL 4.3's requirements.
//
// This is because OpenGL support requires libEGL -- which isn't always available on all
// platforms, and GL support in general is waning.
//
// This should be good enough...hopefully.
#[tauri::command]
pub async fn is_opengl_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  force: bool,
) -> Result<Option<bool>, CommandError> {
  let mut config_lock = config.lock().await;
  if force {
    config_lock.requirements.opengl = None;
  }
  match config_lock.requirements.opengl {
    None => {
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
            CommandError::Configuration(format!("Unable to persist opengl requirement change"))
          })?;
          return Err(CommandError::Configuration(format!(
            "Unable to request GPU adapter to check for OpenGL support"
          )));
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
        Err(_err) => {
          config_lock
            .set_opengl_requirement_met(Some(false))
            .map_err(|_| {
              CommandError::Configuration(format!("Unable to persist opengl requirement change"))
            })?;
          return Err(CommandError::Configuration(format!(
            "Unable to request GPU device with adequate OpenGL support - {:?}",
            _err
          )));
        }
        Ok(device) => device,
      };

      // If we didn't support the above limits, we would have returned an error already
      config_lock
        .set_opengl_requirement_met(Some(true))
        .map_err(|_| {
          CommandError::Configuration(format!("Unable to persist opengl requirement change"))
        })?;
      Ok(Some(true))
    }
    Some(val) => Ok(Some(val)),
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
      CommandError::Configuration(format!("Unable to persist game installation status"))
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
        CommandError::Configuration(format!(
          "Unable to mark partially installed game as uninstalled"
        ))
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
      CommandError::Configuration(format!("Unable to persist active version folder change"))
    })?;
  config_lock
    .set_active_version(new_active_version)
    .map_err(|_| CommandError::Configuration(format!("Unable to persist active version change")))?;
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
    .map_err(|_| CommandError::Configuration(format!("Unable to persist locale change")))?;
  Ok(())
}
