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

use crate::util::file::create_dir;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum SupportedGame {
  Jak1,
  Jak2,
  Jak3,
  JakX,
}

impl SupportedGame {
  fn internal_str(&self) -> &'static str {
    match self {
      SupportedGame::Jak1 => "jak1",
      SupportedGame::Jak2 => "jak2",
      SupportedGame::Jak3 => "jak3",
      SupportedGame::JakX => "jakx",
    }
  }
}

impl FromStr for SupportedGame {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "jak1" => Ok(Self::Jak1),
      "jak2" => Ok(Self::Jak2),
      "jak3" => Ok(Self::Jak3),
      "jakx" => Ok(Self::JakX),
      _ => Err(format!("Invalid variant: {s}")),
    }
  }
}

impl<'de> Deserialize<'de> for SupportedGame {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
      "Jak 1" => Ok(SupportedGame::Jak1),
      "Jak 2" => Ok(SupportedGame::Jak2),
      "Jak 3" => Ok(SupportedGame::Jak3),
      "Jak X" => Ok(SupportedGame::JakX),
      _ => Err(serde::de::Error::unknown_variant(
        &s,
        &["Jak 1", "Jak 2", "Jak 3", "Jak X"],
      )),
    }
  }
}

impl Serialize for SupportedGame {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(match self {
      SupportedGame::Jak1 => "Jak 1",
      SupportedGame::Jak2 => "Jak 2",
      SupportedGame::Jak3 => "Jak 3",
      SupportedGame::JakX => "Jak X",
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameFeatureConfig {
  pub texture_packs: Vec<String>,
}

impl GameFeatureConfig {
  fn default() -> Self {
    Self {
      texture_packs: vec![],
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameConfig {
  pub is_installed: bool,
  pub version: Option<String>,
  pub version_folder: Option<String>,
  pub features: Option<GameFeatureConfig>,
  pub seconds_played: Option<u64>,
}

impl GameConfig {
  fn default() -> Self {
    Self {
      is_installed: false,
      version: None,
      version_folder: None,
      features: Some(GameFeatureConfig::default()),
      seconds_played: Some(0),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirements {
  pub bypass_requirements: Option<bool>,
  pub avx: Option<bool>,
  #[serde(rename = "openGL")]
  pub opengl: Option<bool>,
}

impl Requirements {
  fn default() -> Self {
    Self {
      bypass_requirements: Some(false),
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
  pub games: HashMap<SupportedGame, GameConfig>,
  pub last_active_game: Option<SupportedGame>,
  pub installation_dir: Option<String>,
  pub active_version: Option<String>,
  pub active_version_folder: Option<String>,
  pub locale: Option<String>,
}

fn default_version() -> Option<String> {
  Some("1.0".to_string())
}

impl LauncherConfig {
  fn default(_settings_path: Option<PathBuf>) -> Self {
    let mut default_games = HashMap::new();
    default_games.insert(SupportedGame::Jak1, GameConfig::default());
    default_games.insert(SupportedGame::Jak2, GameConfig::default());
    default_games.insert(SupportedGame::Jak3, GameConfig::default());
    default_games.insert(SupportedGame::JakX, GameConfig::default());
    Self {
      settings_path: _settings_path,
      version: default_version(),
      requirements: Requirements::default(),
      games: default_games,
      last_active_game: None,
      installation_dir: None,
      active_version: None,
      active_version_folder: Some("official".to_string()),
      locale: None,
    }
  }

  fn get_supported_game_config_mut(
    &mut self,
    game_name: &String,
  ) -> Result<&mut GameConfig, ConfigError> {
    let game = match SupportedGame::from_str(game_name) {
      Err(_) => {
        log::warn!("Game is not supported: {}", game_name);
        return Err(ConfigError::Configuration(
          "Game is not supported".to_owned(),
        ));
      }
      Ok(game) => game,
    };
    match self.games.get_mut(&game) {
      None => {
        log::error!("Supported game missing from games map: {}", game_name);
        return Err(ConfigError::Configuration(format!(
          "Supported game missing from games map: {game_name}"
        )));
      }
      Some(cfg) => Ok(cfg),
    }
  }

  fn get_supported_game_config(&mut self, game_name: &String) -> Result<&GameConfig, ConfigError> {
    let game = match SupportedGame::from_str(game_name) {
      Err(_) => {
        log::warn!("Game is not supported: {}", game_name);
        return Err(ConfigError::Configuration(
          "Game is not supported".to_owned(),
        ));
      }
      Ok(game) => game,
    };
    match self.games.get(&game) {
      None => {
        log::error!("Supported game missing from games map: {}", game_name);
        return Err(ConfigError::Configuration(format!(
          "Supported game missing from games map: {game_name}"
        )));
      }
      Some(cfg) => Ok(cfg),
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
            config
          }
          Err(err) => {
            log::error!(
              "Could not parse settings.json file: {}, using defaults",
              err
            );
            LauncherConfig::default(Some(settings_path.to_path_buf()))
          }
        }
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
        return Err(ConfigError::Configuration(
          "No settings path defined, unable to save settings!".to_owned(),
        ));
      }
      Some(path) => path,
    };
    // Ensure the directory exists
    create_dir(&settings_path.parent().unwrap().to_path_buf())?;
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
    if let Err(e) = touch_file(&test_file) {
      log::error!(
        "Provided installation folder could not be written to: {}",
        e
      );
      return Ok(Some("Provided folder cannot be written to".to_owned()));
    }

    // If the directory changes (it's not a no-op), we need to:
    // - wipe any installed games (make them reinstall)
    // - wipe the active version/version types
    if let Some(old_dir) = &self.installation_dir {
      if *old_dir != new_dir {
        self.active_version = None;
        self.active_version_folder = None;
        self
          .update_installed_game_version(&SupportedGame::Jak1.internal_str().to_string(), false)?;
        self
          .update_installed_game_version(&SupportedGame::Jak2.internal_str().to_string(), false)?;
        self
          .update_installed_game_version(&SupportedGame::Jak3.internal_str().to_string(), false)?;
        self
          .update_installed_game_version(&SupportedGame::JakX.internal_str().to_string(), false)?;
      }
    }

    self.installation_dir = Some(new_dir);
    self.save_config()?;
    Ok(None)
  }

  pub fn set_opengl_requirement_met(&mut self, new_val: Option<bool>) -> Result<(), ConfigError> {
    match new_val {
      Some(val) => {
        self.requirements.opengl = Some(val);
      }
      None => self.requirements.opengl = None,
    }
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

  pub fn clear_active_version(&mut self) -> Result<(), ConfigError> {
    self.active_version = None;
    self.active_version_folder = None;
    self.save_config()?;
    Ok(())
  }

  pub fn set_locale(&mut self, new_locale: String) -> Result<(), ConfigError> {
    self.locale = Some(new_locale);
    self.save_config()?;
    Ok(())
  }

  pub fn set_bypass_requirements(&mut self, bypass: bool) -> Result<(), ConfigError> {
    self.requirements.bypass_requirements = Some(bypass);
    self.save_config()?;
    Ok(())
  }

  pub fn update_installed_game_version(
    &mut self,
    game_name: &String,
    installed: bool,
  ) -> Result<(), ConfigError> {
    match SupportedGame::from_str(game_name) {
      Ok(game) => {
        // Retrieve relevant game from config
        match self.games.get_mut(&game) {
          Some(game) => {
            game.is_installed = installed;
            if installed {
              game.version = self.active_version.clone();
              game.version_folder = self.active_version_folder.clone();
            } else {
              game.version = None;
              game.version_folder = None;
            }
          }
          None => {
            return Err(ConfigError::Configuration(format!(
              "Invalid game name - {game_name}, can't update installation status!",
            )));
          }
        }
      }
      Err(_) => {
        return Err(ConfigError::Configuration(format!(
          "Invalid game name - {game_name}, can't update installation status!",
        )));
      }
    }
    self.save_config()?;
    Ok(())
  }

  pub fn is_game_installed(&self, game_name: &String) -> bool {
    match SupportedGame::from_str(game_name) {
      Ok(game) => {
        // Retrieve relevant game from config
        match self.games.get(&game) {
          Some(game) => game.is_installed,
          None => {
            log::warn!(
              "Could not find game to check if it's installed: {}",
              game_name
            );
            false
          }
        }
      }
      Err(_) => {
        log::warn!(
          "Could not find game to check if it's installed: {}",
          game_name
        );
        false
      }
    }
  }

  pub fn game_install_version(&self, game_name: &String) -> String {
    match SupportedGame::from_str(game_name) {
      Ok(game) => {
        // Retrieve relevant game from config
        match self.games.get(&game) {
          Some(game) => game.version.clone().unwrap_or("".to_owned()),
          None => {
            log::warn!(
              "Could not find game to check what version is installed: {}",
              game_name
            );
            "".to_owned()
          }
        }
      }
      Err(_) => {
        log::warn!(
          "Could not find game to check what version is installed: {}",
          game_name
        );
        "".to_owned()
      }
    }
  }

  pub fn game_install_version_folder(&self, game_name: &String) -> String {
    match SupportedGame::from_str(game_name) {
      Ok(game) => {
        // Retrieve relevant game from config
        match self.games.get(&game) {
          Some(game) => game.version_folder.clone().unwrap_or("".to_string()),
          None => {
            log::warn!(
              "Could not find game to check what version type is installed: {}",
              game_name
            );
            "".to_owned()
          }
        }
      }
      Err(_) => {
        log::warn!(
          "Could not find game to check what version is installed: {}",
          game_name
        );
        "".to_owned()
      }
    }
  }

  pub fn game_enabled_textured_packs(&self, game_name: &String) -> Vec<String> {
    // TODO - refactor out duplication
    match SupportedGame::from_str(game_name) {
      Ok(game) => {
        // Retrieve relevant game from config
        match self.games.get(&game) {
          Some(game) => match &game.features {
            Some(features) => features.texture_packs.to_owned(),
            None => Vec::new(),
          },
          None => {
            log::warn!(
              "Could not find game to check which texture packs are enabled: {}",
              game_name
            );
            Vec::new()
          }
        }
      }
      Err(_) => {
        log::warn!(
          "Could not find game to check which texture packs are enabled: {}",
          game_name
        );
        Vec::new()
      }
    }
  }

  pub fn cleanup_game_enabled_texture_packs(
    &mut self,
    game_name: &String,
    cleanup_list: Vec<String>,
  ) -> Result<(), ConfigError> {
    if !cleanup_list.is_empty() {
      return Ok(());
    }
    let game_config = self.get_supported_game_config_mut(game_name)?;
    if let Some(features) = &mut game_config.features {
      features
        .texture_packs
        .retain(|pack| !cleanup_list.contains(pack));
      self.save_config()?;
    }
    Ok(())
  }

  pub fn set_game_enabled_texture_packs(
    &mut self,
    game_name: &String,
    packs: Vec<String>,
  ) -> Result<(), ConfigError> {
    let game_config = self.get_supported_game_config_mut(game_name)?;
    match &mut game_config.features {
      Some(features) => {
        features.texture_packs = packs;
      }
      None => {
        game_config.features = Some(GameFeatureConfig {
          texture_packs: packs,
        });
      }
    }
    self.save_config()?;
    Ok(())
  }

  pub fn update_game_seconds_played(
    &mut self,
    game_name: &String,
    additional_seconds: u64,
  ) -> Result<(), ConfigError> {
    let game_config = self.get_supported_game_config_mut(game_name)?;
    match game_config.seconds_played {
      Some(seconds) => {
        game_config.seconds_played = Some(seconds + additional_seconds);
      }
      None => {
        game_config.seconds_played = Some(additional_seconds);
      }
    }
    self.save_config()?;
    Ok(())
  }
}
