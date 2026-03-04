use std::collections::HashMap;

use anyhow::{Context, Result};
use log::error;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{config::SupportedGame, util::network::download_json};

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

pub struct ModCache {
  pub mod_sources: HashMap<String, ModSourceData>,
}

impl ModCache {
  pub fn default() -> Self {
    Self {
      mod_sources: HashMap::new(),
    }
  }

  async fn refresh_mod_source(&mut self, url: String) -> Result<()> {
    let source_json = download_json(&url).await?;
    let schema: ModSourceDataSchema = serde_json::from_str(&source_json)
      .with_context(|| format!("Failed to parse mod source JSON from {url}"))?;

    let source_name = schema.source_name.clone();
    let mods = schema
      .mods
      .into_iter()
      .map(|(name, schema)| (name, schema, source_name.clone()))
      .map(ModInfo::from)
      .map(|info| (info.name.clone(), info))
      .collect();

    let texture_packs = schema
      .texture_packs
      .into_iter()
      .map(|(name, schema)| (name, schema, source_name.clone()))
      .map(ModInfo::from)
      .map(|info| (info.name.clone(), info))
      .collect();

    self.mod_sources.insert(
      url,
      ModSourceData {
        schema_version: schema.schema_version,
        source_name: schema.source_name,
        last_updated: schema.last_updated,
        mods,
        texture_packs,
      },
    );
    Ok(())
  }

  pub async fn refresh_mod_sources(&mut self, sources: Vec<String>) {
    self.mod_sources.clear();
    for url in sources {
      self
        .refresh_mod_source(url)
        .await
        .unwrap_or_else(|err| error!("{err:#}"));
    }
  }
}
