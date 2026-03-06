use anyhow::Result;
use log::info;
use std::{collections::HashMap, path::Path};
use tauri::{Emitter, Manager};
use walkdir::WalkDir;

use crate::{
  config::{LauncherConfig, SupportedGame},
  util::game_milestones::{GameTaskStatus, MilestoneCriteria, get_jak1_milestones},
};

use super::CommandError;

#[tauri::command]
pub async fn uninstall_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
) -> Result<bool, CommandError> {
  let data_folder = {
    let config_lock = config.lock().await;
    config_lock
      .install_dir()?
      .join("active")
      .join(game_name.to_string())
      .join("data")
  };

  for dir in ["decompiler_out", "iso_data", "out"] {
    let path = data_folder.join(dir);
    std::fs::remove_dir_all(&path).map_err(|e| {
      log::error!("Failed to delete directory {}: {}", path.display(), e);
      CommandError::from(e)
    })?;
  }

  config
    .lock()
    .await
    .update_setting_value("installed", false.into(), Some(game_name))
    .map_err(|_| {
      CommandError::GameManagement("Unable to persist game installation status".to_owned())
    })?;
  app_handle.emit("gameUninstalled", {})?;
  Ok(true)
}

#[tauri::command]
pub async fn reset_game_settings(
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
) -> Result<(), CommandError> {
  let path_to_settings = app_handle
    .path()
    .config_dir()?
    .join("OpenGOAL")
    .join(game_name.to_string())
    .join("settings")
    .join("pc-settings.gc");

  let backup_file = path_to_settings.with_file_name("pc-settings.old.gc");
  if let Err(e) = std::fs::rename(&path_to_settings, &backup_file) {
    if e.kind() != std::io::ErrorKind::NotFound {
      return Err(e.into());
    }
  }
  Ok(())
}

/// Parses a save file and returns the highest milestone reached.
///
/// Save parsing overview:
/// - Save files contain a tagged structure (`game-save`)
/// - Tags are located by scanning 16-byte groups
/// - The name tag appears first and ends with `00 01 00 64`
/// - The task list (type 300) follows and specifies the number of tasks
/// - Each task entry is 16 bytes
///
/// Task interpretation:
/// - Byte 0–4: task-status
/// (invalid 0)
/// (unknown 1)
/// (need-hint 2)
/// (need-introduction 3)
/// (need-reminder-a 4)
/// (need-reminder 5)
/// (need-reward-speech 6)
/// (need-resolution 7)
/// - Byte 11: task id
///
/// A task is considered:
/// - introduced if its status is not 0, 1, 6, or 7
/// - completed if its status is 7
fn get_saves_highest_milestone(
  path: impl AsRef<Path>,
  milestones: &[MilestoneCriteria],
) -> Option<(String, i32)> {
  // Read the file's bytes and generate a list of all completed tasks
  let save_bytes = match std::fs::read(path) {
    Ok(bytes) => bytes,
    Err(err) => {
      log::error!("Failed to read save file: {:?}", err);
      return None;
    }
  };

  let mut tasks: HashMap<u8, GameTaskStatus> = HashMap::new();
  let mut reading_tasks = false;
  let mut tasks_remaining = 0;
  // Iterate through bytes in 16 byte chunks
  for chunk in save_bytes.chunks(16) {
    if reading_tasks {
      // Otherwise, it's a task, parse it
      let new_game_task = GameTaskStatus {
        introduced: chunk[0] != 0 && chunk[0] != 1 && chunk[0] != 6 && chunk[0] != 7,
        completed: chunk[0] == 7,
      };
      tasks.insert(chunk[11], new_game_task);
      tasks_remaining -= 1;
      if tasks_remaining <= 0 {
        break;
      }
    } else {
      // Check to see if we've reached the task list
      if chunk[14] == 0x2C && chunk[15] == 0x01 {
        // Retrieve the amount of tasks that we need to iterate through
        reading_tasks = true;
        tasks_remaining = i32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]);
        info!("Found {} tasks", tasks_remaining);
      }
    }
  }

  // Iterate through the milestones backwards
  for (index, milestone) in milestones.iter().rev().enumerate() {
    for task_id in &milestone.introduced {
      if tasks.contains_key(task_id) && tasks[task_id].introduced {
        return Some((milestone.name.to_owned(), (milestones.len() - index) as i32));
      }
    }
    for task_id in &milestone.completed {
      if tasks.contains_key(task_id) && tasks[task_id].completed {
        return Some((milestone.name.to_owned(), (milestones.len() - index) as i32));
      }
    }
  }

  None
}

/// Returns the most significant milestone the player has reached.
///
/// The frontend uses this value to display a screenshot reflecting the player's progress.
///
/// If no saves are found or no milestones can be determined,
/// the default milestone `"geyser"` is returned.
#[tauri::command]
pub async fn get_furthest_game_milestone(
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
) -> Result<String, CommandError> {
  // TODO: Support Jak 2 and Jak 3?
  let game_save_dir = app_handle
    .path()
    .config_dir()?
    .join("OpenGOAL")
    .join(game_name.to_string())
    .join("saves");

  let milestones = get_jak1_milestones();

  // Recursively scan save files and determine the furthest milestone reached.
  let furthest_milestone_name = WalkDir::new(&game_save_dir)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "bin"))
    .filter_map(|entry| {
      let path = entry.path();
      info!("Scanning save {}", path.display());
      get_saves_highest_milestone(path, &milestones).map(|(name, idx)| {
        info!("Furthest milestone {} at index {}", name, idx);
        (name.to_owned(), idx)
      })
    })
    .max_by_key(|(_, idx)| *idx)
    .map(|(name, _)| name)
    .unwrap_or_else(|| "geyser".to_owned());

  Ok(furthest_milestone_name)
}
