use futures_util::StreamExt;
use std::{collections::HashMap, error::Error, path::Path};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::config::LauncherConfig;

#[tauri::command]
pub async fn list_downloaded_official_versions(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
) -> Result<Vec<String>, ()> {
  let config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Ok(Vec::new()),
    Some(path) => {
      let expected_path = Path::new(&path).join("/versions/official");
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
                    Some(p.to_string_lossy().into_owned())
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
      println!("BLAH - {}", path);
      // TODO - make the dir
      let expected_path = Path::new(&path).join("versions/official/test");
      println!("{}", expected_path.display());
      let client = reqwest::Client::new();
      println!("{}", url);
      let mut req = client.get(url);
      let res = req.send().await.expect("");
      println!("{:?}", res);
      let total = res.content_length().expect("");

      let mut file = File::create(expected_path).await.expect("");
      let mut stream = res.bytes_stream();

      while let Some(chunk) = stream.next().await {
        let chunk = chunk.expect("");
        file.write_all(&chunk).await.expect("");
      }
      Ok(())
    }
  }
}
