#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, RunEvent};

use std::env;

mod commands;
mod config;
mod textures;
mod util;
use commands::{close_splashscreen, copy_dir, get_highest_simd, open_dir};
use textures::{extract_textures, get_all_texture_packs};

pub type FFIResult<T> = Result<T, String>;

fn main() {
  // TODO - switch to https://github.com/daboross/fern so we can setup easy logging
  // to a file as well
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
      app.manage(tokio::sync::Mutex::new(
        config::LauncherConfig::load_config(app.path_resolver().app_config_dir()),
      ));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::config::get_install_directory,
      commands::config::set_install_directory,
      commands::config::is_avx_requirement_met,
      commands::config::is_opengl_requirement_met,
      commands::config::finalize_installation,
      commands::config::is_game_installed,
      commands::config::get_installed_version,
      commands::config::get_installed_version_folder,
      commands::versions::list_downloaded_versions,
      commands::versions::download_official_version,
      commands::versions::go_to_version_folder,
      commands::versions::save_active_version_change,
      commands::versions::get_active_version,
      commands::versions::get_active_version_folder,
      commands::extractor::extract_and_validate_iso,
      commands::extractor::run_decompiler,
      commands::extractor::run_compiler,
      commands::game::launch_game,
      commands::game::uninstall_game,
      commands::game::reset_game_settings,
      commands::game::open_repl,
      commands::support::generate_support_package,
      // Requirements Checking
      get_highest_simd,
      open_dir,
      copy_dir,
      close_splashscreen,
      extract_textures,
      get_all_texture_packs
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
