use anyhow::Result;
use log::info;
use std::{collections::HashMap, path::PathBuf};
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

fn get_saves_highest_milestone(
  path: &PathBuf,
  milestones: &[MilestoneCriteria],
) -> Option<(String, i32)> {
  // Read the file's bytes and generate a list of all completed tasks
  let mut tasks: HashMap<u8, GameTaskStatus> = HashMap::new();
  let save_bytes = match std::fs::read(path) {
    Ok(bytes) => bytes,
    Err(err) => {
      log::error!("Failed to read save file: {:?}", err);
      return None;
    }
  };

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

// Returns the most significant milestone in the game the user has achieved
// this is determined by scanning the user's save files
// and displaying a relevant screenshot in the frontend to reflect their progress
//
// Otherwise, it will default to a default picture (geyser)
#[tauri::command]
pub async fn get_furthest_game_milestone(
  app_handle: tauri::AppHandle,
  game_name: SupportedGame,
) -> Result<String, CommandError> {
  // TODO - currently only checking Jak 1
  // Scan each save file, we inspect the `game-save`'s tag list.
  // - to find the beginning of the tags, scan 16 bytes at a time, find the group that ends with `00 01 00 64` aka 1 element type 100 (name)
  //  - the name tag always comes first
  // - next, we continue to scan until we find type `300` which is the task-list
  //  - this group also says how many tasks there are, each are 16 bytes as well
  // - then it's just a matter of going through each task and seeing if it's completed or not, they are in order of the `game-task` enum
  // - for each entity-perm, byte 0-4 corresponds with it's `task-status`:
  // (invalid 0)
  // (unknown 1)
  // (need-hint 2)
  // (need-introduction 3)
  // (need-reminder-a 4)
  // (need-reminder 5)
  // (need-reward-speech 6)
  // (need-resolution 7)
  // - byte 11 corresponds with it's task id
  // there is also a task status field but we don't really care about it, the task-status entry is sufficient
  let game_save_dir = match app_handle.path().config_dir() {
    Ok(config_dir) => {
      let expected_dir = config_dir
        .join("OpenGOAL")
        .join(game_name.to_string())
        .join("saves");
      if !expected_dir.exists() {
        return Ok("geyser".to_owned());
      }
      expected_dir
    }
    Err(_) => {
      info!("Couldn't determine game save directory");
      return Ok("geyser".to_owned());
    }
  };

  let milestones = get_jak1_milestones();

  // Scan the directory recursively for any `*.bin` files
  // Check each save's contents, we don't assume save 0 is the only important one
  let mut highest_milestone_idx = 0;
  let mut furthest_milestone_name = "geyser".to_owned();
  // TODO - a find all X in a dir function would be nice
  for entry in WalkDir::new(&game_save_dir)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    if let Some(ext) = entry.path().extension() {
      if ext == "bin" {
        info!("Scanning save {}", entry.path().display());
        if let Some((name, idx)) = get_saves_highest_milestone(&entry.into_path(), &milestones) {
          info!("Furthest milestone {} at index {}", name, idx);
          if idx > highest_milestone_idx {
            highest_milestone_idx = idx;
            furthest_milestone_name = name.to_owned();
          }
        }
      }
    }
  }

  Ok(furthest_milestone_name)
}
