use log::{error, info, warn};

#[tauri::command]
pub async fn frontend_log(level: String, log: String) -> Result<(), ()> {
  match level.as_str() {
    "debug" => log::debug!("{}", log),
    "info" => info!("{}", log),
    "warn" => warn!("{}", log),
    "error" => error!("{}", log),
    _ => info!("{}", log),
  }
  Ok(())
}
