use serde::{Serialize, Serializer};

pub mod binaries;
pub mod config;
pub mod game;
pub mod logging;
pub mod support;
pub mod versions;
pub mod window;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
  #[error(transparent)]
  IO(#[from] std::io::Error),
  #[error(transparent)]
  NetworkRequest(#[from] reqwest::Error),
  #[error("{0}")]
  Configuration(String),
  #[error(transparent)]
  TauriEvent(#[from] tauri::Error),
  #[error("{0}")]
  Installation(String),
  #[error("{0}")]
  VersionManagement(String),
  #[error("{0}")]
  GameManagement(String),
  #[error("{0}")]
  BinaryExecution(String),
  #[error("{0}")]
  Support(String),
}

impl Serialize for CommandError {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
