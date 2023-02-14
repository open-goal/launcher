// Main config management for the app, doing it in rust because
// serde provides a much nicer interface for dealing with the file than doing
// it all ourselves in typescript
//
// Read the config, if it's not there, we'll generate a default
//
// Using something like config-rs or figment might be nice to provide a dev override
// but this is also as simple as just editing the file so for now, not needed
//
// serde does not support defaultLiterals yet - https://github.com/serde-rs/serde/issues/368

use std::{fs, path::PathBuf};

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
  #[serde(skip_serializing)]
  #[serde(skip_deserializing)]
  settings_path: Option<PathBuf>,

  #[serde(default = "default_version")]
  pub version: Option<String>,
  pub requirements: Requirements,
  pub games: SupportedGames,
  pub last_active_game: Option<SupportedGame>,
  pub installation_dir: Option<String>,
}

// TODO - what is _loaded?

fn default_version() -> Option<String> {
  Some("1.0".to_string())
}

impl LauncherConfig {
  fn default(_settings_path: Option<PathBuf>) -> Self {
    Self {
      settings_path: _settings_path,
      version: default_version(),
      requirements: Requirements::default(),
      games: SupportedGames::default(),
      last_active_game: None,
      installation_dir: None,
    }
  }

  pub fn load_config(config_dir: Option<std::path::PathBuf>) -> LauncherConfig {
    match config_dir {
      Some(config_dir) => {
        let settings_path = &config_dir.join("settings.json");
        log::info!("Loading configuration at path: {}", settings_path.display());
        if !settings_path.exists() {
          log::error!("Could not locate settings file, using defaults");
          return LauncherConfig::default(Some(settings_path.to_path_buf()));
        }
        // Read the file
        let content = match fs::read_to_string(settings_path) {
          Ok(content) => content,
          Err(err) => {
            log::error!("Could not read settings.json file: {}, using defaults", err);
            return LauncherConfig::default(Some(settings_path.to_path_buf()));
          }
        };

        // Serialize from json
        match serde_json::from_str::<LauncherConfig>(&content) {
          Ok(mut config) => {
            log::info!(
              "Successfully loaded settings file, version {}, app starting up",
              config.version.as_ref().unwrap()
            );
            config.settings_path = Some(settings_path.to_path_buf());
            return config;
          }
          Err(err) => {
            log::error!(
              "Could not parse settings.json file: {}, using defaults",
              err
            );
            return LauncherConfig::default(Some(settings_path.to_path_buf()));
          }
        };
      }
      None => {
        log::warn!("Not loading configuration, no path provided. Using defaults");
        LauncherConfig::default(None)
      }
    }
  }

  pub fn save_config(&self) {
    match &self.settings_path {
      None => {
        log::warn!("Can't save the settings file, as no path was initialized!");
      }
      Some(path) => {
        let file = fs::File::create(path).expect("TODO");
        serde_json::to_writer_pretty(file, &self);
      }
    }
  }

  pub fn set_install_directory(&mut self, new_dir: String) {
    self.installation_dir = Some(new_dir);
    self.save_config();
  }
}
