use std::{path::Path, process::Command};

use tauri::{api::path::config_dir, Manager};

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
        .spawn()
        .expect("failed to execute process");
      Ok(())
    }
  }
}

#[tauri::command]
pub async fn uninstall_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
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
      app_handle.emit_all("gameUninstalled", {}).unwrap();
      Ok(())
    }
  }
}

#[tauri::command]
pub async fn reset_game_settings(game_name: String) -> Result<(), ()> {
  let config_dir = config_dir();
  match &config_dir {
    None => Ok(()),
    Some(path) => {
      let path_to_settings = path
        .join("OpenGOAL")
        .join(game_name)
        .join("settings")
        .join("pc-settings.gc");
      if path_to_settings.exists() {
        let mut backup_file = path_to_settings.clone();
        backup_file.set_file_name("pc-settings.old.gc");
        std::fs::rename(path_to_settings, backup_file);
        Ok(())
      } else {
        Ok(())
      }
    }
  }
}

// #[cfg(target_os = "windows")]
// #[tauri::command]
// pub async fn open_repl(proj_path: PathBuf, curr_dir: PathBuf) {
//   tauri::async_runtime::spawn(async move {
//     use std::process::Command as StdCommand;
//     let repl = StdCommand::new("cmd.exe")
//       .args([
//         "/K",
//         "start",
//         "goalc",
//         "--proj-path",
//         proj_path.to_str().as_ref().unwrap(),
//       ])
//       .current_dir(curr_dir)
//       .spawn()
//       .unwrap();
//   });
// }

// #[cfg(target_os = "linux")]
// #[tauri::command]
// pub async fn open_repl(proj_path: PathBuf, curr_dir: PathBuf) {
//   tauri::async_runtime::spawn(async move {
//     use tauri::api::process::Command;
//     let tauri_cmd = Command::new_sidecar("goalc")
//       .unwrap()
//       .current_dir(curr_dir)
//       .args(["--proj-path", proj_path.to_str().as_ref().unwrap()])
//       .spawn();
//   });
// }

#[tauri::command]
pub async fn open_repl(
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
      // TODO - explore a linux option though this is very annoying because without doing a ton of research
      // we seem to have to handle various terminals.  Which honestly we should probably do on windows too
      //
      // So maybe we can make a menu where the user will specify what terminal to use / what launch-options to use
      let install_path = Path::new(path);
      let binary_dir = install_path.join("versions/official/v0.1.32/");
      let data_folder = install_path.join("active/jak1/data");
      let executable_location = binary_dir.join("goalc.exe");
      let output = Command::new("cmd")
        .args([
          "/K",
          "start",
          "goalc.exe",
          "--proj-path",
          &data_folder.to_string_lossy().into_owned(),
        ])
        .current_dir(binary_dir)
        .spawn()
        .expect("failed to execute process");
      Ok(())
    }
  }
}
