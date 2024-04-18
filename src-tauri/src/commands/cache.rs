use crate::cache::LauncherCache;

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
