use std::process::Command;

#[cfg(target_os = "windows")]
const FILE_OPENING_PROGRAM: &str = "explorer";
#[cfg(target_os = "linux")]
const FILE_OPENING_PROGRAM: &str = "xdg-open";
#[cfg(target_os = "macos")]
const FILE_OPENING_PROGRAM: &str = "open";

pub fn open_dir_in_os(dir: String) -> Result<(), std::io::Error> {
  Command::new(FILE_OPENING_PROGRAM).arg(dir).spawn()?;
  Ok(())
}
