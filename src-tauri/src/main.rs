#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, RunEvent};

use std::env;

mod commands;
mod config;
mod textures;
use commands::{
  close_splashscreen, copy_dir, get_highest_simd, get_install_directory, open_dir, open_repl,
  set_install_directory,
};
use textures::{extract_textures, get_all_texture_packs};
pub type FFIResult<T> = Result<T, String>;

fn main() {
  if env::var_os("RUST_LOG").is_none() {
    env::set_var("RUST_LOG", "debug");
  }

  pretty_env_logger::init();

  tauri::Builder::default()
    .setup(|app| {
      // Load the config (or initialize it with defaults)
      //
      // Tauri is pretty cool - you can "manage" as many instances of structs as you want (so long as it's only 1 per type)
      // Then commands can retrieve said managed structs via `tauri::State`
      //
      // This allows us to avoid hacky globals, and pass around information (in this case, the config)
      // to the relevant places
      app.manage(std::sync::Mutex::new(
        config::config::LauncherConfig::load_config(app.path_resolver().app_config_dir()),
      ));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      // Config Related
      get_install_directory,
      set_install_directory,
      // Requirements Checking
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
