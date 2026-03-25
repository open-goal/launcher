use anyhow::{Context, Result};
use log::info;
use std::io::{BufReader, Cursor};
use std::{
  fs::File,
  io::{Read, Write},
  path::Path,
};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

pub fn append_dir_contents_to_zip(
  zip_file: &mut zip::ZipWriter<&File>,
  dir: &Path,
  internal_folder: &str,
  allowed_extensions: Vec<&str>,
) -> Result<()> {
  if !dir.exists() {
    return Ok(());
  }

  let iter = WalkDir::new(dir).into_iter().filter_map(|e| e.ok());

  let options = SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated)
    .compression_level(Some(9))
    .unix_permissions(0o755);

  let mut buffer = Vec::new();
  for entry in iter {
    let path = entry.path();
    let temp_name = path.strip_prefix(dir).unwrap();
    let name = Path::new(internal_folder).join(temp_name);

    // Write file or directory explicitly
    // Some unzip tools unzip files with directory paths correctly, some do not!
    if path.is_file() {
      let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("unknown");

      if !allowed_extensions.contains(&extension) {
        // Skip files that we don't care about
        log::warn!("skipping {path:?} - extension {extension}");
        continue;
      }

      log::debug!("adding file {path:?} as {name:?} ...");
      #[allow(deprecated)]
      zip_file.start_file_from_path(&name, options)?;
      let mut f = File::open(path)?;

      f.read_to_end(&mut buffer)?;
      zip_file.write_all(&buffer)?;
      buffer.clear();
    } else if !name.as_os_str().is_empty() {
      // Only if not root! Avoids path spec / warning
      // and mapname conversion failed error on unzip
      log::debug!("adding dir {path:?} as {name:?} ...");
      #[allow(deprecated)]
      zip_file.add_directory_from_path(&name, options)?;
    }
  }
  Ok(())
}

pub fn append_file_to_zip(
  zip_file: &mut zip::ZipWriter<&File>,
  src: &Path,
  path_in_zip: &str,
) -> Result<()> {
  if !src.exists() || src.is_dir() {
    log::warn!("'{}', doesnt exist", src.display());
    return Ok(());
  }

  let options = SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated)
    .compression_level(Some(9))
    .unix_permissions(0o755);

  let mut buffer = Vec::new();
  let name = Path::new(path_in_zip);
  #[allow(deprecated)]
  zip_file.start_file_from_path(name, options)?;
  let mut f = File::open(src)?;

  f.read_to_end(&mut buffer)?;
  zip_file.write_all(&buffer)?;
  buffer.clear();

  Ok(())
}

pub fn extract_zip_file(
  zip_path: impl AsRef<Path>,
  extract_dir: &Path,
  strip_top_dir: bool,
) -> Result<()> {
  let archive: Vec<u8> = std::fs::read(zip_path)?;
  zip_extract::extract(Cursor::new(archive), extract_dir, strip_top_dir)?;
  Ok(())
}

pub fn extract_and_delete_zip_file(
  zip_path: impl AsRef<Path>,
  extract_dir: &Path,
  strip_top_dir: bool,
) -> Result<()> {
  extract_zip_file(&zip_path, extract_dir, strip_top_dir)?;
  std::fs::remove_file(&zip_path)?;
  Ok(())
}

pub fn check_if_zip_contains_top_level_entry<P: AsRef<Path>>(
  path: P,
  expected: &str,
) -> Result<bool> {
  let file = File::open(&path)
    .with_context(|| format!("Unable to open zip file {}", path.as_ref().display()))?;
  let reader = BufReader::new(file);
  let mut zip = zip::ZipArchive::new(reader).context("Failed to read zip archive")?;

  for i in 0..zip.len() {
    let file = zip
      .by_index(i)
      .context("Failed reading entry from zip archive")?;
    info!("{}", file.name());
    if file.name().starts_with(&expected) {
      return Ok(true);
    }
  }
  anyhow::bail!(
    "Invalid zip, no top-level '{}' folder in: {}",
    expected,
    path.as_ref().display()
  );
}
