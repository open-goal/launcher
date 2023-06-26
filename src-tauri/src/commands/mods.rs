use std::path::Path;
use std::collections::HashMap;
use crate::config::ModList;

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
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;

  info!("Adding Mod List {}:{}", identifier, url);

  config_lock.add_mod_list(&url, &identifier);

  Ok(())
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