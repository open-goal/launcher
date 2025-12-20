use std::collections::HashMap;

use log::error;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{config::SupportedGame, util::network::download_json};

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
  #[error("{0}")]
  #[allow(dead_code)]
  ModSource(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ModVersion {
  pub version: String,
  pub published_date: String,
  pub assets: HashMap<String, Option<String>>,
  pub supported_games: Option<Vec<SupportedGame>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ModPerGameConfig {
  pub cover_art_url: Option<String>,
  pub thumbnail_art_url: Option<String>,
  pub release_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
pub struct ModInfoSchema {
  pub display_name: String,
  pub description: String,
  pub authors: Vec<String>,
  pub tags: Vec<String>,
  pub supported_games: Vec<SupportedGame>,
  pub website_url: Option<String>,
  pub versions: Vec<ModVersion>,
  pub per_game_config: Option<HashMap<String, ModPerGameConfig>>,
  pub cover_art_url: Option<String>,
  pub thumbnail_art_url: Option<String>,
  pub external_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
pub struct ModSourceDataSchema {
  pub schema_version: String,
  pub source_name: String,
  pub last_updated: String,
  pub mods: HashMap<String, ModInfoSchema>,
  pub texture_packs: HashMap<String, ModInfoSchema>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ModInfo {
  pub name: String,
  pub source: String,
  pub display_name: String,
  pub description: String,
  pub authors: Vec<String>,
  pub tags: Vec<String>,
  pub supported_games: Vec<SupportedGame>,
  pub website_url: Option<String>,
  pub versions: Vec<ModVersion>,
  pub per_game_config: Option<HashMap<String, ModPerGameConfig>>,
  pub cover_art_url: Option<String>,
  pub thumbnail_art_url: Option<String>,
  pub external_link: Option<String>,
}

impl From<ModInfoSchema> for ModInfo {
  fn from(schema: ModInfoSchema) -> Self {
    Self {
      display_name: schema.display_name,
      description: schema.description,
      authors: schema.authors,
      tags: schema.tags,
      supported_games: schema.supported_games,
      website_url: schema.website_url,
      versions: schema.versions,
      per_game_config: schema.per_game_config,
      cover_art_url: schema.cover_art_url,
      thumbnail_art_url: schema.thumbnail_art_url,
      external_link: schema.external_link,
      // name + source filled later
      name: String::new(),
      source: String::new(),
    }
  }
}

impl From<(String, ModInfoSchema, String)> for ModInfo {
  fn from((name, schema, source): (String, ModInfoSchema, String)) -> Self {
    Self {
      name,
      source,
      ..schema.into()
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ModSourceData {
  pub schema_version: String,
  pub source_name: String,
  pub last_updated: String,
  pub mods: HashMap<String, ModInfo>,
  pub texture_packs: HashMap<String, ModInfo>,
}

pub struct LauncherCache {
  pub mod_sources: HashMap<String, ModSourceData>,
}

impl LauncherCache {
  pub fn default() -> Self {
    Self {
      mod_sources: HashMap::new(),
    }
  }

  pub async fn refresh_mod_sources(&mut self, sources: Vec<String>) -> Result<(), CacheError> {
    self.mod_sources.clear();
    for source in sources {
      let source_json = download_json(&source).await;
      match source_json {
        Ok(json) => match serde_json::from_str(&json) {
          Ok(json_value) => {
            let source_schema_data: ModSourceDataSchema = json_value;
            let source_name = source_schema_data.source_name.clone();
            let source_mods_data = source_schema_data
              .mods
              .into_iter()
              .map(|(mod_name, mod_info)| {
                let mut info: ModInfo = mod_info.into();
                info.name = mod_name;
                info.source = source_name.clone();
                (info.name.clone(), info)
              })
              .collect();
            let source_textures_data = source_schema_data
              .texture_packs
              .into_iter()
              .map(|(mod_name, mod_info)| {
                let mut info: ModInfo = mod_info.into();
                info.name = mod_name;
                info.source = source_name.clone();
                (info.name.clone(), info)
              })
              .collect();
            let source_data = ModSourceData {
              schema_version: source_schema_data.schema_version,
              source_name: source_schema_data.source_name,
              last_updated: source_schema_data.last_updated,
              mods: source_mods_data,
              texture_packs: source_textures_data,
            };
            self.mod_sources.insert(source, source_data);
          }
          Err(err) => error!("Unable to convert {json} to typed value: {err:?}"),
        },
        Err(err) => {
          error!("Unable to download json from {source}: {err:?}")
        }
      }
    }
    Ok(())
  }
}
