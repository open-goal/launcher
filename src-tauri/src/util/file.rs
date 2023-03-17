use std::{io::BufRead, path::PathBuf};

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

pub fn read_lines_in_file(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
  Ok(std::fs::read_to_string(path)?)
}

pub fn read_last_lines_from_file(path: &PathBuf, lines: usize) -> Result<String, std::io::Error> {
  if !path.exists() {
    return Ok("".to_owned());
  }
  let buf = rev_buf_reader::RevBufReader::new(std::fs::File::open(path)?);
  Ok(
    buf
      .lines()
      .take(lines)
      .map(|l| l.unwrap_or("".to_owned()))
      .collect::<Vec<String>>()
      .into_iter()
      .rev()
      .collect::<Vec<String>>()
      .join("\n"),
  )
}

pub fn touch_file(path: &PathBuf) -> std::io::Result<()> {
  match std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .open(path)
  {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}
