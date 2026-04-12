use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Context;
use serde_json::Value;
use tracing::{info, instrument, warn};

use crate::{
  commands::CommandError,
  config::{LauncherConfig, SupportedGame},
  util::{
    file::{create_dir, delete_dir, overwrite_dir},
    zip::{check_if_zip_contains_top_level_entry, extract_zip_file},
  },
};

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
  supported_games: Vec<SupportedGame>,
  description: String,
  tags: Vec<String>,
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn list_extracted_texture_pack_info(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
) -> Result<HashMap<String, TexturePackInfo>, CommandError> {
  let expected_path = {
    let config_lock = config.lock().await;
    config_lock
      .install_dir()?
      .join("features")
      .join(game_name.to_string())
      .join("texture-packs")
  };

  let entries = match std::fs::read_dir(&expected_path) {
    Ok(entries) => entries,
    Err(_) => return Ok(HashMap::new()),
  };

  let mut package_map = HashMap::new();

  for entry in entries {
    let entry = entry?;
    let entry_path = entry.path();
    if !entry_path.is_dir() {
      continue;
    }

    let directory_name = entry_path
      .file_name()
      .and_then(|os_str| os_str.to_str())
      .map(String::from)
      .ok_or_else(|| {
        CommandError::GameFeatures(format!(
          "Unable to get directory name for {:?}",
          entry_path.display()
        ))
      })?;

    let mut file_list = Vec::new();

    for entry in glob::glob(
      &entry_path
        .join("custom_assets")
        .join(game_name.to_string())
        .join("texture_replacements/**/*.png")
        .to_string_lossy(),
    )
    .expect("Failed to read glob pattern")
    {
      match entry {
        Ok(path) => {
          let relative_path = path
            .strip_prefix(
              entry_path
                .join("custom_assets")
                .join(game_name.to_string())
                .join("texture_replacements"),
            )
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
      version: "Unknown Version".to_string(),
      author: "Unknown Author".to_string(),
      release_date: "Unknown Release Date".to_string(),
      supported_games: vec![game_name], // if no info, assume it's supported
      description: "Unknown Description".to_string(),
      tags: vec![],
    };

    let metadata_path = entry_path.join("metadata.json");
    if let Ok(metadata) = fs::File::open(&metadata_path)
      .and_then(|f| serde_json::from_reader::<_, TexturePackInfo>(f).map_err(Into::into))
    {
      pack_info = TexturePackInfo {
        has_metadata: true,
        file_list: pack_info.file_list,
        cover_image_path: pack_info.cover_image_path,
        ..metadata
      };
    } else {
      warn!(
        "Unable to load texture pack metadata {}",
        metadata_path.display()
      );
    }
    package_map.insert(directory_name, pack_info);
  }

  Ok(package_map)
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn extract_new_texture_pack(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  zip_path: PathBuf,
) -> Result<(), CommandError> {
  let install_dir = {
    let config_lock = config.lock().await;
    config_lock.install_dir()?
  };

  // First, we'll check the zip file to make sure it has a `custom_assets/<game>/texture_replacements` folder before extracting
  let texture_pack_name = zip_path
    .file_stem()
    .context("Unable to get texture pack name from zip file path")?
    .to_string_lossy()
    .into_owned();
  let expected_top_level_dir = format!("custom_assets/{game_name}/texture_replacements");
  check_if_zip_contains_top_level_entry(&zip_path, &expected_top_level_dir)?;

  // The name of the zip becomes the folder, if one already exists it will be deleted!
  let destination_dir = &install_dir
    .join("features")
    .join(game_name.to_string())
    .join("texture-packs")
    .join(&texture_pack_name);
  // TODO - delete it
  create_dir(destination_dir)?;
  extract_zip_file(&zip_path, destination_dir, false).context("Unable to extract texture pack")?;
  Ok(())
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn update_texture_pack_data(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
) -> Result<(), CommandError> {
  let config_lock = config.lock().await;
  let install_dir = config_lock.install_dir()?;

  let game_texture_pack_dir = install_dir
    .join("active")
    .join(&game_name.to_string())
    .join("data")
    .join("custom_assets")
    .join(&game_name.to_string())
    .join("texture_replacements");
  // Reset texture replacement directory
  delete_dir(&game_texture_pack_dir)?;
  create_dir(&game_texture_pack_dir)?;

  // TODO: refactor this after the config refactor
  if let Ok(Value::Array(texture_packs)) =
    config_lock.get_setting_value("active_texture_packs", Some(game_name))
  {
    for pack in texture_packs.iter().filter_map(|pack| pack.as_str()).rev() {
      let texture_pack_dir = install_dir
        .join("features")
        .join(game_name.to_string())
        .join("texture-packs")
        .join(pack)
        .join("custom_assets")
        .join(game_name.to_string())
        .join("texture_replacements");

      info!("Appending textures from: {}", texture_pack_dir.display());
      overwrite_dir(&texture_pack_dir, &game_texture_pack_dir)?
    }
    return Ok(());
  }
  return Err(CommandError::GameFeatures(format!("TODO: Some error")));
}

#[instrument(skip(config))]
#[tauri::command]
pub async fn delete_texture_packs(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: SupportedGame,
  packs: Vec<String>,
) -> Result<(), CommandError> {
  let texture_pack_dir = {
    let config_lock = config.lock().await;
    config_lock
      .install_dir()?
      .join("features")
      .join(game_name.to_string())
      .join("texture-packs")
  };

  for pack in packs {
    info!("Deleting texture pack: {}", pack);
    delete_dir(texture_pack_dir.join(&pack))?;
  }

  Ok(())
}
