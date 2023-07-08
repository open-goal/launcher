use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

use crate::{
  config::LauncherConfig,
  util::{
    file::{create_dir, overwrite_dir},
    zip::{check_if_zip_contains_top_level_dir, extract_zip_file},
  },
};

use super::CommandError;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TexturePackInfo {
  #[serde(skip_deserializing)]
  file_list: Vec<String>,
  #[serde(skip_deserializing)]
  has_metadata: bool,
  #[serde(skip_deserializing)]
  cover_image_path: Option<String>,
  name: String,
  version: String,
  author: String,
  release_date: String,
  description: String,
  tags: Vec<String>,
}

#[tauri::command]
pub async fn list_extracted_texture_pack_info(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<HashMap<String, TexturePackInfo>, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => return Ok(HashMap::new()),
    Some(path) => Path::new(path),
  };

  let expected_path = Path::new(install_path)
    .join("features")
    .join(game_name)
    .join("texture-packs");
  if !expected_path.exists() || !expected_path.is_dir() {
    log::info!(
      "No {} folder found, returning no texture packs",
      expected_path.display()
    );
    return Ok(HashMap::new());
  }

  let entries = std::fs::read_dir(&expected_path).map_err(|_| {
    CommandError::GameFeatures(format!(
      "Unable to read texture packs from {}",
      expected_path.display()
    ))
  })?;

  let mut package_map = HashMap::new();

  for entry in entries {
    let entry = entry?;
    let entry_path = entry.path();
    if entry_path.is_dir() {
      let directory_name = entry_path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .map(String::from)
        .ok_or_else(|| {
          CommandError::GameFeatures(format!("Unable to get directory name for {:?}", entry_path))
        })?;
      // Get a list of all texture files for this pack
      log::info!("Directory name: {}", directory_name);
      let mut file_list = Vec::new();
      for entry in glob::glob(
        &entry_path
          .join("texture_replacements/**/*.png")
          .to_string_lossy(),
      )
      .expect("Failed to read glob pattern")
      {
        match entry {
          Ok(path) => {
            let relative_path = path
              .strip_prefix(&entry_path.join("texture_replacements"))
              .map_err(|_| {
                CommandError::GameFeatures(format!(
                  "Unable to read texture packs from {}",
                  expected_path.display()
                ))
              })?;
            file_list.push(relative_path.display().to_string().replace("\\", "/"));
          }
          Err(e) => println!("{:?}", e),
        }
      }
      let cover_image_path = match entry_path.join("cover.png").exists() {
        true => Some(entry_path.join("cover.png").to_string_lossy().to_string()),
        false => None,
      };
      let mut pack_info = TexturePackInfo {
        file_list,
        has_metadata: false,
        cover_image_path,
        name: directory_name.to_owned(),
        version: "Unknown".to_string(),
        author: "Unknown".to_string(),
        release_date: "Unknown".to_string(),
        description: "Unknown".to_string(),
        tags: vec![],
      };
      // Read metadata if it's available
      match entry_path.join("metadata.json").exists() {
        true => {
          match std::fs::read_to_string(entry_path.join("metadata.json")) {
            Ok(content) => {
              // Serialize from json
              match serde_json::from_str::<TexturePackInfo>(&content) {
                Ok(pack_metadata) => {
                  pack_info.name = pack_metadata.name;
                  pack_info.version = pack_metadata.version;
                  pack_info.author = pack_metadata.author;
                  pack_info.release_date = pack_metadata.release_date;
                  pack_info.description = pack_metadata.description;
                  pack_info.tags = pack_metadata.tags;
                }
                Err(err) => {
                  log::error!("Unable to parse {}: {}", &content, err);
                }
              }
            }
            Err(err) => {
              log::error!(
                "Unable to read {}: {}",
                entry_path.join("metadata.json").display(),
                err
              );
            }
          };
        }
        false => {}
      }
      package_map.insert(directory_name, pack_info);
    }
  }

  Ok(package_map)
}

#[tauri::command]
pub async fn extract_new_texture_pack(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
  zip_path: String,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract texture pack".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };

  // First, we'll check the zip file to make sure it has a `texture_replacements` folder before extracting
  let zip_path_buf = PathBuf::from(zip_path);
  let texture_pack_name = match zip_path_buf.file_stem() {
    Some(name) => name.to_string_lossy().to_string(),
    None => {
      return Err(CommandError::GameFeatures(
        "Unable to get texture pack name from zip file path".to_string(),
      ));
    }
  };
  let valid_zip =
    check_if_zip_contains_top_level_dir(&zip_path_buf, "texture_replacements".to_string())
      .map_err(|err| {
        log::error!("Unable to read texture replacement zip file: {}", err);
        CommandError::GameFeatures(format!("Unable to read texture replacement pack: {}", err))
      })?;
  if !valid_zip {
    log::error!(
      "Invalid texture pack, no top-level `texture_replacements` folder: {}",
      zip_path_buf.display()
    );
    return Ok(false);
  }
  // It's valid, let's extract it.  The name of the zip becomes the folder, if one already exists it will be deleted!
  let destination_dir = &install_path
    .join("features")
    .join(game_name)
    .join("texture-packs")
    .join(&texture_pack_name);
  // TODO - delete it
  create_dir(destination_dir).map_err(|err| {
    log::error!("Unable to create directory for texture pack: {}", err);
    CommandError::GameFeatures(format!(
      "Unable to create directory for texture pack: {}",
      err
    ))
  })?;
  extract_zip_file(&zip_path_buf, &destination_dir).map_err(|err| {
    log::error!("Unable to read extract replacement pack: {}", err);
    CommandError::GameFeatures(format!("Unable to extract texture pack: {}", err))
  })?;
  Ok(true)
}

#[tauri::command]
pub async fn update_texture_pack_data(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<bool, CommandError> {
  let config_lock = config.lock().await;
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameFeatures(
        "No installation directory set, can't extract texture pack".to_string(),
      ))
    }
    Some(path) => Path::new(path),
  };

  // TODO - confirm exists
  let texture_pack_dir = install_path
    .join("features")
    .join(&game_name)
    .join("texture-packs")
    .join("snowy-legacy")
    .join("texture_replacements");
  let game_texture_pack_dir = install_path
    .join("active")
    .join(&game_name)
    .join("data")
    .join("texture_replacements");
  create_dir(&game_texture_pack_dir)?;

  // TODO - iterate
  overwrite_dir(&texture_pack_dir, &game_texture_pack_dir);

  Ok(true)
}
