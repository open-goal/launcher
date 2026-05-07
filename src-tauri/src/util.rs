pub mod file;
pub mod game_milestones;
pub mod game_tests;
pub mod network;
pub mod os;
pub mod process;
pub mod tar;
pub mod zip;

use crate::TAURI_APP;
use anyhow::{Context, Result};
use tauri::Emitter;

pub fn emit_config_saved() -> Result<()> {
  TAURI_APP
    .get()
    .context("Can't access global app state")?
    .emit("config:saved", ())?;
  Ok(())
}
