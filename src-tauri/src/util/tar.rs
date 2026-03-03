use anyhow::{Context, Result};
use std::path::Path;

use crate::util::zip::{extract_and_delete_zip_file, extract_zip_file};

pub fn extract_tar_ball(tar_path: impl AsRef<Path>, extract_dir: impl AsRef<Path>) -> Result<()> {
  let tar_path = tar_path.as_ref();
  let extract_dir = extract_dir.as_ref();
  let tar_gz = std::fs::File::open(tar_path)
    .with_context(|| format!("failed to open: {}", tar_path.display()))?;
  let tar = flate2::read::GzDecoder::new(tar_gz);
  let mut archive = tar::Archive::new(tar);
  archive.unpack(extract_dir).with_context(|| {
    format!(
      "failed to unpack: {} into {}",
      tar_path.display(),
      extract_dir.display()
    )
  })?;
  Ok(())
}

pub fn extract_and_delete_tar_ball(
  tar_path: impl AsRef<Path>,
  extract_dir: impl AsRef<Path>,
) -> Result<()> {
  let tar_path = tar_path.as_ref();
  let extract_dir = extract_dir.as_ref();
  extract_tar_ball(tar_path, extract_dir)?;
  std::fs::remove_file(tar_path)
    .with_context(|| format!("failed to delete: {}", tar_path.display()))?;
  Ok(())
}

pub fn extract_archive(archive: &Path, dest: &Path) -> Result<()> {
  match archive.extension().and_then(|e| e.to_str()) {
    Some("zip") => extract_zip_file(archive, dest, false),
    Some("gz") => extract_tar_ball(archive, dest),
    _ => anyhow::bail!("Unsupported archive format (expected .zip or .tar.gz)"),
  }
}

pub fn extract_and_delete_archive(archive: &Path, dest: &Path) -> Result<()> {
  match archive.extension().and_then(|e| e.to_str()) {
    Some("zip") => extract_and_delete_zip_file(archive, dest, false),
    Some("gz") => extract_and_delete_tar_ball(archive, dest),
    _ => anyhow::bail!("Unsupported archive format (expected .zip or .tar.gz)"),
  }
}
