use std::path::PathBuf;

pub fn delete_dir_or_folder(path: &PathBuf) -> Result<(), std::io::Error> {
  if path.exists() {
    if path.is_dir() {
      std::fs::remove_dir_all(path)?;
    } else {
      std::fs::remove_file(path)?;
    }
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
