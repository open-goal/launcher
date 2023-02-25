use std::io::Cursor;
use std::path::PathBuf;
use std::{
  fs::File,
  io::{Read, Write},
  path::Path,
};
use walkdir::WalkDir;
use zip::write::FileOptions;

pub fn append_dir_contents_to_zip(
  zip_file: &mut zip::ZipWriter<File>,
  dir: &Path,
  internal_folder: &str,
) -> zip::result::ZipResult<()> {
  if !dir.exists() {
    return Result::Ok(());
  }

  let iter = WalkDir::new(dir).into_iter().filter_map(|e| e.ok());

  let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::DEFLATE)
    .unix_permissions(0o755);

  let mut buffer = Vec::new();
  for entry in iter {
    let path = entry.path();
    let temp_name = path.strip_prefix(dir.clone()).unwrap();
    let name = Path::new(internal_folder).join(temp_name);

    // Write file or directory explicitly
    // Some unzip tools unzip files with directory paths correctly, some do not!
    if path.is_file() {
      println!("adding file {path:?} as {name:?} ...");
      #[allow(deprecated)]
      zip_file.start_file_from_path(&name, options)?;
      let mut f = File::open(path)?;

      f.read_to_end(&mut buffer)?;
      zip_file.write_all(&buffer)?;
      buffer.clear();
    } else if !name.as_os_str().is_empty() {
      // Only if not root! Avoids path spec / warning
      // and mapname conversion failed error on unzip
      println!("adding dir {path:?} as {name:?} ...");
      #[allow(deprecated)]
      zip_file.add_directory_from_path(&name, options)?;
    }
  }
  Result::Ok(())
}

pub fn append_file_to_zip(
  zip_file: &mut zip::ZipWriter<File>,
  src: &Path,
  path_in_zip: &str,
) -> zip::result::ZipResult<()> {
  if !src.exists() || src.is_dir() {
    println!("doesnt exist");
    Result::<(), ()>::Err(());
  }

  let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::DEFLATE)
    .unix_permissions(0o755);

  let mut buffer = Vec::new();
  let name = Path::new(path_in_zip);
  #[allow(deprecated)]
  zip_file.start_file_from_path(&name, options)?;
  let mut f = File::open(src)?;

  f.read_to_end(&mut buffer)?;
  zip_file.write_all(&buffer)?;
  buffer.clear();

  Result::Ok(())
}

pub fn extract_and_delete_zip_file(
  zip_path: &PathBuf,
  extract_dir: &PathBuf,
) -> Result<(), zip_extract::ZipExtractError> {
  let archive: Vec<u8> = std::fs::read(zip_path)?;
  zip_extract::extract(Cursor::new(archive), extract_dir, true)?;
  std::fs::remove_file(zip_path)?;
  Result::Ok(())
}
