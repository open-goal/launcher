use std::process::Command;

pub mod zip;

#[cfg(target_os = "windows")]
const FILE_OPENING_PROGRAM: &str = "explorer";
#[cfg(target_os = "linux")]
const FILE_OPENING_PROGRAM: &str = "explorer";
#[cfg(target_os = "macos")]
const FILE_OPENING_PROGRAM: &str = "explorer";

pub fn open_dir_in_os(dir: String) {
  Command::new(FILE_OPENING_PROGRAM)
    .arg(dir) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}
