use std::path::Path;
use std::process::Command;
use std::{fs, io};
use tauri::command;
use tauri::Manager;

#[derive(Debug, serde::Serialize)]
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

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
  fs::create_dir_all(&dst)?;
  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let ty = entry.file_type()?;
    if ty.is_dir() {
      copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
    } else {
      fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
    }
  }
  Ok(())
}

#[command]
pub async fn copy_dir(dir_src: String, dir_dest: String) -> bool {
  return copy_dir_all(dir_src, dir_dest).is_ok();
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
