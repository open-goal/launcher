use std::{path::Path, process::Command};

use crate::config::LauncherConfig;

#[tauri::command]
pub async fn launch_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<(), ()> {
  let config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Ok(()),
    Some(path) => {
      // TODO - be smarter
      // TODO - make folder if it doesnt exist
      // TODO - copy over the data folder
      // TODO - log it to a file
      // TODO - check error code
      let install_path = Path::new(path);
      let binary_dir = install_path.join("versions/official/v0.1.32/");
      let data_folder = install_path.join("active/jak1/data");
      let executable_location = binary_dir.join("gk.exe");
      let output = Command::new(&executable_location)
        .args([
          "-boot".to_string(),
          "-fakeiso".to_string(),
          "-proj-path".to_string(),
          data_folder.to_string_lossy().into_owned(),
        ])
        .current_dir(binary_dir)
        .output()
        .expect("failed to execute process");
      Ok(())
    }
  }
}
