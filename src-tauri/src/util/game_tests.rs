#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{path::PathBuf, process::Command};

use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::{
  commands::CommandError,
  config::{CommonConfigData, LauncherConfig},
  util::file::create_dir,
};

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
  let config_info = config_lock.common_prelude()?;

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
          Ok(test_results) => Ok(test_results),
          Err(err) => {
            log::error!("Unable to parse {}: {}", &content, err);
            Err(CommandError::BinaryExecution(
              "Unable to parse GPU test result".to_owned(),
            ))
          }
        }
      } else {
        Err(CommandError::BinaryExecution(
          "GPU Test failed with a non-zero exit code".to_owned(),
        ))
      }
    }
    None => Err(CommandError::BinaryExecution(
      "GPU test failed, no exit-code returned".to_owned(),
    )),
  }
}
