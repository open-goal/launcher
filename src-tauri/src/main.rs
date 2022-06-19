#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_plugin_store::PluginBuilder;

mod commands;
use commands::close_splashscreen;
use commands::get_highest_simd;
use commands::open_dir;

fn main() {
  tauri::Builder::default()
    .plugin(PluginBuilder::default().build())
    .invoke_handler(tauri::generate_handler![
      get_highest_simd,
      open_dir,
      close_splashscreen
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
