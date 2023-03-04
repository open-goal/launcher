use std::path::PathBuf;

pub fn delete_dir(path: &PathBuf) -> Result<(), std::io::Error> {
  if path.exists() && path.is_dir() {
    std::fs::remove_dir_all(path)?;
  }
  Ok(())
}

pub fn create_dir(path: &PathBuf) -> Result<(), std::io::Error> {
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
