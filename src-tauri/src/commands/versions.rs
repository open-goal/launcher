use futures_util::StreamExt;
use std::{io::Cursor, path::Path};
use tauri::Manager;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{config::LauncherConfig, util};

#[tauri::command]
pub async fn list_downloaded_versions(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version_folder: String,
) -> Result<Vec<String>, ()> {
  let config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Ok(Vec::new()),
    Some(path) => {
      let expected_path = Path::new(path).join("versions").join(version_folder);
      if !expected_path.is_dir() {
        Ok(Vec::new())
      } else {
        match std::fs::read_dir(expected_path) {
          Err(_) => Ok(Vec::new()),
          Ok(entries) => Ok(
            entries
              .filter_map(|e| {
                e.ok().and_then(|d| {
                  let p = d.path();
                  if p.is_dir() {
                    Some(
                      p.file_name()
                        .map(|name| name.to_string_lossy().into_owned())
                        .unwrap_or("".into()),
                    )
                  } else {
                    None
                  }
                })
              })
              .collect(),
          ),
        }
      }
    }
  }
}

#[tauri::command]
pub async fn download_official_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version: String,
  url: String,
) -> Result<(), bool> {
  let config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Ok(()),
    Some(path) => {
      // TODO - severe lack of safety here!
      // TODO - make the dir and the file name
      let expected_path = Path::new(&path).join("versions/official/test.zip");
      let client = reqwest::Client::new();
      let mut req = client.get(url);
      let res = req.send().await.expect("");
      let total = res.content_length().expect("");

      let mut file = File::create(expected_path).await.expect("");
      let mut stream = res.bytes_stream();

      while let Some(chunk) = stream.next().await {
        let chunk = chunk.expect("");
        file.write_all(&chunk).await.expect("");
      }

      let target_dir = Path::new(&path).join("versions/official/").join(version);

      let zip_path = Path::new(&path).join("versions/official/test.zip");

      let archive: Vec<u8> = std::fs::read(&zip_path.clone()).unwrap();
      zip_extract::extract(Cursor::new(archive), &target_dir, true).expect("");

      std::fs::remove_file(zip_path).expect("TODO");

      Ok(())
    }
  }
}

#[tauri::command]
pub async fn go_to_version_folder(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  version_folder: String,
) -> Result<(), ()> {
  let config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Err(()),
    Some(path) => {
      let expected_path = Path::new(path).join("versions").join(version_folder);
      util::open_dir_in_os(expected_path.to_string_lossy().into_owned());
      Ok(())
    }
  }
}

#[tauri::command]
pub async fn save_active_version_change(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  version_folder: String,
  new_active_version: String,
) -> Result<(), ()> {
  let mut config_lock = config.lock().await;
  // TODO - error checking
  config_lock.set_active_version_folder(version_folder);
  config_lock.set_active_version(new_active_version);
  app_handle.emit_all("toolingVersionChanged", {}).unwrap();
  Ok(())
}

#[tauri::command]
pub async fn get_active_version(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<String>, ()> {
  let config_lock = config.lock().await;
  Ok(config_lock.active_version.clone())
}

#[tauri::command]
pub async fn get_active_version_folder(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Option<String>, ()> {
  let config_lock = config.lock().await;
  Ok(config_lock.active_version_folder.clone())
}
