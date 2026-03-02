#[cfg(not(target_os = "windows"))]
pub fn get_installed_vcc_runtime() -> anyhow::Result<semver::Version> {
  anyhow::bail!("VCC runtime is only available on Windows");
}

#[cfg(target_os = "windows")]
pub fn get_installed_vcc_runtime() -> anyhow::Result<semver::Version> {
  use anyhow::Context;
  use winreg::{
    RegKey,
    enums::{HKEY_LOCAL_MACHINE, KEY_READ},
  };
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let path = r"SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64";
  let key = hklm
    .open_subkey_with_flags(path, KEY_READ)
    .context("opening VCC runtime registry key")?;

  let installed: u32 = key
    .get_value("Installed")
    .context("reading VCC Installed flag")?;
  if installed != 1 {
    anyhow::bail!("VCC runtime is not installed (Installed flag is not 1)");
  }

  let major: u32 = key.get_value("Major").context("reading VCC Major")?;
  let minor: u32 = key.get_value("Minor").context("reading VCC Minor")?;
  let patch: u32 = key.get_value("Bld").context("reading VCC Bld")?;

  Ok(semver::Version::new(
    major as u64,
    minor as u64,
    patch as u64,
  ))
}
