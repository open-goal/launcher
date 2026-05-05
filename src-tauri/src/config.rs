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
use crate::{commands::CommandError, util::file::delete_dir};
use anyhow::Context;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use strum_macros::{Display, EnumIter};
use ts_rs::TS;

use crate::util::file::touch_file;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  JSONError(#[from] serde_json::Error),
  #[error("{0}")]
  Configuration(String),
}

#[derive(
  Debug, Eq, PartialEq, Hash, Clone, Copy, Serialize, Deserialize, Display, EnumIter, TS,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SupportedGame {
  Jak1,
  Jak2,
  Jak3,
  JakX,
}

impl SupportedGame {
  pub fn required_diskspace(&self) -> u64 {
    match self {
      SupportedGame::Jak1 => 4 * 1024 * 1024 * 1024,  // 4 GB
      SupportedGame::Jak2 => 11 * 1024 * 1024 * 1024, // 11 GB
      SupportedGame::Jak3 => 11 * 1024 * 1024 * 1024, // 11 GB
      SupportedGame::JakX => 11 * 1024 * 1024 * 1024, // TODO
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, TS)]
#[serde(rename_all = "camelCase", default)]
pub struct GameConfig {
  pub is_installed: bool,
  pub version: Option<String>,
  pub texture_packs: Vec<String>,
  pub seconds_played: u64,
  #[serde(rename = "mods")]
  pub mods_installed_version: HashMap<String, HashMap<String, String>>,
}

impl GameConfig {
  pub fn active_texture_packs(&self) -> Vec<String> {
    self.texture_packs.clone()
  }

  pub fn version(&self) -> Option<String> {
    self.version.clone()
  }

  pub fn has_installed_mod(&self, source: &str, mod_name: &str) -> bool {
    self
      .mods_installed_version
      .get(source)
      .is_some_and(|mods| mods.contains_key(mod_name))
  }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, TS)]
#[serde(rename_all = "camelCase", default)]
pub struct Requirements {
  pub bypass_requirements: bool,
  pub avx: bool,
  #[serde(rename = "openGL")]
  pub opengl: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, TS)]
#[serde(rename_all = "camelCase", default)]
pub struct DecompilerSettings {
  pub rip_levels_enabled: bool,
  pub rip_collision_enabled: bool,
  pub rip_textures_enabled: bool,
  pub rip_streamed_audio_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, TS)]
#[serde(rename_all = "camelCase", default)]
#[ts(export)]
pub struct LauncherConfig {
  #[serde(skip)]
  #[ts(skip)]
  settings_path: PathBuf,
  #[serde(default = "default_version")]
  pub version: String,
  pub requirements: Requirements,
  #[serde(default = "default_games")]
  pub games: HashMap<SupportedGame, GameConfig>,
  pub installation_dir: Option<PathBuf>,
  pub active_version: Option<String>,
  pub locale: Option<String>,
  pub mod_sources: Vec<String>,
  pub decompiler_settings: DecompilerSettings,
  pub check_for_latest_mod_version: bool,
  pub proceed_after_successful_operation: bool,
  pub auto_update_games: bool,
  pub delete_previous_versions: bool,
}

pub struct CommonConfigData {
  pub install_path: PathBuf,
  pub active_version: String,
  #[allow(dead_code)]
  pub tooling_version: Version,
}

impl CommonConfigData {
  pub fn get_exec_location(&self, executable_name: &str) -> ExecutableLocation {
    let exec_dir = self
      .install_path
      .join("versions")
      .join("official")
      .join(&self.active_version);

    let mut exec_path: PathBuf = exec_dir.join(executable_name);
    if cfg!(windows) {
      exec_path.set_extension("exe");
    }

    ExecutableLocation {
      executable_dir: exec_dir,
      executable_path: exec_path,
    }
  }
}

pub struct ExecutableLocation {
  pub executable_dir: PathBuf,
  pub executable_path: PathBuf,
}

fn default_version() -> String {
  "3.0".to_owned()
}

fn default_games() -> HashMap<SupportedGame, GameConfig> {
  HashMap::from([
    (SupportedGame::Jak1, GameConfig::default()),
    (SupportedGame::Jak2, GameConfig::default()),
    (SupportedGame::Jak3, GameConfig::default()),
    (SupportedGame::JakX, GameConfig::default()),
  ])
}

