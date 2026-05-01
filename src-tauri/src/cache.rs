use std::collections::HashMap;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tauri_plugin_os::platform;
use tracing::error;
use ts_rs::TS;

use crate::{
  config::{LauncherConfig, SupportedGame},
  util::network::download_json,
};

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ModVersion {
  pub version: String,
  pub published_date: String,
  pub assets: HashMap<String, Option<String>>,
  pub supported_games: Option<Vec<SupportedGame>>,
  pub asset_download_counts: Option<HashMap<String, u64>>,
}

impl ModVersion {
  pub fn supports_platform(&self) -> bool {
    self
      .assets
      .get(platform())
      .and_then(|url| url.as_ref())
      .is_some()
  }
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
  pub installed: bool,
  pub download_count: u64,
}

impl From<ModInfoSchema> for ModInfo {
  fn from(schema: ModInfoSchema) -> Self {
    let download_count = schema
    .versions
    .iter()
    .filter_map(|version| version.asset_download_counts.as_ref())
    .flat_map(|counts| counts.values())
    .sum();

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
      installed: false,
      download_count: download_count,
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

impl ModSourceData {
  pub fn by_platform(&self) -> Self {
    Self {
      schema_version: self.schema_version.clone(),
      source_name: self.source_name.clone(),
      last_updated: self.last_updated.clone(),
      mods: self
        .mods
        .iter()
        .filter_map(|(name, info)| {
          let mut info = info.clone();
          info.versions.retain(|v| v.supports_platform());
          if info.versions.is_empty() {
            None
          } else {
            Some((name.clone(), info))
          }
        })
        .map(|(name, info)| (name.clone(), info.clone()))
        .collect(),
      texture_packs: self
        .texture_packs
        .iter()
        .filter_map(|(name, info)| {
          let mut info = info.clone();
          info.versions.retain(|v| v.supports_platform());
          if info.versions.is_empty() {
            None
          } else {
            Some((name.clone(), info))
          }
        })
        .map(|(name, info)| (name.clone(), info.clone()))
        .collect(),
    }
  }
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

  pub fn by_platform(&self) -> HashMap<String, ModSourceData> {
    self
      .mod_sources
      .iter()
      .map(|(url, source)| (url.clone(), source.by_platform()))
      .collect()
  }

  fn mods_for_game(&self, game: SupportedGame, config: &LauncherConfig) -> Vec<ModInfo> {
    let mut mods: Vec<ModInfo> = self
      .mod_sources
      .values()
      .flat_map(|source| source.mods.values())
      .filter_map(|info| {
        if !info.supported_games.contains(&game) {
          return None;
        }

        let mut info = info.clone();

        info.versions.retain(|version| {
          if !version.supports_platform() {
            return false;
          }

          version
            .supported_games
            .as_ref()
            .is_none_or(|games| games.contains(&game))
        });

        if info.versions.is_empty() {
          return None;
        }

        info.installed = config.is_mod_installed(game, &info.source, &info.name);
        Some(info)
      })
      .collect();

    mods.sort_by(|a, b| {
      a.display_name
        .to_lowercase()
        .cmp(&b.display_name.to_lowercase())
    });
    mods
  }

  pub fn available_mods(&self, config: &LauncherConfig) -> AvailableModsByGame {
    AvailableModsByGame {
      jak1: self.mods_for_game(SupportedGame::Jak1, config),
      jak2: self.mods_for_game(SupportedGame::Jak2, config),
      jak3: self.mods_for_game(SupportedGame::Jak3, config),
    }
  }
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AvailableModsByGame {
  pub jak1: Vec<ModInfo>,
  pub jak2: Vec<ModInfo>,
  pub jak3: Vec<ModInfo>,
}
