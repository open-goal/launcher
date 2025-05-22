#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use directories::UserDirs;
use fern::colors::{Color, ColoredLevelConfig};
use tauri::{Manager, RunEvent};
use tokio::sync::OnceCell;
use util::file::create_dir;

use backtrace::Backtrace;
use native_dialog::{DialogBuilder, MessageLevel};
use std::io::Write;

mod cache;
mod commands;
mod config;
mod util;

fn log_crash(panic_info: Option<&std::panic::PanicHookInfo>, error: Option<tauri::Error>) {
  let backtrace = Backtrace::new();
  let log_contents;
  if let Some(panic_info) = panic_info {
    log_contents = format!("panic occurred: {panic_info:?}\n{backtrace:?}");
  } else if let Some(error) = error {
    log_contents = format!("unexpected app error occurred: {error:?}\n{backtrace:?}",);
  } else {
    log_contents = format!("unexpected error occurred: {backtrace:?}");
  }
  log::error!("{}", log_contents);
  if let Some(user_dirs) = UserDirs::new() {
    if let Some(desktop_dir) = user_dirs.desktop_dir() {
      match std::fs::File::create(desktop_dir.join("og-launcher-crash.log")) {
        Ok(mut file) => {
          if let Err(err) = file.write_all(log_contents.as_bytes()) {
            log::error!("unable to log crash report to a file - {:?}", err)
          }
        }
        Err(err) => log::error!("unable to log crash report to a file - {:?}", err),
      }
    }
  }

  let mut dialog_text = "Unrecoverable crash occurred!".to_string();
  if cfg!(windows) {
    dialog_text = format!("{dialog_text} Ensure you have not uninstalled WebView2: https://developer.microsoft.com/en-us/microsoft-edge/webview2/?form=MA13LH#download");
  }
  dialog_text = format!("{dialog_text}\n\nDetails:\n{log_contents}");

  DialogBuilder::message()
    .set_level(MessageLevel::Error)
    .set_title("OpenGOAL Launcher Crash Info")
    .set_text(format!("{:?}", &dialog_text))
    .alert()
    .show()
    .unwrap();
}

fn panic_hook(info: &std::panic::PanicHookInfo) {
  log_crash(Some(info), None);
}

static TAURI_APP: OnceCell<tauri::AppHandle> = OnceCell::const_new();

