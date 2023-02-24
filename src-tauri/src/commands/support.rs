use serde::{Deserialize, Serialize};
use std::{
  io::{BufWriter, Write},
  path::Path,
};
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use zip::write::FileOptions;

use tauri::Manager;

use crate::{
  config::LauncherConfig,
  util::zip::{append_dir_contents_to_zip, append_file_to_zip},
};

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
}

#[tauri::command]
pub async fn generate_support_package(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  save_path: String,
) -> Result<(), ()> {
  let mut package = SupportPackage::default();
  let config_lock = config.lock().await;
  // TODO - ask the user for the directory
  match &config_lock.installation_dir {
    None => Ok(()),
    Some(path) => {
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

      for disk in system_info.disks() {
        package.disk_info.push(format!(
          "{}:{}-{}GB/{}GB",
          disk.mount_point().to_string_lossy(),
          disk.name().to_string_lossy(),
          disk.available_space() / 1024 / 1024 / 1024,
          disk.total_space() / 1024 / 1024 / 1024
        ))
      }

      // TODO - consider adding a regex for all file appending so we skip weird files that weren't expected

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
      let save_path = Path::new(&path.clone()).join("support-package.zip");
      let save_file = std::fs::File::create(save_path).expect("TODO");
      let mut zip_file = zip::ZipWriter::new(save_file);

      // Save OpenGOAL config folder (this includes saves and settings)
      let game_config_dir = Path::new("C:\\Users\\xtvas\\AppData\\Roaming\\OpenGOAL");
      append_dir_contents_to_zip(&mut zip_file, &game_config_dir, "Game Settings and Saves")
        .expect("good");

      // Save Launcher config folder
      // TODO - prompt on first startup to delete data folder
      let launcher_logs = Path::new("C:\\Users\\xtvas\\AppData\\Roaming\\OpenGOAL-Launcher");
      append_dir_contents_to_zip(
        &mut zip_file,
        &launcher_logs.join("logs"),
        "Launcher Settings and Logs/logs",
      )
      .expect("good");
      append_file_to_zip(
        &mut zip_file,
        &launcher_logs.join("settings.json"),
        "Launcher Settings and Logs/settings.json",
      )
      .expect("TODO");

      // Save Logs
      // TODO - for all games
      let jak1_log_dir = Path::new("C:\\Users\\xtvas\\Downloads\\yee\\active\\jak1\\data\\log");
      append_dir_contents_to_zip(&mut zip_file, &jak1_log_dir, "Game Logs and ISO Info/Jak 1")
        .expect("TODO");

      // Per Game Info
      let texture_repl_dir =
        Path::new("C:\\Users\\xtvas\\Downloads\\yee\\active\\jak1\\data\\texture_replacements");
      package.game_info.jak1.has_texture_packs =
        texture_repl_dir.exists() && !texture_repl_dir.read_dir().unwrap().next().is_none();
      let build_info_path = Path::new(
        "C:\\Users\\xtvas\\Downloads\\yee\\active\\jak1\\data\\iso_data\\jak1\\buildinfo.json",
      );
      append_file_to_zip(
        &mut zip_file,
        &build_info_path,
        "Game Logs and ISO Info/Jak 1/buildinfo.json",
      )
      .expect("TODO");
      let data_dir = Path::new("C:\\Users\\xtvas\\Downloads\\yee\\active\\jak1\\data");
      let version_data_dir =
        Path::new("C:\\Users\\xtvas\\Downloads\\yee\\versions\\official\\v0.1.32\\data");
      package
        .game_info
        .jak1
        .release_integrity
        .decompiler_folder_modified = dir_diff::is_different(
        data_dir.join("decompiler"),
        version_data_dir.join("decompiler"),
      )
      .unwrap();
      package
        .game_info
        .jak1
        .release_integrity
        .game_folder_modified =
        dir_diff::is_different(data_dir.join("game"), version_data_dir.join("game")).unwrap();
      package.game_info.jak1.release_integrity.goal_src_modified =
        dir_diff::is_different(data_dir.join("goal_src"), version_data_dir.join("goal_src"))
          .unwrap();

      // Dump High Level Info
      let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::DEFLATE)
        .unix_permissions(0o755);
      zip_file.start_file("support-info.json", options).unwrap();
      let mut json_buffer = Vec::new();
      let json_writer = BufWriter::new(&mut json_buffer);
      serde_json::to_writer_pretty(json_writer, &package).expect("TODO");
      zip_file.write_all(&json_buffer).expect("TODO");

      zip_file.finish().expect("TODO");

      Ok(())
    }
  }
}
