#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::RunEvent;

mod commands;
use commands::close_splashscreen;
use commands::copy_dir;
use commands::extract_textures;
use commands::get_all_texture_packs;
use commands::get_highest_simd;
use commands::open_dir;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
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
