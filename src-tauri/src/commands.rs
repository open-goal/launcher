use fs_extra::dir::copy;
use serde::{Deserialize, Serialize};
use tauri::command;
use tauri::Manager;

use crate::util::open_dir_in_os;

pub mod config;
pub mod extractor;
pub mod game;
pub mod support;
pub mod versions;

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandError {
  ArchitectureNotx86,
  AVXNotSupported,
  Unknown,
}

#[command]
pub async fn get_highest_simd() -> Result<String, CommandError> {
  return highest_simd().await;
}

#[cfg(target_arch = "x86_64")]
async fn highest_simd() -> Result<String, CommandError> {
  if is_x86_feature_detected!("avx2") {
    return Ok("AVX2".to_string());
  } else if is_x86_feature_detected!("avx") {
    return Ok("AVX".to_string());
  } else {
    return Err(CommandError::AVXNotSupported);
  }
}

#[cfg(not(target_arch = "x86_64"))]
fn highest_simd() -> Result<String, CommandError> {
  return Err(CommandError::ArchitectureNotx86);
}

#[command]
pub fn open_dir(dir: String) {
  return open_dir_in_os(dir);
}

#[command]
pub async fn copy_dir(dir_src: String, dir_dest: String) -> bool {
  let mut options = fs_extra::dir::CopyOptions::new();
  options.copy_inside = true;
  options.overwrite = true;
  options.content_only = true;
  if let Err(_e) = copy(dir_src, dir_dest, &options) {
    return false;
  }
  return true;
}

#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) {
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  // Show main window
  window.get_window("main").unwrap().show().unwrap();
}
