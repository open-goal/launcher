use std::{path::Path, process::Command};

use crate::config::LauncherConfig;

#[tauri::command]
pub async fn extract_and_validate_iso(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  path_to_iso: String,
  game_name: String,
) -> Result<(), ()> {
  let config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Ok(()),
    Some(path) => {
      // TODO - be smarter
      // TODO - make folder if it doesnt exist
      // TODO - copy over the data folder
      // TODO - log it to a file
      // TODO - check error code
      let install_path = Path::new(path);
      let binary_dir = install_path.join("versions/official/v0.1.32/");
      let data_folder = install_path.join("active/jak1/data");
      let executable_location = binary_dir.join("extractor.exe");
      let output = Command::new(&executable_location)
        .args([
          path_to_iso,
          "--extract".to_string(),
          "--validate".to_string(),
          "--proj-path".to_string(),
          data_folder.to_string_lossy().into_owned(),
        ])
        .current_dir(binary_dir)
        .output()
        .expect("failed to execute process");
      Ok(())
    }
  }
}

#[tauri::command]
pub async fn run_decompiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  path_to_iso: String,
  game_name: String,
) -> Result<(), ()> {
  let config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Ok(()),
    Some(path) => {
      // TODO - be smarter
      // TODO - make folder if it doesnt exist
      // TODO - copy over the data folder
      // TODO - log it to a file
      let install_path = Path::new(path);
      let binary_dir = install_path.join("versions/official/v0.1.32/");
      let data_folder = install_path.join("active/jak1/data");
      let executable_location = binary_dir.join("extractor.exe");
      let output = Command::new(&executable_location)
        .args([
          path_to_iso,
          "--decompile".to_string(),
          "--proj-path".to_string(),
          data_folder.to_string_lossy().into_owned(),
        ])
        .current_dir(binary_dir)
        .output()
        .expect("failed to execute process");
      Ok(())
    }
  }
}

#[tauri::command]
pub async fn run_compiler(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  path_to_iso: String,
  game_name: String,
) -> Result<(), ()> {
  let config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Ok(()),
    Some(path) => {
      // TODO - be smarter
      // TODO - make folder if it doesnt exist
      // TODO - copy over the data folder
      // TODO - log it to a file
      let install_path = Path::new(path);
      let binary_dir = install_path.join("versions/official/v0.1.32/");
      let data_folder = install_path.join("active/jak1/data");
      let executable_location = binary_dir.join("extractor.exe");
      let output = Command::new(&executable_location)
        .args([
          path_to_iso,
          "--compile".to_string(),
          "--proj-path".to_string(),
          data_folder.to_string_lossy().into_owned(),
        ])
        .current_dir(binary_dir)
        .output()
        .expect("failed to execute process");
      Ok(())
    }
  }
}
