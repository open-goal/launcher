use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use strum::IntoEnumIterator;
use sysinfo::{Disks, System};
use tauri::Manager;
use tempfile::NamedTempFile;
use zip::write::SimpleFileOptions;

#[cfg(windows)]
use crate::util::os::get_installed_vcc_runtime;
use crate::{
  config::{LauncherConfig, SupportedGame},
  util::zip::{
    append_dir_contents_to_zip, append_file_to_zip, check_if_zip_contains_top_level_entry,
  },
};

use super::CommandError;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GPUInfo {
  pub opengl_test_passed: bool,
  pub renderer_name: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseIntegrity {
  pub decompiler_folder_state: String,
  pub game_folder_state: String,
  pub goal_src_state: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
  pub release_integrity: ReleaseIntegrity,
  pub has_texture_packs: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerGameInfo {
  pub jak1: GameInfo,
  pub jak2: GameInfo,
  pub jak3: GameInfo,
  pub jakx: GameInfo,
}

impl PerGameInfo {
  fn get_game_info(&mut self, game_name: SupportedGame) -> &mut GameInfo {
    match game_name {
      SupportedGame::Jak1 => &mut self.jak1,
      SupportedGame::Jak2 => &mut self.jak2,
      SupportedGame::Jak3 => &mut self.jak3,
      SupportedGame::JakX => &mut self.jakx,
    }
  }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportPackage {
  #[cfg(windows)]
  pub installed_vcc_runtime: Option<String>,
  pub total_memory_megabytes: u64,
  pub cpu_name: String,
  pub cpu_vendor: String,
  pub cpu_brand: String,
  pub os_name: String,
  pub os_name_long: String,
  pub os_kernel_ver: String,
  pub disk_info: Vec<String>,
  pub gpu_info: Option<GPUInfo>,
  pub install_dir: String,
  pub game_info: PerGameInfo,
  pub launcher_version: String,
  pub active_tooling_version: String,
  pub extractor_binary_exists: bool,
  pub game_binary_exists: bool,
}

fn get_game_info_folder_diff_state(
  base_data_dir: &Path,
  base_active_version_dir: &Path,
  sub_dir: &str,
) -> String {
  // The data_dir is always guaranteed to be there (it's just the extracted tooling)
  // however, the active version folder may or may not exist (might not be installed)
  let active_version_dir = base_active_version_dir.join(sub_dir);
  if !active_version_dir.exists() {
    return "Not Installed".to_string();
  }

  let data_dir = base_data_dir.join(sub_dir);
  match dir_diff::is_different(active_version_dir, data_dir) {
    Ok(true) => "Different".to_string(),
    Ok(false) => "Match".to_string(),
    Err(_) => "Unknown".to_string(),
  }
}

fn dump_per_game_info(
  config_lock: &tokio::sync::MutexGuard<'_, LauncherConfig>,
  app_handle: &tauri::AppHandle,
  package: &mut SupportPackage,
  zip_file: &mut zip::ZipWriter<&std::fs::File>,
  install_path: &Path,
  game_name: SupportedGame,
) -> anyhow::Result<()> {
  let game = game_name.to_string();
  // Save OpenGOAL config folder (this includes saves and settings)
  let game_config_dir = app_handle.path().config_dir()?.join("OpenGOAL").join(&game);

  append_dir_contents_to_zip(
    zip_file,
    &game_config_dir.join("settings"),
    &format!("Game Settings and Saves/{game}/settings"),
    vec!["gc", "json"],
  )?;

  append_dir_contents_to_zip(
    zip_file,
    &game_config_dir.join("misc"),
    &format!("Game Settings and Saves/{game}/misc"),
    vec!["gc", "json"],
  )?;

  append_dir_contents_to_zip(
    zip_file,
    &game_config_dir.join("saves"),
    &format!("Game Settings and Saves/{game}/saves"),
    vec!["bin"],
  )?;

  // Save Logs
  let active_game_data_dir = install_path.join("active").join(&game).join("data");
  let game_log_dir = &active_game_data_dir.join("log");

  append_dir_contents_to_zip(
    zip_file,
    &game_log_dir,
    &format!("Game Logs and ISO Info/{game}"),
    vec!["log", "json", "txt"],
  )?;

  let texture_repl_dir = active_game_data_dir.join("texture_replacements");

  package.game_info.get_game_info(game_name).has_texture_packs = fs::read_dir(&texture_repl_dir)
    .map(|mut it| it.next().is_some())
    .unwrap_or(false);

  let build_info_path = active_game_data_dir
    .join("iso_data")
    .join(&game)
    .join("buildinfo.json");

  append_file_to_zip(
    zip_file,
    &build_info_path,
    &format!("Game Logs and ISO Info/{game}/buildinfo.json"),
  )?;

  if let Some(active_version) = &config_lock.active_version {
    let version_data_dir = install_path
      .join("versions")
      .join("official")
      .join(active_version)
      .join("data");

    let game_info = package.game_info.get_game_info(game_name);
    game_info.release_integrity.decompiler_folder_state =
      get_game_info_folder_diff_state(&version_data_dir, &active_game_data_dir, "decompiler");
    game_info.release_integrity.game_folder_state =
      get_game_info_folder_diff_state(&version_data_dir, &active_game_data_dir, "game");
    game_info.release_integrity.goal_src_state =
      get_game_info_folder_diff_state(&version_data_dir, &active_game_data_dir, "goal_src");
  }

  // Append mod settings and logs
  let mods_dir = install_path.join("features").join(&game).join("mods");
  info!("Scanning mod directory {mods_dir:?}");

  let Ok(mod_sources) = fs::read_dir(&mods_dir) else {
    return Ok(());
  };

  for source_entry in mod_sources {
    let source_entry = match source_entry {
      Ok(entry) => entry,
      Err(_) => continue,
    };

    let source_path = source_entry.path();
    if !source_path.is_dir() {
      continue;
    }

    let source_name = source_path
      .file_name()
      .map(|s| s.to_string_lossy().into_owned())
      .unwrap_or_else(|| "unknown".to_string());

    let Ok(mods) = fs::read_dir(&source_path) else {
      continue;
    };

    for mod_entry in mods {
      let mod_entry = match mod_entry {
        Ok(entry) => entry,
        Err(_) => continue,
      };

      let mod_path = mod_entry.path();
      if !mod_path.is_dir() {
        continue;
      }

      let folder_name = match mod_path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => continue,
      };

      if folder_name == "_settings" {
        append_dir_contents_to_zip(
          zip_file,
          &mod_path,
          &format!("Game Settings and Saves/{game}/mods/{source_name}/settings"),
          vec!["gc", "json"],
        )?;

        append_dir_contents_to_zip(
          zip_file,
          &mod_path,
          &format!("Game Settings and Saves/{game}/mods/{source_name}/saves"),
          vec!["bin"],
        )?;
        continue;
      }

      let mod_log_dir = mod_path.join("data").join("log");
      if !mod_log_dir.exists() {
        continue;
      }

      append_dir_contents_to_zip(
        zip_file,
        &mod_log_dir,
        &format!("Game Logs and ISO Info/{game}/mods/{source_name}/{folder_name}/logs"),
        vec!["log", "json", "txt"],
      )?;
    }
  }

  Ok(())
}

#[tauri::command]
pub async fn generate_support_package(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  user_path: String,
) -> Result<(), CommandError> {
  let mut package = SupportPackage::default();
  let config_lock = config.lock().await;
  let install_path = &config_lock.install_dir()?;

  // System Information
  let mut system_info = System::new_all();
  system_info.refresh_all();
  #[cfg(windows)]
  {
    package.installed_vcc_runtime = get_installed_vcc_runtime().ok().map(|v| v.to_string());
  }
  package.total_memory_megabytes = system_info.total_memory() / 1024 / 1024;
  package.cpu_name = system_info.cpus()[0].name().to_string();
  package.cpu_vendor = system_info.cpus()[0].vendor_id().to_string();
  package.cpu_brand = system_info.cpus()[0].brand().to_string();
  package.os_name = System::os_version().unwrap_or_else(|| "unknown".to_string());
  package.os_name_long = System::long_os_version().unwrap_or_else(|| "unknown".to_string());
  package.os_kernel_ver = System::kernel_version().unwrap_or_else(|| "unknown".to_string());
  package.launcher_version = app_handle.package_info().version.to_string();
  package.active_tooling_version = config_lock
    .active_version
    .clone()
    .unwrap_or("unset".to_string());
  package.install_dir = install_path.to_string_lossy().to_string();

  if let Some(active_version) = &config_lock.active_version {
    #[cfg(windows)]
    {
      package.extractor_binary_exists = install_path
        .join("versions")
        .join("official")
        .join(active_version)
        .join("extractor.exe")
        .exists();
      package.game_binary_exists = install_path
        .join("versions")
        .join("official")
        .join(active_version)
        .join("gk.exe")
        .exists();
    }

    #[cfg(not(windows))]
    {
      package.extractor_binary_exists = install_path
        .join("versions")
        .join("official")
        .join(active_version)
        .join("extractor")
        .exists();
      package.game_binary_exists = install_path
        .join("versions")
        .join("official")
        .join(active_version)
        .join("gk")
        .exists();
    }
  }

  for disk in Disks::new_with_refreshed_list().into_iter() {
    package.disk_info.push(format!(
      "{} | Name - {} | Capacity - {}GB/{}GB",
      disk.mount_point().to_string_lossy(),
      disk.name().to_string_lossy(),
      disk.available_space() / 1024 / 1024 / 1024,
      disk.total_space() / 1024 / 1024 / 1024
    ))
  }

  package.gpu_info = crate::util::game_tests::run_game_gpu_test(&config_lock, &app_handle)
    .await
    .ok()
    .map(|result| GPUInfo {
      opengl_test_passed: result.success,
      renderer_name: format!(
        "{}:{}",
        result
          .gpu_renderer_string
          .as_deref()
          .unwrap_or("unknown renderer"),
        result
          .gpu_vendor_string
          .as_deref()
          .unwrap_or("unknown vendor")
      ),
    });

  // Create zip file
  let save_path = Path::new(&user_path);
  let save_file = NamedTempFile::new().map_err(|e| {
    CommandError::Support(format!(
      "Failed to create temp file and unable to create support file: {e}"
    ))
  })?;
  let mut zip_file = zip::ZipWriter::new(save_file.as_file());

  // Save Launcher config folder
  let launcher_config_dir = app_handle.path().app_config_dir()?;
  let launcher_log_dir = app_handle.path().app_log_dir()?;

  append_dir_contents_to_zip(
    &mut zip_file,
    &launcher_log_dir,
    "Launcher Settings and Logs/logs",
    vec!["log"],
  )?;

  append_file_to_zip(
    &mut zip_file,
    &launcher_config_dir.join("settings.json"),
    "Launcher Settings and Logs/settings.json",
  )?;

  // Per Game Info
  for game in SupportedGame::iter() {
    if let Err(e) = dump_per_game_info(
      &config_lock,
      &app_handle,
      &mut package,
      &mut zip_file,
      install_path,
      game,
    ) {
      error!("Failed to dump support info for {game}: {e}");
    }
  }

  // Dump High Level Info
  let options = SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated)
    .compression_level(Some(9))
    .unix_permissions(0o755);

  zip_file
    .start_file("support-info.json", options)
    .map_err(|e| {
      CommandError::Support(format!(
        "Create high level support info entry in support package: {e}"
      ))
    })?;

  serde_json::to_writer_pretty(&mut zip_file, &package).map_err(|e| {
    CommandError::Support(format!(
      "Unable to write high-level support info to the support package: {e}"
    ))
  })?;

  zip_file
    .finish()
    .map_err(|e| CommandError::Support(format!("Unable to finalize zip file: {e}")))?;

  // Sanity check that the zip file was actually made correctly
  // TODO: really hate Result<bool> get rid of it
  let info_found =
    check_if_zip_contains_top_level_entry(&save_file, "support-info.json").map_err(|e| {
      CommandError::Support(format!(
        "Support package was unable to be written properly: {e}"
      ))
    })?;
  if !info_found {
    return Err(CommandError::Support(
      "Support package was unable to be written properly".to_owned(),
    ));
  }
  // Seems good, move it to the user's intended destination
  fs::copy(save_file.path(), save_path).map_err(|e| {
    CommandError::Support(format!(
      "Support package was unable to be moved from its temporary file location: {e}"
    ))
  })?;

  fs::remove_file(save_file.path()).map_err(|e| {
    CommandError::Support(format!(
      "Support package was copied but the original file could not be removed: {e}"
    ))
  })?;
  Ok(())
}
