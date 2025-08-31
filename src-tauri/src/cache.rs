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
#[ts(export)]
pub struct ModInfo {
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

  pub async fn refresh(&mut self, sources: &[String]) -> Result<(), CacheError> {
    self.mod_sources.clear();

    for source in sources {
      match download_json(source).await {
        Ok(json) => match serde_json::from_str::<ModSourceData>(&json) {
          Ok(data) => {
            self.mod_sources.insert(source.clone(), data);
          }
          Err(err) => error!("Unable to convert downloaded json to ModSourceData: {err:?}"),
        },
        Err(err) => error!("Unable to download json from {source}: {err:?}"),
      }
    }
    Ok(())
  }
}
