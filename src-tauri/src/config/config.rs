// Main config management for the app, doing it in rust because
// serde provides a much nicer interface for dealing with the file than doing
// it all ourselves
//
// Read the config, if it's not there, we'll generate a default
//
// Using something like config-rs or figment might be nice to provide a dev override
// but this is also as simple as just editing the file so for now, not needed
//
// serde does not support defaultLiterals yet - https://github.com/serde-rs/serde/issues/368

use std::fs;

use log::{error, info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SupportedGame {
  JAK1,
  JAK2,
  JAK3,
  JAKX,
}

impl SupportedGame {
  fn as_str(&self) -> &'static str {
    match self {
      Self::JAK1 => "Jak 1",
      Self::JAK2 => "Jak 2",
      Self::JAK3 => "Jak 3",
      Self::JAKX => "Jak X",
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameConfig {
  is_installed: bool,
  version: Option<String>,
}

impl GameConfig {
  fn default() -> Self {
    Self {
      is_installed: false,
      version: None,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupportedGames {
  #[serde(rename = "Jak 1")]
  jak1: GameConfig,
  #[serde(rename = "Jak 2")]
  jak2: GameConfig,
  #[serde(rename = "Jak 3")]
  jak3: GameConfig,
  #[serde(rename = "Jak X")]
  jakx: GameConfig,
}

impl SupportedGames {
  fn default() -> Self {
    Self {
      jak1: GameConfig::default(),
      jak2: GameConfig::default(),
      jak3: GameConfig::default(),
      jakx: GameConfig::default(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Requirements {
  avx: Option<bool>,
  #[serde(rename = "openGL")]
  opengl: Option<bool>,
}

impl Requirements {
  fn default() -> Self {
    Self {
      avx: None,
      opengl: None,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherConfig {
  #[serde(default = "default_version")]
  pub version: Option<String>,
  pub requirements: Requirements,
  pub games: SupportedGames,
  pub last_active_game: Option<SupportedGame>,
}

fn default_version() -> Option<String> {
  Some("1.0".to_string())
}

impl LauncherConfig {
  fn default() -> Self {
    Self {
      version: default_version(),
      requirements: Requirements::default(),
      games: SupportedGames::default(),
      last_active_game: None,
    }
  }

  pub fn load_config(config_dir: Option<std::path::PathBuf>) -> LauncherConfig {
    match config_dir {
      Some(config_dir) => {
        let settings_path = &config_dir.join("settings.json");
        info!("Loading configuration at path: {}", settings_path.display());
        if !settings_path.exists() {
          error!("Could not locate settings file, using defaults");
          return LauncherConfig::default();
        }
        // Read the file
        let content = match fs::read_to_string(settings_path) {
          Ok(content) => content,
          Err(err) => {
            error!("Could not read settings.json file: {}, using defaults", err);
            return LauncherConfig::default();
          }
        };

        // Serialize from json
        match serde_json::from_str::<LauncherConfig>(&content) {
          Ok(config) => {
            log::info!(
              "Successfully loaded settings file, version {}, app starting up",
              config.version.as_ref().unwrap()
            );
            return config;
          }
          Err(err) => {
            error!(
              "Could not parse settings.json file: {}, using defaults",
              err
            );
            return LauncherConfig::default();
          }
        };
      }
      None => {
        warn!("Not loading configuration, no path provided. Using defaults");
        LauncherConfig::default()
      }
    }
  }
}
