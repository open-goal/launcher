use std::collections::HashMap;

use crate::{
  cache::{LauncherCache, ModSourceData},
  config::LauncherConfig,
};

use super::CommandError;

#[tauri::command]
pub async fn refresh(
  cache: tauri::State<'_, tokio::sync::Mutex<LauncherCache>>,
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<(), CommandError> {
  let sources = { config.lock().await.mod_sources.clone() };
  let mut cache = cache.lock().await;
  cache
    .refresh(&sources)
    .await
    .map_err(|e| CommandError::Cache(format!("Unable to refresh mod source cache: {e}")))?;
  Ok(())
}

#[tauri::command]
pub async fn get_mod_sources_data(
  cache: tauri::State<'_, tokio::sync::Mutex<LauncherCache>>,
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<HashMap<String, ModSourceData>, CommandError> {
  refresh(cache.clone(), config).await?;
  let by_name = {
    let cache = cache.lock().await;
    cache
      .mod_sources
      .values()
      .cloned()
      .map(|d| (d.source_name.clone(), d))
      .collect::<HashMap<_, _>>()
  };

  Ok(by_name)
}
