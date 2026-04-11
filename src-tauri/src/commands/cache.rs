use std::collections::HashMap;

use tracing::instrument;

use crate::{
  cache::{ModCache, ModSourceData},
  config::LauncherConfig,
};

use super::CommandError;

#[instrument(skip(config, cache))]
#[tauri::command]
pub async fn refresh_mod_sources(
  cache: tauri::State<'_, tokio::sync::Mutex<ModCache>>,
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<(), ()> {
  let mut cache_lock = cache.lock().await;
  let config_lock = config.lock().await;
  let mod_sources = config_lock.mod_sources.clone();
  cache_lock.refresh_mod_sources(mod_sources).await;
  Ok(())
}

#[instrument(skip(cache))]
#[tauri::command]
pub async fn get_mod_sources_data(
  cache: tauri::State<'_, tokio::sync::Mutex<ModCache>>,
) -> Result<HashMap<String, ModSourceData>, CommandError> {
  let cache_lock = cache.lock().await;
  Ok(cache_lock.by_platform())
}
