use std::path::PathBuf;

pub fn extract_tar_ball(
  tar_path: &PathBuf,
  extract_dir: &PathBuf,
) -> Result<(), std::io::Error> {
  log::info!("extracting {}", tar_path.display());
  let tar_gz = std::fs::File::open(tar_path)?;
  let tar = flate2::read::GzDecoder::new(tar_gz);
  let mut archive = tar::Archive::new(tar);
  archive.unpack(extract_dir)?;
  Ok(())
}

pub fn extract_and_delete_tar_ball(
  tar_path: &PathBuf,
  extract_dir: &PathBuf,
) -> Result<(), std::io::Error> {
  extract_tar_ball(tar_path, extract_dir)?;
  log::info!("deleting {}", tar_path.display());
  std::fs::remove_file(tar_path)?;
  Ok(())
}
