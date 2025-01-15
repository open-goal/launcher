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

#[cfg(not(target_os = "windows"))]
pub fn get_installed_vcc_runtime() -> Option<semver::Version> {
  None
}

#[cfg(target_os = "windows")]
pub fn get_installed_vcc_runtime() -> Option<semver::Version> {
  use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_READ},
    RegKey,
  };
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let path = r"SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64";

  if let Ok(key) = hklm.open_subkey_with_flags(path, KEY_READ) {
    match key.get_value::<u32, _>("Installed") {
      Ok(val) => {
        if val != 1 {
          log::error!("VCC runtime exists in the registry but is not marked as installed");
          return None;
        }
      }
      Err(err) => {
        log::error!("Couldn't determine if VCC runtime was installed: {}", err);
        return None;
      }
    };
    let patch_version: u32 = match key.get_value("Bld") {
      Ok(val) => val,
      Err(err) => {
        log::error!(
          "Couldn't determine installed VCC runtime patch version: {}",
          err
        );
        return None;
      }
    };
    let minor_version: u32 = match key.get_value("Minor") {
      Ok(val) => val,
      Err(err) => {
        log::error!(
          "Couldn't determine installed VCC runtime minor version: {}",
          err
        );
        return None;
      }
    };
    let major_version: u32 = match key.get_value("Major") {
      Ok(val) => val,
      Err(err) => {
        log::error!(
          "Couldn't determine installed VCC runtime major version: {}",
          err
        );
        return None;
      }
    };
    let installed_version = semver::Version::new(
      major_version.into(),
      minor_version.into(),
      patch_version.into(),
    );
    log::info!("Detected VCC Runtime: {major_version}.{minor_version}.{patch_version}");
    return Some(installed_version);
  }
  return None;
}
