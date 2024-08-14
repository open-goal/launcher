use std::collections::HashMap;

use crate::{
  cache::{LauncherCache, ModSourceData},
  config::LauncherConfig,
};

use super::CommandError;

#[tauri::command]
pub async fn refresh_mod_sources(
  cache: tauri::State<'_, tokio::sync::Mutex<LauncherCache>>,
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<(), CommandError> {
  let mut cache_lock = cache.lock().await;
  let config_lock = config.lock().await;
  if let Some(mod_sources) = &config_lock.mod_sources {
    cache_lock
      .refresh_mod_sources(mod_sources.to_vec())
      .await
      .map_err(|_| CommandError::Cache("Unable to refresh mod source cache".to_owned()))?;
  }
  Ok(())
}

#[tauri::command]
pub async fn get_mod_sources_data(
  cache: tauri::State<'_, tokio::sync::Mutex<LauncherCache>>,
) -> Result<HashMap<String, ModSourceData>, CommandError> {
  let cache_lock = cache.lock().await;
  Ok(cache_lock.mod_sources.clone())
}
