#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::RunEvent;
use tauri_plugin_store::PluginBuilder;

mod commands;
use commands::close_splashscreen;
use commands::get_highest_simd;
use commands::open_dir;
use commands::copy_dir;

fn main() {
  tauri::Builder::default()
    .plugin(PluginBuilder::default().build())
    .invoke_handler(tauri::generate_handler![
      get_highest_simd,
      open_dir,
      copy_dir,
      close_splashscreen
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
