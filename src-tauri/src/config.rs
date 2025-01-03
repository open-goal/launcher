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
use serde_json::{json, Value};

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
  pub features: GameFeatureConfig,
  pub seconds_played: u64,
  pub mods_installed_version: HashMap<String, HashMap<String, String>>,
}

impl GameConfig {
  fn default() -> Self {
    Self {
      is_installed: false,
      version: None,
      version_folder: None,
      features: GameFeatureConfig::default(),
      seconds_played: 0,
      mods_installed_version: HashMap::new(),
    }
  }

  pub fn active_texture_packs(&self) -> Vec<String> {
    self.features.texture_packs.clone()
  }

  pub fn version(&self) -> Option<String> {
    self.version.clone()
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirements {
  pub bypass_requirements: bool,
  pub avx: bool,
  #[serde(rename = "openGL")]
  pub opengl: bool,
}

impl Requirements {
  fn default() -> Self {
    Self {
      bypass_requirements: false,
      avx: false,
      opengl: false,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecompilerSettings {
  pub rip_levels_enabled: bool,
  pub rip_collision_enabled: bool,
  pub rip_textures_enabled: bool,
  pub rip_streamed_audio_enabled: bool,
}

impl DecompilerSettings {
  fn default() -> Self {
    Self {
      rip_levels_enabled: false,
      rip_collision_enabled: false,
      rip_textures_enabled: false,
      rip_streamed_audio_enabled: false,
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
  pub mod_sources: Vec<String>,
  pub decompiler_settings: DecompilerSettings,
  pub check_for_latest_mod_version: bool,
  pub proceed_after_successful_operation: bool,
  pub auto_update_games: bool,
  pub delete_previous_versions: bool,
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
      mod_sources: Vec::new(),
      decompiler_settings: DecompilerSettings::default(),
      check_for_latest_mod_version: true,
      proceed_after_successful_operation: true,
      auto_update_games: false,
      delete_previous_versions: false,
    }
  }

  fn get_supported_game_config_mut(
    &mut self,
    game_name: &String,
  ) -> Result<&mut GameConfig, ConfigError> {
    SupportedGame::from_str(game_name)
      .ok()
      .and_then(|game| self.games.get_mut(&game))
      .ok_or_else(|| {
        log::error!("Game not found or unsupported: {}", game_name);
        ConfigError::Configuration(format!("Game not found or unsupported: {game_name}"))
      })
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
            // Remove usages of non-official versions
            let mut found_violations = false;
            if config
              .active_version_folder
              .as_ref()
              .is_some_and(|x| x != "official")
            {
              log::warn!("non-official versions is a deprecated feature, erasing!");
              config.active_version = None;
              config.active_version_folder = None;
              found_violations = true;
            }
            // check the games as well
            for (_, game_info) in config.games.iter_mut() {
              if game_info
                .version_folder
                .as_ref()
                .is_some_and(|x| x != "official")
              {
                log::warn!("non-official versions is a deprecated feature, erasing!");
                game_info.version = None;
                game_info.version_folder = None;
                found_violations = true;
              }
            }
            if found_violations {
              config.save_config().expect("TODO NOW");
            }
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

  pub fn set_install_directory(&mut self, new_dir: String) -> Result<(), ConfigError> {
    // Do some tests on this folder, if they fail, return a decent error
    let path = Path::new(&new_dir);
    if !path.exists() {
      return Err(ConfigError::Configuration(
        "Provided folder does not exist".to_owned(),
      ));
    }

    if !path.is_dir() {
      return Err(ConfigError::Configuration(
        "Provided folder is not a folder".to_owned(),
      ));
    }

    // Check our permissions on the folder by touching a file (and deleting it)
    let test_file = path.join(".perm-test.tmp");
    if let Err(e) = touch_file(&test_file) {
      log::error!(
        "Provided installation folder could not be written to: {}",
        e
      );
      return Err(ConfigError::Configuration(
        "Provided folder cannot be written to".to_owned(),
      ));
    }

    // If the directory changes (it's not a no-op), we need to:
    // - wipe any installed games (make them reinstall)
    // - wipe the active version/version types
    if let Some(old_dir) = &self.installation_dir {
      if *old_dir != new_dir {
        self.active_version = None;
        self.active_version_folder = None;
        self.update_setting_value("installed", false.into(), Some("jak1".to_owned()))?;
        self.update_setting_value("installed", false.into(), Some("jak2".to_owned()))?;
        self.update_setting_value("installed", false.into(), Some("jak3".to_owned()))?;
        self.update_setting_value("installed", false.into(), Some("jakx".to_owned()))?;
      }
    }

    self.installation_dir = Some(new_dir);
    self.save_config()?;
    Ok(())
  }

  pub fn update_setting_value(
    &mut self,
    key: &str,
    val: Value,
    game_name: Option<String>,
  ) -> Result<(), ConfigError> {
    if let Some(game_config) = game_name
      .as_deref()
      .and_then(|name| SupportedGame::from_str(name).ok())
      .and_then(|game| self.games.get_mut(&game))
    {
      match key {
        "installed" => {
          let installed = val.as_bool().unwrap_or(false);
          game_config.is_installed = installed;
          if installed {
            game_config.version = self.active_version.clone();
            game_config.version_folder = self.active_version_folder.clone();
          } else {
            game_config.version = None;
            game_config.version_folder = None;
          }
        }
        "installed_version" => game_config.version = val.as_str().map(|s| s.to_string()),
        "install_version_folder" => {
          game_config.version_folder = val.as_str().map(|s| s.to_string())
        }
        "seconds_played" => game_config.seconds_played += val.as_u64().unwrap_or(0),
        _ => {
          log::error!("Key '{}' not recognized", key);
          return Err(ConfigError::Configuration("Invalid key".to_owned()));
        }
      }
    } else {
      match key {
        "opengl_requirements_met" => self.requirements.opengl = val.as_bool().unwrap_or(false),
        "avx" => self.requirements.avx = val.as_bool().unwrap_or(false),
        "bypass_requirements" => {
          self.requirements.bypass_requirements = val.as_bool().unwrap_or(false)
        }
        "active_version" => self.active_version = val.as_str().map(|s| s.to_string()),
        "active_version_folder" => self.active_version_folder = val.as_str().map(|s| s.to_string()),
        "locale" => self.locale = val.as_str().map(|s| s.to_string()),
        "check_for_latest_mod_version" => {
          self.check_for_latest_mod_version = val.as_bool().unwrap_or(true)
        }
        "auto_update_games" => self.auto_update_games = val.as_bool().unwrap_or(false),
        "delete_previous_versions" => {
          self.delete_previous_versions = val.as_bool().unwrap_or(false)
        }
        "rip_levels" => {
          self.decompiler_settings.rip_levels_enabled = val.as_bool().unwrap_or(false)
        }
        "rip_collision" => {
          self.decompiler_settings.rip_collision_enabled = val.as_bool().unwrap_or(false)
        }
        "rip_textures" => {
          self.decompiler_settings.rip_textures_enabled = val.as_bool().unwrap_or(false)
        }
        "rip_streamed_audio" => {
          self.decompiler_settings.rip_streamed_audio_enabled = val.as_bool().unwrap_or(false)
        }
        "add_mod_source" => {
          let mod_source = val.as_str().map(|s| s.to_string()).unwrap_or("".to_owned());
          if !self.mod_sources.contains(&mod_source) {
            self.mod_sources.push(mod_source);
          }
        }
        "remove_mod_source" => {
          let mod_source = val.as_str().map(|s| s.to_string()).unwrap_or("".to_owned());
          self.mod_sources.retain(|source| source != &mod_source);
        }
        _ => {
          log::error!("Key '{}' not recognized", key);
          return Err(ConfigError::Configuration("Invalid key".to_owned()));
        }
      }
    }
    self.save_config()?;
    Ok(())
  }

  pub fn get_setting_value(
    &self,
    key: &str,
    game_name: Option<String>,
  ) -> Result<Value, ConfigError> {
    if let Some(game_config) = game_name
      .as_deref()
      .and_then(|name| SupportedGame::from_str(name).ok())
      .and_then(|game| self.games.get(&game))
    {
      match key {
        "installed" => return Ok(Value::Bool(game_config.is_installed)),
        "installed_version" => return Ok(json!(game_config.version())),
        "install_version_folder" => return Ok(json!(game_config.version_folder)),
        "active_texture_packs" => return Ok(json!(game_config.active_texture_packs())),
        "seconds_played" => return Ok(json!(game_config.seconds_played)),
        "installed_mods" => return Ok(json!(game_config.mods_installed_version)),
        _ => {
          log::error!("Key '{}' not recognized", key);
          return Err(ConfigError::Configuration("Invalid key".to_owned()));
        }
      }
    } else {
      match key {
        "opengl_requirements_met" => Ok(Value::Bool(self.requirements.opengl)),
        "bypass_requirements" => Ok(Value::Bool(self.requirements.bypass_requirements)),
        "install_directory" => Ok(
          self
            .installation_dir
            .as_ref()
            .map_or(Value::Null, |v| Value::String(v.clone())),
        ),
        "active_version" => Ok(
          self
            .active_version
            .as_ref()
            .map_or(Value::Null, |v| Value::String(v.clone())),
        ),
        "active_version_folder" => Ok(
          self
            .active_version_folder
            .as_ref()
            .map_or(Value::Null, |v| Value::String(v.clone())),
        ),
        "locale" => Ok(
          self
            .locale
            .as_ref()
            .map_or(Value::Null, |v| Value::String(v.clone())),
        ),
        "mod_sources" => Ok(json!(self.mod_sources)),
        "check_for_latest_mod_version" => Ok(Value::Bool(self.check_for_latest_mod_version)),
        "proceed_after_successful_operation" => {
          Ok(Value::Bool(self.proceed_after_successful_operation))
        }
        "auto_update_games" => Ok(Value::Bool(self.auto_update_games)),
        "delete_previous_versions" => Ok(Value::Bool(self.delete_previous_versions)),
        "rip_levels" => Ok(Value::Bool(self.decompiler_settings.rip_levels_enabled)),
        "rip_collision" => Ok(Value::Bool(self.decompiler_settings.rip_collision_enabled)),
        "rip_textures" => Ok(Value::Bool(self.decompiler_settings.rip_textures_enabled)),
        "rip_streamed_audio" => Ok(Value::Bool(
          self.decompiler_settings.rip_streamed_audio_enabled,
        )),
        _ => {
          log::error!("Key '{}' not recognized", key);
          Err(ConfigError::Configuration("Invalid key".to_owned()))
        }
      }
    }
  }

  pub fn update_mods_setting_value(
    &mut self,
    key: &str,
    game_name: String,
    source_name: Option<String>,
    version_name: Option<String>,
    mod_name: Option<String>,
    texture_packs: Option<Vec<String>>,
  ) -> Result<(), ConfigError> {
    let game_config = self.get_supported_game_config_mut(&game_name)?;
    let source = source_name.unwrap_or("".to_owned());
    let version = version_name.unwrap_or("".to_owned());
    let mod_name = mod_name.unwrap_or("".to_owned());
    let texture_packs = texture_packs.unwrap_or(Vec::new());

    match key {
      "add_texture_packs" => {
        game_config.features.texture_packs = texture_packs;
      }
      "add_mod" => {
        game_config
          .mods_installed_version
          .entry(source)
          .or_insert_with(HashMap::new)
          .insert(mod_name, version);
      }
      "uninstall_mod" => {
        game_config
          .mods_installed_version
          .get_mut(&source)
          .map(|mods| mods.remove(&mod_name));
      }
      _ => todo!(),
    }

    self.save_config()?;
    Ok(())
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
    game_config
      .features
      .texture_packs
      .retain(|pack| !cleanup_list.contains(pack));
    self.save_config()?;
    Ok(())
  }

  pub fn set_game_enabled_texture_packs(
    &mut self,
    game_name: &String,
    packs: Vec<String>,
  ) -> Result<(), ConfigError> {
    let game_config = self.get_supported_game_config_mut(game_name)?;
    game_config.features.texture_packs = packs;
    self.save_config()?;
    Ok(())
  }
}
