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
      commands::binaries::extract_and_validate_iso,
      commands::binaries::launch_game,
      commands::binaries::open_repl,
      commands::binaries::run_compiler,
      commands::binaries::run_decompiler,
      commands::config::finalize_installation,
      commands::config::get_active_version_folder,
      commands::config::get_active_version,
      commands::config::get_install_directory,
      commands::config::get_installed_version_folder,
      commands::config::get_installed_version,
      commands::config::is_avx_requirement_met,
      commands::config::is_avx_supported,
      commands::config::is_game_installed,
      commands::config::is_opengl_requirement_met,
      commands::config::save_active_version_change,
      commands::config::set_install_directory,
      commands::game::reset_game_settings,
      commands::game::uninstall_game,
      commands::support::generate_support_package,
      commands::versions::download_version,
      commands::versions::go_to_version_folder,
      commands::versions::list_downloaded_versions,
      commands::window::close_splashscreen
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
