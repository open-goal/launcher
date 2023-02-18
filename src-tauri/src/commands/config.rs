use crate::config::LauncherConfig;

#[tauri::command]
pub async fn get_install_directory(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<String>, ()> {
  let config_lock = config.lock().await;
  match config_lock.installation_dir {
    None => Ok(None),
    Some(_) => Ok(Some(
      config_lock.installation_dir.as_ref().unwrap().to_string(),
    )),
  }
}

#[tauri::command]
pub async fn set_install_directory(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  new_dir: String,
) -> Result<(), ()> {
  let mut config_lock = config.lock().await;
  config_lock.set_install_directory(new_dir);
  Ok(())
}

#[tauri::command]
pub async fn is_avx_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<bool>, ()> {
  let config_lock = config.lock().await;
  match config_lock.requirements.avx {
    None => Ok(None),
    Some(_) => Ok(config_lock.requirements.avx),
  }
}

#[tauri::command]
pub async fn is_opengl_requirement_met(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<bool>, ()> {
  let config_lock = config.lock().await;
  match config_lock.requirements.opengl {
    None => Ok(None),
    Some(_) => Ok(config_lock.requirements.opengl),
  }
}

#[tauri::command]
pub async fn finalize_installation(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<(), ()> {
  let mut config_lock = config.lock().await;
  config_lock.update_installed_game_version(game_name);
  Ok(())
}

#[tauri::command]
pub async fn is_game_installed(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<bool, ()> {
  let config_lock = config.lock().await;
  Ok((config_lock.is_game_installed(game_name)))
}