impl LauncherConfig {
  fn default_with_path(_settings_path: PathBuf) -> Self {
    Self {
      settings_path: _settings_path,
      version: default_version(),
      requirements: Requirements::default(),
      games: default_games(),
      installation_dir: None,
      active_version: None,
      locale: None,
      mod_sources: Vec::new(),
      decompiler_settings: DecompilerSettings::default(),
      check_for_latest_mod_version: true,
      proceed_after_successful_operation: true,
      auto_update_games: false,
      delete_previous_versions: false,
    }
  }

  fn backup(settings_path: &PathBuf) {
    tracing::warn!("Creating a backup copy of existing settings.");
    let dest = settings_path.with_file_name("settings.backup.json");
    let _ = fs::copy(settings_path.clone(), dest);
  }

  fn get_supported_game_config_mut(
    &mut self,
    game_name: SupportedGame,
  ) -> Result<&mut GameConfig, ConfigError> {
    self.games.get_mut(&game_name).ok_or_else(|| {
      tracing::error!("Game not found or unsupported: {}", game_name);
      ConfigError::Configuration(format!("Game not found or unsupported: {game_name}"))
    })
  }

  pub fn load_config(config_dir: std::path::PathBuf) -> LauncherConfig {
    let settings_path = config_dir.join("settings.json");
    tracing::info!("Loading configuration at path: {}", settings_path.display());

    let file = std::fs::File::open(&settings_path);
    let mut config: LauncherConfig = match file {
      Ok(file) => serde_json::from_reader(file).unwrap_or_else(|_| {
        tracing::warn!("Failed to load or parse settings file, using defaults");
        LauncherConfig::backup(&settings_path);
        LauncherConfig::default_with_path(settings_path.clone())
      }),
      Err(_) => LauncherConfig::default_with_path(settings_path.clone()),
    };

    config.settings_path = settings_path;
    return config;
  }

  pub fn save_config(&self) -> Result<(), ConfigError> {
    let settings_path = &self.settings_path;

    // Ensure the directory exists
    create_dir(&settings_path.parent().unwrap())?;
    let file = fs::File::create(settings_path)?;
    serde_json::to_writer_pretty(file, &self)?;
    Ok(())
  }

  pub fn reset_to_defaults(&mut self) -> Result<(), ConfigError> {
    let original_installation_dir = self.installation_dir.clone();
    *self = Self::default_with_path(self.settings_path.clone());
    self.installation_dir = original_installation_dir;
    Self::save_config(self)?;
    Ok(())
  }

  pub fn set_install_directory(&mut self, path: PathBuf) -> Result<(), ConfigError> {
    // Do some tests on this folder, if they fail, return a decent error
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
    touch_file(&test_file)
      .with_context(|| format!("Provided installation folder could not be written to."))?;

    // If the directory changes (it's not a no-op), we need to:
    // - wipe any installed games (make them reinstall)
    // - wipe the active version/version types
    if let Some(old_dir) = &self.installation_dir {
      if *old_dir != path {
        self.active_version = None;
        self.update_setting_value("installed", false.into(), Some(SupportedGame::Jak1))?;
        self.update_setting_value("installed", false.into(), Some(SupportedGame::Jak2))?;
        self.update_setting_value("installed", false.into(), Some(SupportedGame::Jak3))?;
        self.update_setting_value("installed", false.into(), Some(SupportedGame::JakX))?;
      }
    }

    self.installation_dir = Some(path);
    self.save_config()?;
    Ok(())
  }

