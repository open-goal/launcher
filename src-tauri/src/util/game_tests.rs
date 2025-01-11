#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
  path::{Path, PathBuf},
  process::Command,
};

use semver::Version;
use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::{commands::CommandError, config::LauncherConfig, util::file::create_dir};

// TODO - duplicate code, cleanup later

struct CommonConfigData {
  install_path: std::path::PathBuf,
  active_version: String,
  tooling_version: Version,
}

fn common_prelude(
  config: &tokio::sync::MutexGuard<LauncherConfig>,
) -> Result<CommonConfigData, CommandError> {
  let install_path = match &config.installation_dir {
    None => {
      return Err(CommandError::BinaryExecution(
        "No installation directory set, can't perform operation".to_owned(),
      ))
    }
    Some(path) => Path::new(path),
  };

  let active_version = config
    .active_version
    .as_ref()
    .ok_or(CommandError::BinaryExecution(
      "No active version set, can't perform operation".to_owned(),
    ))?;

  let tooling_version = Version::parse(active_version.strip_prefix('v').unwrap_or(&active_version))
    .unwrap_or(Version::new(0, 1, 35)); // assume new format if none can be found

  Ok(CommonConfigData {
    install_path: install_path.to_path_buf(),
    active_version: active_version.clone(),
    tooling_version: tooling_version,
  })
}

fn bin_ext(filename: &str) -> String {
  if cfg!(windows) {
    return format!("{filename}.exe");
  }
  filename.to_string()
}

struct ExecutableLocation {
  executable_dir: PathBuf,
  executable_path: PathBuf,
}

fn get_exec_location(
  config_info: &CommonConfigData,
  executable_name: &str,
) -> Result<ExecutableLocation, CommandError> {
  let exec_dir = config_info
    .install_path
    .join("versions")
    .join("official")
    .join(&config_info.active_version);
  let exec_path = exec_dir.join(bin_ext(executable_name));
  if !exec_path.exists() {
    log::error!(
      "Could not find the required binary '{}', can't perform operation",
      exec_path.to_string_lossy()
    );
    return Err(CommandError::BinaryExecution(format!(
      "Could not find the required binary '{}', can't perform operation",
      exec_path.to_string_lossy()
    )));
  }
  Ok(ExecutableLocation {
    executable_dir: exec_dir,
    executable_path: exec_path,
  })
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GPUTestOutput {
  pub error: String,
  pub error_cause: String,
  pub success: bool,
  pub gpu_renderer_string: Option<String>,
  pub gpu_vendor_string: Option<String>,
}

pub async fn run_game_gpu_test(
  config_lock: &tokio::sync::MutexGuard<'_, LauncherConfig>,
  app_handle: &tauri::AppHandle,
) -> Result<GPUTestOutput, CommandError> {
  let config_info = common_prelude(config_lock)?;

  let exec_info = get_exec_location(&config_info, "gk")?;
  let gpu_test_result_path = &match app_handle.path().app_data_dir() {
    Ok(path) => path,
    Err(err) => {
      log::error!(
        "Error encountered when determined path for binary for GPU test: {:?}",
        err
      );
      return Err(CommandError::BinaryExecution(
        "Could not determine path to save GPU test results".to_owned(),
      ));
    }
  };
  create_dir(gpu_test_result_path)?;
  let gpu_test_result_path = &gpu_test_result_path.join("gpu-test-result.json");

  log::info!(
    "Running GPU test on game version {:?} and storing in folder: {:?}",
    &config_info.active_version,
    gpu_test_result_path
  );

  let mut command = Command::new(exec_info.executable_path);
  command
    .args([
      "-v".to_string(),
      "--gpu-test".to_string(),
      "opengl".to_string(),
      "--gpu-test-out-path".to_string(),
      gpu_test_result_path.to_string_lossy().into_owned(),
    ])
    .current_dir(exec_info.executable_dir);
  #[cfg(windows)]
  {
    command.creation_flags(0x08000000);
  }
  let output = command.output()?;
  match output.status.code() {
    Some(code) => {
      if code == 0 {
        // Parse the JSON file
        // Read the file
        let content = match std::fs::read_to_string(gpu_test_result_path) {
          Ok(content) => content,
          Err(err) => {
            log::error!("Unable to read {}: {}", gpu_test_result_path.display(), err);
            return Err(CommandError::BinaryExecution(
              "Unable to read gpu test result".to_owned(),
            ));
          }
        };

        // Serialize from json
        match serde_json::from_str::<GPUTestOutput>(&content) {
          Ok(test_results) => {
            return Ok(test_results);
          }
          Err(err) => {
            log::error!("Unable to parse {}: {}", &content, err);
            return Err(CommandError::BinaryExecution(
              "Unable to parse GPU test result".to_owned(),
            ));
          }
        }
      } else {
        return Err(CommandError::BinaryExecution(
          "GPU Test failed with a non-zero exit code".to_owned(),
        ));
      }
    }
    None => {
      return Err(CommandError::BinaryExecution(
        "GPU test failed, no exit-code returned".to_owned(),
      ))
    }
  }
}
