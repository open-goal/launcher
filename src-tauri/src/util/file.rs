extern crate rustc_serialize;

use anyhow::{Context, Result};
use rustc_serialize::base64::{MIME, ToBase64};
use rustc_serialize::hex::ToHex;
use std::{fs::File, io::Read, path::Path};

pub fn delete_dir(path: impl AsRef<Path>) -> Result<()> {
  let path = path.as_ref();
  if path.is_dir() {
    std::fs::remove_dir_all(path)
      .with_context(|| format!("Failed to delete directory: {}", path.display()))?;
  }
  Ok(())
}

pub fn create_dir(path: &Path) -> Result<()> {
  std::fs::create_dir_all(path)
    .with_context(|| format!("Failed to create directory: {}", path.display()))
}

pub fn overwrite_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
  let src = src.as_ref();
  let dst = dst.as_ref();
  if src.exists() {
    let options = fs_extra::dir::CopyOptions::new()
      .copy_inside(true)
      .overwrite(true)
      .content_only(true);
    fs_extra::dir::copy(src, dst, &options).with_context(|| {
      format!(
        "Unable to copy directory from {} to {}",
        src.display(),
        dst.display()
      )
    })?;
  }
  Ok(())
}

pub fn touch_file(path: impl AsRef<Path>) -> Result<()> {
  let path = path.as_ref();
  std::fs::OpenOptions::new()
    .create(true)
    .truncate(true)
    .write(true)
    .open(path)
    .with_context(|| format!("Failed to touch file: {}", path.display()))?;
  Ok(())
}

// TODO: come back to these last two functions later
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
