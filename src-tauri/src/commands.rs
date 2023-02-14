use fs_extra::dir::copy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use tauri::command;
use tauri::Manager;

use crate::config::config::LauncherConfig;

#[tauri::command]
pub fn get_install_directory(config: tauri::State<Mutex<LauncherConfig>>) -> Option<String> {
  // Ideally we'd want to use MutexGuard but that doesn't sit nicely with tauri's commands
  // Instead, we manage the lock ourselves
  // TODO - switch to tokio and async functions which apparently work better
  let config_lock: std::sync::MutexGuard<LauncherConfig> = config.lock().unwrap();
  match config_lock.installation_dir {
    None => None,
    Some(_) => Some(config_lock.installation_dir.as_ref().unwrap().to_string()),
  }
}

#[tauri::command]
pub fn set_install_directory(config: tauri::State<Mutex<LauncherConfig>>, new_dir: String) {
  let mut config_lock: std::sync::MutexGuard<LauncherConfig> = config.lock().unwrap();
  config_lock.set_install_directory(new_dir);
}

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
  return open_appdir(dir);
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

#[cfg(target_os = "windows")]
fn open_appdir(dir: String) {
  println!("Opening directory");
  Command::new("explorer")
    .arg(dir) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}

#[cfg(target_os = "linux")]
fn open_appdir(dir: String) {
  println!("Opening directory");
  Command::new("xdg-open")
    .arg(dir) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}

#[cfg(target_os = "macos")]
fn open_appdir(dir: String) {
  println!("Opening directory");
  Command::new("open")
    .arg(dir) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
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

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn open_repl(proj_path: PathBuf, curr_dir: PathBuf) {
  tauri::async_runtime::spawn(async move {
    use std::process::Command as StdCommand;
    let repl = StdCommand::new("cmd.exe")
      .args([
        "/K",
        "start",
        "goalc",
        "--proj-path",
        proj_path.to_str().as_ref().unwrap(),
      ])
      .current_dir(curr_dir)
      .spawn()
      .unwrap();
  });
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn open_repl(proj_path: PathBuf, curr_dir: PathBuf) {
  tauri::async_runtime::spawn(async move {
    use tauri::api::process::Command;
    let tauri_cmd = Command::new_sidecar("goalc")
      .unwrap()
      .current_dir(curr_dir)
      .args(["--proj-path", proj_path.to_str().as_ref().unwrap()])
      .spawn();
  });
}
