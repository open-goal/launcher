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
use std::io::Write;

mod cache;
mod commands;
mod config;
mod util;

fn log_crash(panic_info: Option<&std::panic::PanicInfo>, error: Option<tauri::Error>) {
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
}

fn panic_hook(info: &std::panic::PanicInfo) {
  log_crash(Some(info), None);
}

static TAURI_APP: OnceCell<tauri::AppHandle> = OnceCell::const_new();

fn main() {
  // In the event that some catastrophic happens, atleast log it out
  // the panic_hook will log to a file in the folder of the executable
  std::panic::set_hook(Box::new(panic_hook));

  let tauri_setup = tauri::Builder::default()
    .setup(|app| {
      let _ = TAURI_APP.set(app.app_handle());

      // Setup Logging
      let log_path = app
        .path_resolver()
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
            log::info!("{}", path.path().display());
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
        app.path_resolver().app_config_dir(),
      ));
      app.manage(config);
      let cache = tokio::sync::Mutex::new(cache::LauncherCache::default());
      app.manage(cache);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::binaries::extract_and_validate_iso,
      commands::binaries::get_end_of_logs,
      commands::binaries::get_launch_game_string,
      commands::binaries::launch_game,
      commands::binaries::open_repl,
      commands::binaries::run_compiler,
      commands::binaries::run_decompiler,
      commands::binaries::update_data_directory,
      commands::cache::get_mod_sources_data,
      commands::cache::refresh_mod_sources,
      commands::config::is_rip_levels_enabled,
      commands::config::set_rip_levels_enabled,
      commands::config::is_rip_collision_enabled,
      commands::config::set_rip_collision_enabled,
      commands::config::is_rip_textures_enabled,
      commands::config::set_rip_textures_enabled,
      commands::config::is_rip_streamed_audio_enabled,
      commands::config::set_rip_streamed_audio_enabled,
      commands::config::cleanup_enabled_texture_packs,
      commands::config::delete_old_data_directory,
      commands::config::does_active_tooling_version_support_game,
      commands::config::finalize_installation,
      commands::config::get_active_tooling_version_folder,
      commands::config::get_active_tooling_version,
      commands::config::get_bypass_requirements,
      commands::config::get_enabled_texture_packs,
      commands::config::get_install_directory,
      commands::config::get_installed_version_folder,
      commands::config::get_installed_version,
      commands::config::get_locale,
      commands::config::get_playtime,
      commands::config::does_active_tooling_version_meet_minimum,
      commands::config::has_old_data_directory,
      commands::config::is_avx_requirement_met,
      commands::config::is_diskspace_requirement_met,
      commands::config::is_game_installed,
      commands::config::is_opengl_requirement_met,
      commands::config::is_minimum_vcc_runtime_installed,
      commands::config::reset_to_defaults,
      commands::config::save_active_version_change,
      commands::config::set_bypass_requirements,
      commands::config::set_enabled_texture_packs,
      commands::config::set_install_directory,
      commands::config::set_locale,
      commands::download::download_file,
      commands::features::add_mod_source,
      commands::features::base_game_iso_exists,
      commands::features::compile_for_mod_install,
      commands::features::decompile_for_mod_install,
      commands::features::delete_texture_packs,
      commands::features::extract_iso_for_mod_install,
      commands::features::extract_new_mod,
      commands::features::extract_new_texture_pack,
      commands::features::get_installed_mods,
      commands::features::get_mod_sources,
      commands::features::is_mod_support_enabled,
      commands::features::launch_mod,
      commands::features::list_extracted_texture_pack_info,
      commands::features::remove_mod_source,
      commands::features::save_mod_install_info,
      commands::features::update_texture_pack_data,
      commands::game::get_furthest_game_milestone,
      commands::game::reset_game_settings,
      commands::game::uninstall_game,
      commands::logging::frontend_log,
      commands::support::generate_support_package,
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
