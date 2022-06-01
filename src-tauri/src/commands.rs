use std::process::Command;
use tauri::command;

#[derive(Debug, serde::Serialize)]
pub enum CommandError {
  ArchitectureNotx86,
  AVXNotSupported,
  Unknown,
}

#[command]
pub fn get_highest_simd() -> Result<String, CommandError> {
  return highest_simd();
}

#[cfg(target_arch = "x86_64")]
fn highest_simd() -> Result<String, CommandError> {
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
pub fn open__dir(dir: String) {
  return open__appdir(dir);
}

#[cfg(target_os = "windows")]
fn open__appdir(dir: String) {
  println!("Opening directory");
  Command::new("explorer")
    .arg(dir) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}

#[cfg(target_os = "linux")]
fn open__appdir(dir: String) {
  println!("Opening directory");
  Command::new("xdg-open")
    .arg(dir) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}

#[cfg(target_os = "macos")]
fn open__appdir(dir: String) {
  println!("Opening directory");
  Command::new("open")
    .arg(dir) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}
