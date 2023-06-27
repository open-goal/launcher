use std::path::Path;
use std::collections::HashMap;
use crate::config::ModList;
use crate::config::ModConfig;

use log::info;

use crate::{
  config::LauncherConfig,
  util::{
    file::{create_dir, delete_dir},
    network::download_file,
    os::open_dir_in_os,
    tar::extract_and_delete_tar_ball,
    zip::extract_and_delete_zip_file,
  },
};

use super::CommandError;

#[tauri::command]
pub async fn add_mod_list(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  url: String,
  identifier: String,
  mods_json: String,
) -> Result<(), CommandError> {  
  
  match serde_json::from_str::<Vec<ModConfig>>(&mods_json) {
    Ok(mods) => {
      info!("Adding Mod List {}:{}", identifier, url);
  
      let mut config_lock = config.lock().await;
      config_lock.add_mod_list(&url, &identifier, mods);

      Ok(())
    }
    Err(e) => {
      info!("failed to parse JSON for {}:{}, err: {}", identifier, url, e);
      panic!("Failed to parse JSON: {}", e);
    }
  }

  
}

#[tauri::command]
pub async fn remove_mod_list(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  identifier: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;

  info!("Removing Mod List {}", identifier);

  config_lock.remove_mod_list(&identifier);

  Ok(())
}

#[tauri::command]
pub async fn get_mod_lists(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Vec<ModList>, CommandError> {
  let config_lock = config.lock().await;

  Ok(config_lock.get_mod_lists())
}