fn main() {
  std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
  // In the event that some catastrophic happens, atleast log it out
  // the panic_hook will log to a file in the folder of the executable
  std::panic::set_hook(Box::new(panic_hook));

  let tauri_setup = tauri::Builder::default()
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .plugin(tauri_plugin_os::init())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_fs::init())
    .setup(|app| {
      let _ = TAURI_APP.set(app.app_handle().clone());

      // Setup Logging
      let log_path = app
        .path()
        .app_log_dir()
        .expect("Could not determine log path")
        .join("app");
      create_dir(&log_path)?;

      // configure colors for the whole line
      let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Cyan)
        .debug(Color::Green)
        .trace(Color::White);

      // configure colors for the name of the level.
      // since almost all of them are the same as the color for the whole line, we
      // just copy `colors_line` and overwrite our changes
      let colors_level = colors_line.info(Color::Cyan);

      let log_setup_ok = fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(move |out, message, record| {
          out.finish(format_args!(
            "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
            color_line = format_args!(
              "\x1B[{}m",
              colors_line.get_color(&record.level()).to_fg_str()
            ),
            date = chrono::Local::now().format("%H:%M:%S"),
            target = record.target(),
            level = colors_level.color(record.level()),
            message = message,
          ));
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        .filter(|metadata| metadata.target() != "tao::platform_impl::platform::event_loop::runner") // suppress tauri log spam (windows only)
        // - and per-module overrides
        // .level_for("opengoal-launcher", log::LevelFilter::Debug)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::DateBased::new(&log_path, "/%Y-%m-%d.log"))
        // Apply globally
        .apply();
      match log_setup_ok {
        Ok(_) => {
          log::info!("Logging Initialized");
          // Truncate rotated log files to '5'
          let mut paths: Vec<_> = std::fs::read_dir(&log_path)?.map(|r| r.unwrap()).collect();
          paths.sort_by_key(|dir| dir.path());
          paths.reverse();
          let mut i = 0;
          for path in paths {
            i += 1;
            if i > 5 {
              log::info!("deleting - {}", path.path().display());
              std::fs::remove_file(path.path())?;
            }
          }
        }
        Err(err) => log::error!("Could not initialize logging {:?}", err),
      };

      // Load the config (or initialize it with defaults)
      //
      // Tauri is pretty cool - you can "manage" as many instances of structs as you want (so long as it's only 1 per type)
      // Then commands can retrieve said managed structs via `tauri::State`
      //
      // This allows us to avoid hacky globals, and pass around information (in this case, the config)
      // to the relevant places
      let config = tokio::sync::Mutex::new(config::LauncherConfig::load_config(
        app.path().app_config_dir().ok(),
      ));
      app.manage(config);
      let cache = tokio::sync::Mutex::new(cache::LauncherCache::default());
      app.manage(cache);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::binaries::extract_and_validate_iso,
      commands::binaries::get_launch_game_string,
      commands::binaries::launch_game,
      commands::binaries::open_repl,
      commands::binaries::run_compiler,
      commands::binaries::run_decompiler,
      commands::binaries::update_data_directory,
      commands::cache::get_mod_sources_data,
      commands::cache::refresh_mod_sources,
      commands::config::cleanup_enabled_texture_packs,
      commands::config::does_active_tooling_version_meet_minimum,
      commands::config::does_active_tooling_version_support_game,
      commands::config::get_setting_value,
      commands::config::is_avx_requirement_met,
      commands::config::is_opengl_requirement_met,
      commands::config::reset_to_defaults,
      commands::config::set_install_directory,
      commands::config::update_mods_setting_value,
      commands::config::update_setting_value,
      commands::download::download_file,
      commands::features::mods::base_game_iso_exists,
      commands::features::mods::compile_for_mod_install,
      commands::features::mods::decompile_for_mod_install,
      commands::features::mods::download_and_extract_new_mod,
      commands::features::mods::extract_iso_for_mod_install,
      commands::features::mods::extract_new_mod,
      commands::features::mods::get_launch_mod_string,
      commands::features::mods::get_local_mod_cover_base64,
      commands::features::mods::get_local_mod_thumbnail_base64,
      commands::features::mods::launch_mod,
      commands::features::mods::open_repl_for_mod,
      commands::features::mods::reset_mod_settings,
      commands::features::mods::save_mod_install_info,
      commands::features::mods::uninstall_mod,
      commands::features::texture_packs::delete_texture_packs,
      commands::features::texture_packs::extract_new_texture_pack,
      commands::features::texture_packs::list_extracted_texture_pack_info,
      commands::features::texture_packs::update_texture_pack_data,
      commands::game::get_furthest_game_milestone,
      commands::game::reset_game_settings,
      commands::game::uninstall_game,
      commands::logging::frontend_log,
      commands::support::generate_support_package,
      commands::util::delete_old_data_directory,
      commands::util::has_old_data_directory,
      commands::util::is_diskspace_requirement_met,
      commands::util::is_macos_version_15_or_above,
      commands::util::is_minimum_vcc_runtime_installed,
      commands::util::path_exists,
      commands::versions::download_version,
      commands::versions::ensure_active_version_still_exists,
      commands::versions::go_to_version_folder,
      commands::versions::list_downloaded_versions,
      commands::versions::remove_version,
      commands::window::open_dir_in_os,
      commands::window::open_main_window,
    ])
    .build(tauri::generate_context!())
    .map_err(|err| {
      log_crash(None, Some(err));
    });
  match tauri_setup {
    Ok(app) => {
      log::info!("application starting up");
      app.run(|_app_handle, event| {
        if let RunEvent::ExitRequested { .. } = event {
          log::info!("Exit requested, exiting!");
          std::process::exit(0);
        }
      })
    }
    Err(err) => {
      log::error!("Could not setup tauri application {:?}, exiting", err);
      std::process::exit(1);
    }
  };
}
