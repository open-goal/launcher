use serde::{Deserialize, Serialize};
use std::{
  io::{BufWriter, Write},
  path::Path,
};
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use zip::write::FileOptions;

use tauri::api::path::config_dir;

use crate::{
  config::LauncherConfig,
  util::zip::{append_dir_contents_to_zip, append_file_to_zip},
};

use super::CommandError;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GPUInfo {
  pub name: String,
  pub driver_info: String,
  pub driver_name: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseIntegrity {
  pub decompiler_folder_modified: bool,
  pub game_folder_modified: bool,
  pub goal_src_modified: bool,
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
  // TODO - switch this to enums or w/e, being lazy
  fn get_game_info(&mut self, game_name: &str) -> &mut GameInfo {
    match game_name {
      "jak1" => &mut self.jak1,
      "jak2" => &mut self.jak2,
      "jak3" => &mut self.jak3,
      "jakx" => &mut self.jakx,
      _ => &mut self.jak1,
    }
  }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportPackage {
  pub total_memory_megabytes: u64,
  pub cpu_name: String,
  pub cpu_vendor: String,
  pub cpu_brand: String,
  pub os_name: String,
  pub os_name_long: String,
  pub os_kernel_ver: String,
  pub disk_info: Vec<String>,
  pub gpu_info: Vec<GPUInfo>,
  pub game_info: PerGameInfo,
  pub launcher_version: String,
  pub extractor_binary_exists: bool,
  pub game_binary_exists: bool,
}

fn dump_per_game_info(
  config_lock: &tokio::sync::MutexGuard<'_, LauncherConfig>,
  package: &mut SupportPackage,
  zip_file: &mut zip::ZipWriter<std::fs::File>,
  install_path: &Path,
  game_name: &str,
) -> Result<(), CommandError> {
  // Save OpenGOAL config folder (this includes saves and settings)
  let game_config_dir = match config_dir() {
    None => {
      return Err(CommandError::Support(
        "Couldn't determine application config directory".to_owned(),
      ))
    }
    Some(path) => path.join("OpenGOAL"),
  };
  append_dir_contents_to_zip(
    zip_file,
    &game_config_dir.join(&game_name).join("settings"),
    format!("Game Settings and Saves/{game_name}/settings").as_str(),
    vec!["gc", "json"],
  )
  .map_err(|_| {
    CommandError::Support("Unable to append game settings to the support package".to_owned())
  })?;
  append_dir_contents_to_zip(
    zip_file,
    &game_config_dir.join(&game_name).join("misc"),
    format!("Game Settings and Saves/{game_name}/misc").as_str(),
    vec!["gc", "json"],
  )
  .map_err(|_| {
    CommandError::Support("Unable to append game misc settings to the support package".to_owned())
  })?;
  append_dir_contents_to_zip(
    zip_file,
    &game_config_dir.join(&game_name).join("saves"),
    format!("Game Settings and Saves/{game_name}/saves").as_str(),
    vec!["bin"],
  )
  .map_err(|_| {
    CommandError::Support("Unable to append game saves to the support package".to_owned())
  })?;

  // Save Logs
  let active_version_dir = install_path.join("active");
  let jak1_log_dir = active_version_dir.join(&game_name).join("data").join("log");
  append_dir_contents_to_zip(
    zip_file,
    &jak1_log_dir,
    format!("Game Logs and ISO Info/{game_name}").as_str(),
    vec!["log", "json", "txt"],
  )
  .map_err(|_| CommandError::Support("Unable to append game logs to support package".to_owned()))?;

  let texture_repl_dir = active_version_dir
    .join(&game_name)
    .join("data")
    .join("texture_replacements");
  package.game_info.get_game_info(game_name).has_texture_packs =
    texture_repl_dir.exists() && texture_repl_dir.read_dir().unwrap().next().is_some();
  let build_info_path = active_version_dir
    .join(&game_name)
    .join("data")
    .join("iso_data")
    .join(&game_name)
    .join("buildinfo.json");
  append_file_to_zip(
    zip_file,
    &build_info_path,
    format!("Game Logs and ISO Info/{game_name}/buildinfo.json").as_str(),
  )
  .map_err(|_| {
    CommandError::Support("Unable to append iso metadata to support package".to_owned())
  })?;

  if config_lock.active_version_folder.is_some() && config_lock.active_version_folder.is_some() {
    let data_dir = active_version_dir.join(&game_name).join("data");
    let version_data_dir = install_path
      .join("versions")
      .join(config_lock.active_version_folder.as_ref().unwrap())
      .join(config_lock.active_version.as_ref().unwrap())
      .join("data");
    package
      .game_info
      .get_game_info(game_name)
      .release_integrity
      .decompiler_folder_modified = dir_diff::is_different(
      data_dir.join("decompiler"),
      version_data_dir.join("decompiler"),
    )
    .unwrap_or(true);
    package
      .game_info
      .get_game_info(game_name)
      .release_integrity
      .game_folder_modified =
      dir_diff::is_different(data_dir.join("game"), version_data_dir.join("game")).unwrap_or(true);
    package
      .game_info
      .get_game_info(game_name)
      .release_integrity
      .goal_src_modified =
      dir_diff::is_different(data_dir.join("goal_src"), version_data_dir.join("goal_src"))
        .unwrap_or(true);
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
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::Support(
        "No installation directory set, can't generate the support package".to_owned(),
      ))
    }
    Some(path) => Path::new(path),
  };

  // System Information
  let mut system_info = System::new_all();
  system_info.refresh_all();
  package.total_memory_megabytes = system_info.total_memory() / 1024 / 1024;
  package.cpu_name = system_info.cpus()[0].name().to_string();
  package.cpu_vendor = system_info.cpus()[0].vendor_id().to_string();
  package.cpu_brand = system_info.cpus()[0].brand().to_string();
  package.os_name = system_info.os_version().unwrap_or("unknown".to_string());
  package.os_name_long = system_info
    .long_os_version()
    .unwrap_or("unknown".to_string());
  package.os_kernel_ver = system_info
    .kernel_version()
    .unwrap_or("unknown".to_string());
  package.launcher_version = app_handle.package_info().version.to_string();
  if let Some(active_version) = &config_lock.active_version {
    if cfg!(windows) {
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
    } else {
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

  for disk in system_info.disks() {
    package.disk_info.push(format!(
      "{} | Name - {} | Capacity - {}GB/{}GB",
      disk.mount_point().to_string_lossy(),
      disk.name().to_string_lossy(),
      disk.available_space() / 1024 / 1024 / 1024,
      disk.total_space() / 1024 / 1024 / 1024
    ))
  }

  let gpu_info_instance = wgpu::Instance::default();
  for a in gpu_info_instance.enumerate_adapters(wgpu::Backends::all()) {
    let info = a.get_info();
    let gpu_info = GPUInfo {
      name: info.name,
      driver_name: info.driver,
      driver_info: info.driver_info,
    };
    package.gpu_info.push(gpu_info);
  }

  // Create zip file
  let save_path = Path::new(&user_path);
  let save_file = std::fs::File::create(save_path)
    .map_err(|_| CommandError::Support("Unable to create support file".to_owned()))?;
  let mut zip_file = zip::ZipWriter::new(save_file);

  // Save Launcher config folder
  let launcher_config_dir = match app_handle.path_resolver().app_config_dir() {
    None => {
      return Err(CommandError::Support(
        "Couldn't determine launcher config directory".to_owned(),
      ))
    }
    Some(path) => path,
  };
  let launcher_log_dir = match app_handle.path_resolver().app_log_dir() {
    None => {
      return Err(CommandError::Support(
        "Couldn't determine launcher log directory".to_owned(),
      ))
    }
    Some(path) => path,
  };
  append_dir_contents_to_zip(
    &mut zip_file,
    &launcher_log_dir,
    "Launcher Settings and Logs/logs",
    vec!["log"],
  )
  .map_err(|_| {
    CommandError::Support("Unable to append launcher logs to the support package".to_owned())
  })?;
  append_file_to_zip(
    &mut zip_file,
    &launcher_config_dir.join("settings.json"),
    "Launcher Settings and Logs/settings.json",
  )
  .map_err(|_| {
    CommandError::Support("Unable to append launcher settings to the support package".to_owned())
  })?;

  // Per Game Info
  dump_per_game_info(
    &config_lock,
    &mut package,
    &mut zip_file,
    install_path,
    "jak1",
  )
  .map_err(|_| {
    CommandError::Support(
      "Unable to dump per game info for jak 1 to the support package".to_owned(),
    )
  })?;
  dump_per_game_info(
    &config_lock,
    &mut package,
    &mut zip_file,
    install_path,
    "jak2",
  )
  .map_err(|_| {
    CommandError::Support(
      "Unable to dump per game info for jak 2 to the support package".to_owned(),
    )
  })?;

  // Dump High Level Info
  let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::DEFLATE)
    .unix_permissions(0o755);
  zip_file
    .start_file("support-info.json", options)
    .map_err(|_| {
      CommandError::Support("Create high level support info entry in support package".to_owned())
    })?;
  let mut json_buffer = Vec::new();
  let json_writer = BufWriter::new(&mut json_buffer);
  serde_json::to_writer_pretty(json_writer, &package).map_err(|_| {
    CommandError::Support(
      "Unable to write high-level support info to the support package".to_owned(),
    )
  })?;
  zip_file.write_all(&json_buffer).map_err(|_| {
    CommandError::Support(
      "Unable to write high-level support info to the support package".to_owned(),
    )
  })?;
  zip_file
    .finish()
    .map_err(|_| CommandError::Support("Unable to finalize zip file".to_owned()))?;

  Ok(())
}
