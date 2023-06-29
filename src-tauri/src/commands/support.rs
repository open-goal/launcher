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
      return Err(CommandError::Support(format!(
        "No installation directory set, can't generate the support package"
      )))
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

  for disk in system_info.disks() {
    package.disk_info.push(format!(
      "{}:{}-{}GB/{}GB",
      disk.mount_point().to_string_lossy(),
      disk.name().to_string_lossy(),
      disk.available_space() / 1024 / 1024 / 1024,
      disk.total_space() / 1024 / 1024 / 1024
    ))
  }

  // TODO - maybe long-term this can replace glewinfo / support vulkan?
  let gpu_info_instance = wgpu::Instance::default();
  for a in gpu_info_instance.enumerate_adapters(wgpu::Backends::all()) {
    let info = a.get_info();
    let mut gpu_info = GPUInfo::default();
    gpu_info.name = info.name;
    gpu_info.driver_name = info.driver;
    gpu_info.driver_info = info.driver_info;
    package.gpu_info.push(gpu_info);
  }

  // Create zip file
  let save_path = Path::new(&user_path);
  let save_file = std::fs::File::create(save_path)
    .map_err(|_| CommandError::Support(format!("Unable to create support file")))?;
  let mut zip_file = zip::ZipWriter::new(save_file);

  // Save OpenGOAL config folder (this includes saves and settings)
  let game_config_dir = match config_dir() {
    None => {
      return Err(CommandError::Support(format!(
        "Couldn't determine application config directory"
      )))
    }
    Some(path) => path.join("OpenGOAL"),
  };
  append_dir_contents_to_zip(
    &mut zip_file,
    &game_config_dir.join("jak1").join("settings"),
    "Game Settings and Saves/jak1/settings",
    vec!["gc", "json"],
  )
  .map_err(|_| {
    CommandError::Support(format!(
      "Unable to append game settings to the support package"
    ))
  })?;
  append_dir_contents_to_zip(
    &mut zip_file,
    &game_config_dir.join("jak1").join("misc"),
    "Game Settings and Saves/jak1/misc",
    vec!["gc", "json"],
  )
  .map_err(|_| {
    CommandError::Support(format!(
      "Unable to append game misc settings to the support package"
    ))
  })?;
  append_dir_contents_to_zip(
    &mut zip_file,
    &game_config_dir.join("jak1").join("saves"),
    "Game Settings and Saves/jak1/saves",
    vec!["bin"],
  )
  .map_err(|_| {
    CommandError::Support(format!(
      "Unable to append game saves to the support package"
    ))
  })?;

  // Save Launcher config folder
  let launcher_config_dir = match app_handle.path_resolver().app_config_dir() {
    None => {
      return Err(CommandError::Support(format!(
        "Couldn't determine launcher config directory"
      )))
    }
    Some(path) => path,
  };
  append_dir_contents_to_zip(
    &mut zip_file,
    &launcher_config_dir.join("logs"),
    "Launcher Settings and Logs/logs",
    vec!["log"],
  )
  .map_err(|_| {
    CommandError::Support(format!(
      "Unable to append launcher logs to the support package"
    ))
  })?;
  append_file_to_zip(
    &mut zip_file,
    &launcher_config_dir.join("settings.json"),
    "Launcher Settings and Logs/settings.json",
  )
  .map_err(|_| {
    CommandError::Support(format!(
      "Unable to append launcher settings to the support package"
    ))
  })?;

  // Save Logs
  let active_version_dir = install_path.join("active");
  // TODO - for all games
  let jak1_log_dir = active_version_dir.join("jak1").join("data").join("log");
  append_dir_contents_to_zip(
    &mut zip_file,
    &jak1_log_dir,
    "Game Logs and ISO Info/Jak 1",
    vec!["log", "json", "txt"],
  )
  .map_err(|_| CommandError::Support(format!("Unable to append game logs to support package")))?;

  // Per Game Info
  let texture_repl_dir = active_version_dir
    .join("jak1")
    .join("data")
    .join("texture_replacements");
  package.game_info.jak1.has_texture_packs =
    texture_repl_dir.exists() && !texture_repl_dir.read_dir().unwrap().next().is_none();
  let build_info_path = active_version_dir
    .join("jak1")
    .join("data")
    .join("iso_data")
    .join("jak1")
    .join("buildinfo.json");
  append_file_to_zip(
    &mut zip_file,
    &build_info_path,
    "Game Logs and ISO Info/Jak 1/buildinfo.json",
  )
  .map_err(|_| {
    CommandError::Support(format!("Unable to append iso metadata to support package"))
  })?;

  if config_lock.active_version_folder.is_some() && config_lock.active_version_folder.is_some() {
    let data_dir = active_version_dir.join("jak1").join("data");
    let version_data_dir = install_path
      .join("versions")
      .join(config_lock.active_version_folder.as_ref().unwrap())
      .join(config_lock.active_version.as_ref().unwrap())
      .join("data");
    package
      .game_info
      .jak1
      .release_integrity
      .decompiler_folder_modified = dir_diff::is_different(
      data_dir.join("decompiler"),
      version_data_dir.join("decompiler"),
    )
    .unwrap_or(true);
    package
      .game_info
      .jak1
      .release_integrity
      .game_folder_modified =
      dir_diff::is_different(data_dir.join("game"), version_data_dir.join("game")).unwrap_or(true);
    package.game_info.jak1.release_integrity.goal_src_modified =
      dir_diff::is_different(data_dir.join("goal_src"), version_data_dir.join("goal_src"))
        .unwrap_or(true);
  }

  // Dump High Level Info
  let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::DEFLATE)
    .unix_permissions(0o755);
  zip_file
    .start_file("support-info.json", options)
    .map_err(|_| {
      CommandError::Support(format!(
        "Create high level support info entry in support package"
      ))
    })?;
  let mut json_buffer = Vec::new();
  let json_writer = BufWriter::new(&mut json_buffer);
  serde_json::to_writer_pretty(json_writer, &package).map_err(|_| {
    CommandError::Support(format!(
      "Unable to write high-level support info to the support package"
    ))
  })?;
  zip_file.write_all(&json_buffer).map_err(|_| {
    CommandError::Support(format!(
      "Unable to write high-level support info to the support package"
    ))
  })?;
  zip_file
    .finish()
    .map_err(|_| CommandError::Support(format!("Unable to finalize zip file")))?;

  Ok(())
}
