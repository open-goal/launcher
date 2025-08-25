use log::info;
use serde::{Deserialize, Serialize};
use std::{
  fs,
  io::{BufWriter, Write},
  path::Path,
};
use strum::IntoEnumIterator;
use sysinfo::{Disks, System};
use tauri::Manager;
use tempfile::NamedTempFile;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

use crate::{
  config::{LauncherConfig, SupportedGame},
  util::{
    os::get_installed_vcc_runtime,
    zip::{append_dir_contents_to_zip, append_file_to_zip, check_if_zip_contains_top_level_entry},
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
  pub game_info: PerGameInfo,
  pub launcher_version: String,
  pub extractor_binary_exists: bool,
  pub game_binary_exists: bool,
}

fn dump_per_game_info(
  config_lock: &tokio::sync::MutexGuard<'_, LauncherConfig>,
  app_handle: &tauri::AppHandle,
  package: &mut SupportPackage,
  zip_file: &mut zip::ZipWriter<&std::fs::File>,
  install_path: &Path,
  game_name: SupportedGame,
) -> Result<(), CommandError> {
  // Save OpenGOAL config folder (this includes saves and settings)
  let game_config_dir = match app_handle.path().config_dir() {
    Ok(path) => path.join("OpenGOAL"),
    Err(_) => {
      return Err(CommandError::Support(
        "Couldn't determine application config directory".to_owned(),
      ));
    }
  };
  append_dir_contents_to_zip(
    zip_file,
    &game_config_dir.join(game_name.to_string()).join("settings"),
    format!("Game Settings and Saves/{game_name}/settings").as_str(),
    vec!["gc", "json"],
  )
  .map_err(|_| {
    CommandError::Support("Unable to append game settings to the support package".to_owned())
  })?;
  append_dir_contents_to_zip(
    zip_file,
    &game_config_dir.join(game_name.to_string()).join("misc"),
    format!("Game Settings and Saves/{game_name}/misc").as_str(),
    vec!["gc", "json"],
  )
  .map_err(|_| {
    CommandError::Support("Unable to append game misc settings to the support package".to_owned())
  })?;
  append_dir_contents_to_zip(
    zip_file,
    &game_config_dir.join(game_name.to_string()).join("saves"),
    format!("Game Settings and Saves/{game_name}/saves").as_str(),
    vec!["bin"],
  )
  .map_err(|_| {
    CommandError::Support("Unable to append game saves to the support package".to_owned())
  })?;

  // Save Logs
  let active_version_dir = install_path.join("active");
  let game_log_dir = active_version_dir
    .join(game_name.to_string())
    .join("data")
    .join("log");
  append_dir_contents_to_zip(
    zip_file,
    &game_log_dir,
    format!("Game Logs and ISO Info/{game_name}").as_str(),
    vec!["log", "json", "txt"],
  )
  .map_err(|_| CommandError::Support("Unable to append game logs to support package".to_owned()))?;

  let texture_repl_dir = active_version_dir
    .join(game_name.to_string())
    .join("data")
    .join("texture_replacements");
  package.game_info.get_game_info(game_name).has_texture_packs =
    texture_repl_dir.exists() && texture_repl_dir.read_dir().unwrap().next().is_some();
  let build_info_path = active_version_dir
    .join(game_name.to_string())
    .join("data")
    .join("iso_data")
    .join(game_name.to_string())
    .join("buildinfo.json");
  append_file_to_zip(
    zip_file,
    &build_info_path,
    format!("Game Logs and ISO Info/{game_name}/buildinfo.json").as_str(),
  )
  .map_err(|_| {
    CommandError::Support("Unable to append iso metadata to support package".to_owned())
  })?;

  if config_lock.active_version.is_some() {
    let data_dir = active_version_dir.join(game_name.to_string()).join("data");
    let version_data_dir = install_path
      .join("versions")
      .join("official")
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

  // Append mod settings and logs
  let mod_directory = install_path
    .join("features")
    .join(game_name.to_string())
    .join("mods");
  info!("Scanning mod directory {mod_directory:?}");
  if mod_directory.exists() {
    let mod_source_iter = WalkDir::new(mod_directory)
      .into_iter()
      .filter_map(|e| e.ok());
    for source_entry in mod_source_iter {
      let mod_source_path = source_entry.path();
      let mod_source_name = mod_source_path.file_name().unwrap().to_string_lossy(); // TODO
      if mod_source_path.is_dir() {
        let mod_iter = WalkDir::new(mod_source_path)
          .max_depth(0)
          .into_iter()
          .filter_map(|e| e.ok());
        for mod_entry in mod_iter {
          let mod_path = mod_entry.path();
          if mod_path.is_dir() {
            let folder_name = mod_path.file_name().unwrap().to_string_lossy();
            // Check for settings
            if folder_name.eq("_settings") {
              if mod_path.exists() {
                append_dir_contents_to_zip(
                  zip_file,
                  mod_path,
                  format!("Game Settings and Saves/{game_name}/mods/{mod_source_name}/settings")
                    .as_str(),
                  vec!["gc", "json"],
                )
                .map_err(|_| {
                  CommandError::Support(
                    "Unable to append mod settings to support package".to_owned(),
                  )
                })?;
                append_dir_contents_to_zip(
                  zip_file,
                  mod_path,
                  format!("Game Settings and Saves/{game_name}/mods/{mod_source_name}/saves")
                    .as_str(),
                  vec!["bin"],
                )
                .map_err(|_| {
                  CommandError::Support("Unable to append mod saves to support package".to_owned())
                })?;
              }
            } else {
              // Get logs for each individual mod
              let mod_log_folder = mod_path.join("data").join("log");
              if mod_log_folder.exists() {
                append_dir_contents_to_zip(
                  zip_file,
                  &mod_log_folder,
                  format!(
                    "Game Logs and ISO Info/{game_name}/mods/{mod_source_name}/{folder_name}/logs"
                  )
                  .as_str(),
                  vec!["log", "json", "txt"],
                )
                .map_err(|_| {
                  CommandError::Support("Unable to append mod logs to support package".to_owned())
                })?;
              }
            }
          }
        }
      }
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
  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::Support(
        "No installation directory set, can't generate the support package".to_owned(),
      ));
    }
    Some(path) => Path::new(path),
  };

  // System Information
  let mut system_info = System::new_all();
  system_info.refresh_all();
  package.installed_vcc_runtime = get_installed_vcc_runtime().map(|v| v.to_string());
  package.total_memory_megabytes = system_info.total_memory() / 1024 / 1024;
  package.cpu_name = system_info.cpus()[0].name().to_string();
  package.cpu_vendor = system_info.cpus()[0].vendor_id().to_string();
  package.cpu_brand = system_info.cpus()[0].brand().to_string();
  package.os_name = System::os_version().unwrap_or("unknown".to_string());
  package.os_name_long = System::long_os_version().unwrap_or("unknown".to_string());
  package.os_kernel_ver = System::kernel_version().unwrap_or("unknown".to_string());
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

  for disk in Disks::new_with_refreshed_list().into_iter() {
    package.disk_info.push(format!(
      "{} | Name - {} | Capacity - {}GB/{}GB",
      disk.mount_point().to_string_lossy(),
      disk.name().to_string_lossy(),
      disk.available_space() / 1024 / 1024 / 1024,
      disk.total_space() / 1024 / 1024 / 1024
    ))
  }

  let test_result = crate::util::game_tests::run_game_gpu_test(&config_lock, &app_handle).await;
  match test_result {
    Ok(result) => {
      let gpu_info = GPUInfo {
        opengl_test_passed: result.success,
        renderer_name: format!(
          "{}:{}",
          result
            .gpu_renderer_string
            .unwrap_or("unknown renderer".to_string()),
          result
            .gpu_vendor_string
            .unwrap_or("unknown vendor".to_string())
        ),
      };
      package.gpu_info = Some(gpu_info);
    }
    Err(_) => {
      package.gpu_info = None;
    }
  };

  // Create zip file
  let save_path = Path::new(&user_path);
  let save_file = NamedTempFile::new().map_err(|_| {
    CommandError::Support("Failed to create temp file and unable to create support file".to_owned())
  })?;
  let mut zip_file = zip::ZipWriter::new(save_file.as_file());

  // Save Launcher config folder
  let launcher_config_dir = match app_handle.path().app_config_dir() {
    Ok(path) => path,
    Err(_) => {
      return Err(CommandError::Support(
        "Couldn't determine launcher config directory".to_owned(),
      ));
    }
  };
  let launcher_log_dir = match app_handle.path().app_log_dir() {
    Ok(path) => path,
    Err(_) => {
      return Err(CommandError::Support(
        "Couldn't determine launcher log directory".to_owned(),
      ));
    }
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
  for game in SupportedGame::iter() {
    dump_per_game_info(
      &config_lock,
      &app_handle,
      &mut package,
      &mut zip_file,
      install_path,
      game,
    )
    .map_err(|_| {
      CommandError::Support(format!(
        "Unable to dump per game info for {game} to the support package",
      ))
    })?;
  }

  // Dump High Level Info
  let options = SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated)
    .compression_level(Some(9))
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

  // Sanity check that the zip file was actually made correctly
  let info_found =
    check_if_zip_contains_top_level_entry(&save_file, "support-info.json").map_err(|_| {
      CommandError::Support("Support package was unable to be written properly".to_owned())
    })?;
  if !info_found {
    return Err(CommandError::Support(
      "Support package was unable to be written properly".to_owned(),
    ));
  }
  // Seems good, move it to the user's intended destination
  fs::copy(save_file.path(), save_path).map_err(|e| {
    CommandError::Support(format!(
      "Support package was unable to be moved from its temporary file location: {:?}",
      e
    ))
  })?;

  fs::remove_file(save_file.path()).map_err(|e| {
    CommandError::Support(format!(
      "Support package was copied but the original file could not be removed: {:?}",
      e
    ))
  })?;
  Ok(())
}
