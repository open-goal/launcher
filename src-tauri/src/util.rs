use std::process::Command;

#[cfg(target_os = "windows")]
pub fn open_dir_in_os(dir: String) {
  Command::new("explorer")
    .arg(dir) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}

#[cfg(target_os = "linux")]
pub fn open_dir_in_os(dir: String) {
  Command::new("xdg-open")
    .arg(dir) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}

#[cfg(target_os = "macos")]
pub fn open_dir_in_os(dir: String) {
  Command::new("open")
    .arg(dir) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}
