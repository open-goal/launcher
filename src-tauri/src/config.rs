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

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::util::file::touch_file;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  JSONError(#[from] serde_json::Error),
  #[error("{0}")]
  Configuration(String),
}

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
  pub is_installed: bool,
  pub version: Option<String>,
  pub version_folder: Option<String>,
}

impl GameConfig {
  fn default() -> Self {
    Self {
      is_installed: false,
      version: None,
      version_folder: None,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupportedGames {
  #[serde(rename = "Jak 1")]
  pub jak1: GameConfig,
  #[serde(rename = "Jak 2")]
  pub jak2: GameConfig,
  #[serde(rename = "Jak 3")]
  pub jak3: GameConfig,
  #[serde(rename = "Jak X")]
  pub jakx: GameConfig,
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
  pub avx: Option<bool>,
  #[serde(rename = "openGL")]
  pub opengl: Option<bool>,
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
  pub movie_dir: Option<String>,
  pub active_version: Option<String>,
  pub active_version_folder: Option<String>,
}

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
      movie_dir: None,
      active_version: None,
      active_version_folder: Some("official".to_string()),
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

  pub fn save_config(&self) -> Result<(), ConfigError> {
    let settings_path = match &self.settings_path {
      None => {
        log::warn!("Can't save the settings file, as no path was initialized!");
        return Err(ConfigError::Configuration(format!(
          "No settings path defined, unable to save settings!"
        )));
      }
      Some(path) => path,
    };
    let file = fs::File::create(settings_path)?;
    serde_json::to_writer_pretty(file, &self)?;
    Ok(())
  }

  pub fn reset_to_defaults(&mut self) -> Result<(), ConfigError> {
    let original_installation_dir = self.installation_dir.clone();
    *self = Self::default(self.settings_path.clone());
    self.installation_dir = original_installation_dir;
    Self::save_config(self)?;
    Ok(())
  }

  pub fn set_install_directory(&mut self, new_dir: String) -> Result<Option<String>, ConfigError> {
    // Do some tests on this folder, if they fail, return a decent error
    let path = Path::new(&new_dir);
    if !path.exists() {
      return Ok(Some("Provided folder does not exist".to_owned()));
    }

    if !path.is_dir() {
      return Ok(Some("Provided folder is not a folder".to_owned()));
    }

    // Check our permissions on the folder by touching a file (and deleting it)
    let test_file = path.join(".perm-test.tmp");
    match touch_file(&test_file) {
      Err(e) => {
        log::error!(
          "Provided installation folder could not be written to: {}",
          e
        );
        return Ok(Some("Provided folder cannot be written to".to_owned()));
      }
      _ => (),
    }

    // If the directory changes (it's not a no-op), we need to:
    // - wipe any installed games (make them reinstall)
    // - wipe the active version/version types
    match &self.installation_dir {
      Some(old_dir) => {
        if *old_dir != new_dir {
          self.active_version = None;
          self.active_version_folder = None;
          // TODO - when i cleanup the gross code below, also clean this up
          self.games.jak1.is_installed = false;
          self.games.jak1.version = None;
          self.games.jak1.version_folder = None;
        }
      }
      _ => (),
    }

    self.installation_dir = Some(new_dir);
    self.save_config()?;
    Ok(None)
  }

  pub fn set_movie_directory(&mut self, new_dir: String) -> Result<Option<String>, ConfigError> {
    // Do some tests on this file, if they fail, return a decent error
    let path = Path::new(&new_dir);
    if !path.exists() {
      return Ok(Some("Provided file does not exist".to_owned()));
    }

    if !path.is_file() {
      return Ok(Some("Provided file is not a file".to_owned()));
    }

    // TODO Check our permissions on the folder by touching a file (and deleting it)

    self.movie_dir = Some(new_dir);
    self.save_config()?;
    Ok(None)
  }

  pub fn set_opengl_requirement_met(&mut self, new_val: bool) -> Result<(), ConfigError> {
    self.requirements.opengl = Some(new_val);
    self.save_config()?;
    Ok(())
  }

  pub fn set_active_version(&mut self, new_version: String) -> Result<(), ConfigError> {
    self.active_version = Some(new_version);
    self.save_config()?;
    Ok(())
  }

  pub fn set_active_version_folder(
    &mut self,
    new_version_folder: String,
  ) -> Result<(), ConfigError> {
    self.active_version_folder = Some(new_version_folder);
    self.save_config()?;
    Ok(())
  }

  // TODO - this pattern isn't great.  It's made awkward by trying to be backwards compatible
  // with the old format though
  //
  // I think there should be an enum involved here somewhere/somehow
  pub fn update_installed_game_version(
    &mut self,
    game_name: &String,
    installed: bool,
  ) -> Result<(), ConfigError> {
    match game_name.as_str() {
      "jak1" => {
        self.games.jak1.is_installed = installed;
        if installed {
          self.games.jak1.version = self.active_version.clone();
          self.games.jak1.version_folder = self.active_version_folder.clone();
        } else {
          self.games.jak1.version = None;
          self.games.jak1.version_folder = None;
        }
      }
      "jak2" => {
        self.games.jak2.is_installed = installed;
        if installed {
          self.games.jak2.version = self.active_version.clone();
          self.games.jak2.version_folder = self.active_version_folder.clone();
        } else {
          self.games.jak2.version = None;
          self.games.jak2.version_folder = None;
        }
      }
      "jak3" => {
        self.games.jak3.is_installed = installed;
        if installed {
          self.games.jak3.version = self.active_version.clone();
          self.games.jak3.version_folder = self.active_version_folder.clone();
        } else {
          self.games.jak3.version = None;
          self.games.jak3.version_folder = None;
        }
      }
      "jakx" => {
        self.games.jakx.is_installed = installed;
        if installed {
          self.games.jakx.version = self.active_version.clone();
          self.games.jakx.version_folder = self.active_version_folder.clone();
        } else {
          self.games.jakx.version = None;
          self.games.jakx.version_folder = None;
        }
      }
      _ => {}
    }
    self.save_config()?;
    Ok(())
  }

  pub fn is_game_installed(&self, game_name: &String) -> bool {
    match game_name.as_str() {
      "jak1" => {
        return self.games.jak1.is_installed;
      }
      "jak2" => {
        return self.games.jak2.is_installed;
      }
      "jak3" => {
        return self.games.jak3.is_installed;
      }
      "jakx" => {
        return self.games.jakx.is_installed;
      }
      _ => false,
    }
  }

  pub fn game_install_version(&self, game_name: &String) -> String {
    match game_name.as_str() {
      "jak1" => {
        return self.games.jak1.version.clone().unwrap_or("".to_string());
      }
      "jak2" => {
        return self.games.jak2.version.clone().unwrap_or("".to_string());
      }
      "jak3" => {
        return self.games.jak3.version.clone().unwrap_or("".to_string());
      }
      "jakx" => {
        return self.games.jakx.version.clone().unwrap_or("".to_string());
      }
      _ => "".to_owned(),
    }
  }

  pub fn game_install_version_folder(&self, game_name: &String) -> String {
    match game_name.as_str() {
      "jak1" => {
        return self
          .games
          .jak1
          .version_folder
          .clone()
          .unwrap_or("".to_string());
      }
      "jak2" => {
        return self
          .games
          .jak2
          .version_folder
          .clone()
          .unwrap_or("".to_string());
      }
      "jak3" => {
        return self
          .games
          .jak3
          .version_folder
          .clone()
          .unwrap_or("".to_string());
      }
      "jakx" => {
        return self
          .games
          .jakx
          .version_folder
          .clone()
          .unwrap_or("".to_string());
      }
      _ => "".to_owned(),
    }
  }
}
