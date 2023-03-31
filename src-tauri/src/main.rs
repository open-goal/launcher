#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use fern::colors::{Color, ColoredLevelConfig};
use tauri::{Manager, RunEvent};
use util::file::create_dir;

use backtrace::Backtrace;
use std::{env, io::Write};

mod commands;
mod config;
mod textures;
mod util;

fn panic_hook(info: &std::panic::PanicInfo) {
  let backtrace = Backtrace::new();
  log::error!("panic occurred: {:?}\n{:?}", info, backtrace);
  match std::fs::File::create("og-launcher-crash.log") {
    Ok(mut file) => {
      if let Err(err) =
        file.write_all(format!("panic occurred: {:?}\n{:?}", info, backtrace).as_bytes())
      {
        log::error!("unable to log crash report to a file - {:?}", err)
      }
    }
    Err(err) => log::error!("unable to log crash report to a file - {:?}", err),
  }
  std::process::exit(1);
}

fn main() {
  // In the event that some catastrophic happens, atleast log it out
  // the panic_hook will log to a file in the folder of the executable
  std::panic::set_hook(Box::new(panic_hook));

  let tauri_setup = tauri::Builder::default()
    .setup(|app| {
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
      // just clone `colors_line` and overwrite our changes
      let colors_level = colors_line.clone().info(Color::Cyan);
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
      app.manage(tokio::sync::Mutex::new(
        config::LauncherConfig::load_config(app.path_resolver().app_config_dir()),
      ));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::logging::frontend_log,
      commands::binaries::update_data_directory,
      commands::binaries::get_end_of_logs,
      commands::binaries::extract_and_validate_iso,
      commands::binaries::launch_game,
      commands::binaries::open_repl,
      commands::binaries::run_compiler,
      commands::binaries::run_decompiler,
      commands::config::finalize_installation,
      commands::config::has_old_data_directory,
      commands::config::delete_old_data_directory,
      commands::config::get_active_tooling_version_folder,
      commands::config::get_active_tooling_version,
      commands::config::get_install_directory,
      commands::config::get_installed_version_folder,
      commands::config::get_installed_version,
      commands::config::is_avx_requirement_met,
      commands::config::is_game_installed,
      commands::config::is_opengl_requirement_met,
      commands::config::set_opengl_requirement_met,
      commands::config::save_active_version_change,
      commands::config::set_install_directory,
      commands::game::reset_game_settings,
      commands::game::uninstall_game,
      commands::support::generate_support_package,
      commands::versions::download_version,
      commands::versions::remove_version,
      commands::versions::go_to_version_folder,
      commands::versions::list_downloaded_versions,
      commands::window::close_splashscreen,
      commands::window::open_dir_in_os
    ])
    .build(tauri::generate_context!())
    .map_err(|err| {
      let backtrace = Backtrace::new();
      log::error!(
        "unexpected top level error occurred: {:?}\n{:?}",
        err,
        backtrace
      );
      match std::fs::File::create("og-launcher-crash.log") {
        Ok(mut file) => {
          if let Err(file_err) = file.write_all(
            format!(
              "unexpected top level error occurred: {:?}\n{:?}",
              err, backtrace
            )
            .as_bytes(),
          ) {
            log::error!("unable to log crash report to a file - {:?}", file_err)
          }
        }
        Err(err) => log::error!("unable to log crash report to a file - {:?}", err),
      }
      std::process::exit(1);
    });
  match tauri_setup {
    Ok(app) => {
      log::info!("application starting up");
      app.run(|_app_handle, event| match event {
        RunEvent::ExitRequested { .. } => {
          log::info!("Exit requested, exiting!");
          std::process::exit(0);
        }
        _ => (),
      })
    }
    Err(err) => {
      log::error!("Could not setup tauri application {:?}, exiting", err);
      std::process::exit(1);
    }
  };
}