  pub fn update_setting_value(
    &mut self,
    key: &str,
    val: Value,
    game_name: Option<SupportedGame>,
  ) -> Result<(), ConfigError> {
    if let Some(game_config) = game_name.and_then(|game| self.games.get_mut(&game)) {
      match key {
        "installed" => {
          let installed = val.as_bool().unwrap_or(false);
          game_config.is_installed = installed;
          if installed {
            game_config.version = self.active_version.clone();
          } else {
            game_config.version = None;
          }
        }
        "installed_version" => game_config.version = val.as_str().map(|s| s.to_string()),
        "seconds_played" => game_config.seconds_played += val.as_u64().unwrap_or(0),
        _ => {
          tracing::error!("Key '{}' not recognized", key);
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
          tracing::error!("Key '{}' not recognized", key);
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
    game_name: Option<SupportedGame>,
  ) -> Result<Value, ConfigError> {
    if let Some(game_config) = game_name.and_then(|game| self.games.get(&game)) {
      match key {
        "installed" => Ok(Value::Bool(game_config.is_installed)),
        "installed_version" => Ok(json!(game_config.version())),
        "active_texture_packs" => Ok(json!(game_config.active_texture_packs())),
        "seconds_played" => Ok(json!(game_config.seconds_played)),
        "installed_mods" => Ok(json!(game_config.mods_installed_version)),
        _ => {
          tracing::error!("Key '{}' not recognized in game config", key);
          Err(ConfigError::Configuration("Invalid key".to_owned()))
        }
      }
    } else {
      match key {
        "opengl_requirements_met" => Ok(Value::Bool(self.requirements.opengl)),
        "bypass_requirements" => Ok(Value::Bool(self.requirements.bypass_requirements)),
        "install_directory" => Ok(self.installation_dir.as_ref().map_or(Value::Null, |v| {
          Value::String(v.to_string_lossy().into_owned())
        })),
        "active_version" => Ok(
          self
            .active_version
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
          tracing::error!("Key '{}' not recognized", key);
          Err(ConfigError::Configuration("Invalid key".to_owned()))
        }
      }
    }
  }

  pub fn update_mods_setting_value(
    &mut self,
    key: &str,
    game_name: SupportedGame,
    source_name: Option<String>,
    version_name: Option<String>,
    mod_name: Option<String>,
    texture_packs: Option<Vec<String>>,
  ) -> Result<(), ConfigError> {
    let game_config = self.get_supported_game_config_mut(game_name)?;
    let source = source_name.unwrap_or("".to_owned());
    let version = version_name.unwrap_or("".to_owned());
    let mod_name = mod_name.unwrap_or("".to_owned());
    let texture_packs = texture_packs.unwrap_or_default();

    match key {
      "add_texture_packs" => {
        game_config.texture_packs = texture_packs;
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
    game_name: SupportedGame,
    cleanup_list: Vec<String>,
  ) -> Result<(), ConfigError> {
    if !cleanup_list.is_empty() {
      return Ok(());
    }
    let game_config = self.get_supported_game_config_mut(game_name)?;
    game_config
      .texture_packs
      .retain(|pack| !cleanup_list.contains(pack));
    self.save_config()?;
    Ok(())
  }

  pub fn install_dir(&self) -> anyhow::Result<PathBuf> {
    self
      .installation_dir
      .as_ref()
      .cloned()
      .ok_or_else(|| anyhow::anyhow!("No installation directory set"))
  }

  pub fn common_prelude(&self) -> Result<CommonConfigData, CommandError> {
    let install_path = self.installation_dir.as_ref().ok_or_else(|| {
      CommandError::BinaryExecution(
        "No installation directory set, can't perform operation".to_owned(),
      )
    })?;

    let active_version = self.active_version.as_ref().ok_or_else(|| {
      CommandError::BinaryExecution("No active version set, can't perform operation".to_owned())
    })?;

    let version_str = active_version.strip_prefix('v').unwrap_or(active_version);

    let tooling_version = Version::parse(version_str).unwrap_or_else(|_| Version::new(0, 1, 35));

    Ok(CommonConfigData {
      install_path: install_path.clone(),
      active_version: active_version.clone(),
      tooling_version,
    })
  }

  pub fn ensure_active_version_still_exists(&mut self) -> anyhow::Result<bool> {
    let Some(active_version) = &self.active_version else {
      return Ok(false);
    };

    let version_dir = self
      .install_dir()?
      .join("versions")
      .join("official")
      .join(active_version);

    if version_dir.exists() {
      return Ok(true);
    }

    self.update_setting_value("active_version", serde_json::Value::Null, None)?;
    Ok(false)
  }

  pub fn remove_version(&mut self, version: &str) -> anyhow::Result<()> {
    let version_dir = self
      .install_dir()?
      .join("versions")
      .join("official")
      .join(version);

    delete_dir(&version_dir)?;

    if self.active_version.as_deref() == Some(version) {
      self.update_setting_value("active_version", serde_json::Value::Null, None)?;
    }

    Ok(())
  }

  pub fn list_downloaded_versions(&self, folder: &str) -> anyhow::Result<Vec<String>> {
    let dir = self.install_dir()?.join("versions").join(folder);

    Ok(
      std::fs::read_dir(&dir)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect(),
    )
  }

  pub fn is_mod_installed(&self, game: SupportedGame, source: &str, mod_name: &str) -> bool {
    self
      .games
      .get(&game)
      .is_some_and(|config| config.has_installed_mod(source, mod_name))
  }
}
