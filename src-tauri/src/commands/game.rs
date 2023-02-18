use std::{path::Path, process::Command};

use crate::config::LauncherConfig;

#[tauri::command]
pub async fn launch_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
  in_debug: bool,
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
      let mut args = vec!["-boot".to_string(), "-fakeiso".to_string()];
      // TODO - order unfortunately matters for gk args, this will be fixed eventually...
      if in_debug {
        args.push("-debug".to_string());
      }
      args.push("-proj-path".to_string());
      args.push(data_folder.to_string_lossy().into_owned());
      let output = Command::new(&executable_location)
        .args(args)
        .current_dir(binary_dir)
        .output()
        .expect("failed to execute process");
      Ok(())
    }
  }
}

#[tauri::command]
pub async fn uninstall_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  game_name: String,
) -> Result<(), ()> {
  let mut config_lock = config.lock().await;
  match &config_lock.installation_dir {
    None => Ok(()),
    Some(path) => {
      // TODO - cleanup
      let data_folder = Path::new(path).join("active/jak1/data");
      std::fs::remove_dir_all(data_folder.join("decompiler_out"));
      std::fs::remove_dir_all(data_folder.join("iso_data"));
      std::fs::remove_dir_all(data_folder.join("out"));
      config_lock.update_installed_game_version(game_name, false);
      Ok(())
    }
  }
}
