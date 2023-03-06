#[tauri::command]
pub async fn frontend_log(level: String, log: String) -> Result<(), ()> {
  match level.as_str() {
    "debug" => log::debug!("{}", log),
    "info" => log::info!("{}", log),
    "warn" => log::warn!("{}", log),
    "error" => log::error!("{}", log),
    _ => log::info!("{}", log),
  }
  Ok(())
}
