// not all util functions are used yet, allow dead code in this module
#![allow(dead_code)]
extern crate rustc_serialize;

use rustc_serialize::base64::{ToBase64, MIME};
use rustc_serialize::hex::ToHex;
use std::{
  fs::File,
  io::Read,
  path::{Path, PathBuf},
};

pub fn delete_dir<T: AsRef<Path>>(path: T) -> Result<(), std::io::Error> {
  if path.as_ref().exists() && path.as_ref().is_dir() {
    std::fs::remove_dir_all(path)?;
  }
  Ok(())
}

pub fn create_dir(path: &Path) -> Result<(), std::io::Error> {
  if path.exists() {
    return Ok(());
  }
  std::fs::create_dir_all(path)?;
  Ok(())
}

pub fn delete_file(path: &PathBuf) -> Result<(), std::io::Error> {
  if path.exists() && path.is_file() {
    std::fs::remove_file(path)?;
  }
  Ok(())
}

pub fn overwrite_dir(src: &PathBuf, dst: &PathBuf) -> Result<(), fs_extra::error::Error> {
  if src.exists() {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;
    options.content_only = true;
    fs_extra::dir::copy(src, dst, &options)?;
  }
  Ok(())
}

pub fn read_lines_in_file(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
  Ok(std::fs::read_to_string(path)?)
}

pub fn touch_file(path: &PathBuf) -> std::io::Result<()> {
  match std::fs::OpenOptions::new()
    .create(true)
    .truncate(true)
    .write(true)
    .open(path)
  {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}

pub fn get_image_file_type(hex: &str) -> &str {
  if hex.starts_with("ffd8ffe0") {
    return "jpeg";
  } else if hex.starts_with("89504e47") {
    return "png";
  } else if hex.starts_with("47494638") {
    return "gif";
  }
  panic!("invalid file type")
}

pub fn to_image_base64(path: &str) -> String {
  let mut file = File::open(path).unwrap();
  let mut vec = Vec::new();
  let _ = file.read_to_end(&mut vec);
  let base64 = vec.to_base64(MIME);
  let hex = vec.to_hex();
  format!(
    "data:image/{};base64,{}",
    get_image_file_type(&hex),
    base64.replace("\r\n", "")
  )
}
