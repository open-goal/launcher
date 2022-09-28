#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::RunEvent;

mod commands;
mod textures;
use commands::{close_splashscreen, copy_dir, get_highest_simd, open_dir};
use textures::{extract_textures, get_all_texture_packs};

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
