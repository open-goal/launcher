#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use config::config::LauncherConfig;
use tauri::{RunEvent, Manager};

mod commands;
mod config;
mod textures;
use commands::{close_splashscreen, copy_dir, get_highest_simd, open_dir, open_repl};
use textures::{extract_textures, get_all_texture_packs};

pub type FFIResult<T> = Result<T, String>;

#[tauri::command]
async fn test_config_command(config: tauri::State<'_, LauncherConfig>, test: String) -> FFIResult<()> {
    println!("{} | {}", config.version, test);
    Ok(())
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      app.manage(config::config::LauncherConfig::init_config(app.path_resolver().app_config_dir()));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        test_config_command,
      get_highest_simd,
      open_dir,
      copy_dir,
      close_splashscreen,
      extract_textures,
      get_all_texture_packs,
      open_repl
    ])
    .build(tauri::generate_context!())
    .expect("error building tauri app")
    .run(|_app_handle, event| match event {
      RunEvent::ExitRequested { .. } => {
        std::process::exit(0);
      }
      _ => (),
    })
}
