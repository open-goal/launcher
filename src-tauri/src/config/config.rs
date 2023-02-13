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

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LauncherConfig {
  #[serde(default = "default_version")]
  pub version: String,
  pub test: i32
}

fn default_version() -> String {
  "2.0.0".to_string()
}

impl LauncherConfig {
  fn default() -> Self {
    Self { version: default_version(), test: 0 }
  }

  pub fn init_config(config_dir: Option<std::path::PathBuf>) -> LauncherConfig {
    println!("HEY!");
    match config_dir {
      Some(config_dir) => {
        println!("{}", config_dir.display());
        LauncherConfig::default()
      }
      None => {
        println!("sad!");
        LauncherConfig::default()
      }
    }
  }
}
