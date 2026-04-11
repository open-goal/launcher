use std::process::Command;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::{config::LauncherConfig, util::file::create_dir};

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
) -> Result<GPUTestOutput> {
  let config_info = config_lock.common_prelude()?;

  let exec_info = config_info.get_exec_location("gk");
  let result_path = app_handle.path().app_data_dir()?;
  create_dir(&result_path)?;
  let result_path = result_path.join("gpu-test-result.json");

  tracing::info!(
    "Running GPU test on game version {} and storing in {}",
    &config_info.active_version,
    result_path.display()
  );

  let mut command = Command::new(exec_info.executable_path);
  command
    .current_dir(exec_info.executable_dir)
    .arg("-v")
    .arg("--gpu-test")
    .arg("opengl")
    .arg("--gpu-test-out-path")
    .arg(&result_path);

  #[cfg(windows)]
  {
    use std::os::windows::process::CommandExt;
    command.creation_flags(0x08000000);
  }

  let output = command.output()?;
  if !output.status.success() {
    anyhow::bail!(
      "GPU test failed: status={} stdout={} stderr={}",
      output.status,
      String::from_utf8_lossy(&output.stdout),
      String::from_utf8_lossy(&output.stderr),
    );
  }

  let context = tokio::fs::read_to_string(&result_path)
    .await
    .with_context(|| {
      format!(
        "Failed to read GPU test result file: {}",
        result_path.display()
      )
    })?;

  return serde_json::from_str::<GPUTestOutput>(&context).with_context(|| {
    format!(
      "Failed to parse GPU test result file: {}",
      result_path.display()
    )
  });
}
