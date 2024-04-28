use std::collections::HashMap;

use crate::cache::{LauncherCache, ModSourceData};

use super::CommandError;

#[tauri::command]
pub async fn refresh_mod_sources(
  app_handle: tauri::AppHandle,
  cache: tauri::State<'_, tokio::sync::Mutex<LauncherCache>>,
) -> Result<(), CommandError> {
  let mut cache_lock = cache.lock().await;
  cache_lock
    .refresh_mod_sources(vec!["https://localhost:8081/mock-mods.json".to_string()])
    .await
    .map_err(|_| CommandError::Cache("Unable to refresh mod source ca che".to_owned()))?;
  Ok(())
}

#[tauri::command]
pub async fn get_mod_sources_data(
  app_handle: tauri::AppHandle,
  cache: tauri::State<'_, tokio::sync::Mutex<LauncherCache>>,
) -> Result<HashMap<String, ModSourceData>, CommandError> {
  let cache_lock = cache.lock().await;
  Ok(cache_lock.mod_sources.clone())
}
