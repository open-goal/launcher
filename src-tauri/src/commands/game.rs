use std::{
  collections::HashMap,
  fs::File,
  path::{Path, PathBuf},
  vec,
};

use log::info;
use tauri::{
  api::{path::config_dir, private::AsTauriContext},
  Manager,
};
use walkdir::WalkDir;

use crate::config::LauncherConfig;

use super::CommandError;

#[tauri::command]
pub async fn uninstall_game(
  config: tauri::State<'_, tokio::sync::Mutex<LauncherConfig>>,
  app_handle: tauri::AppHandle,
  game_name: String,
) -> Result<(), CommandError> {
  let mut config_lock = config.lock().await;

  let install_path = match &config_lock.installation_dir {
    None => {
      return Err(CommandError::GameManagement(
        "No installation directory set, can't perform uninstallation".to_owned(),
      ))
    }
    Some(path) => Path::new(path),
  };

  let data_folder = Path::new(install_path)
    .join("active")
    .join(&game_name)
    .join("data");

  std::fs::remove_dir_all(data_folder.join("decompiler_out"))?;
  std::fs::remove_dir_all(data_folder.join("iso_data"))?;
  std::fs::remove_dir_all(data_folder.join("out"))?;

  config_lock
    .update_installed_game_version(&game_name, false)
    .map_err(|_| {
      CommandError::GameManagement("Unable to persist game installation status".to_owned())
    })?;
  app_handle.emit_all("gameUninstalled", {})?;
  Ok(())
}

#[tauri::command]
pub async fn reset_game_settings(game_name: String) -> Result<(), CommandError> {
  let config_dir = match config_dir() {
    None => {
      return Err(CommandError::GameManagement(
        "Could not determine game config directory".to_owned(),
      ))
    }
    Some(path) => path,
  };

  let path_to_settings = config_dir
    .join("OpenGOAL")
    .join(game_name)
    .join("settings")
    .join("pc-settings.gc");
  if path_to_settings.exists() {
    let mut backup_file = path_to_settings.clone();
    backup_file.set_file_name("pc-settings.old.gc");
    std::fs::rename(path_to_settings, backup_file)?;
    Ok(())
  } else {
    Err(CommandError::GameManagement(
      "Game config directory does not exist, cannot reset settings".to_owned(),
    ))
  }
}

struct MilestoneCriteria {
  name: String,
  // some milestones are considered reached once you've completed something
  // (ie. collecting a cell in an area)
  completed: Vec<u8>,
  // others are reached when they've been introduced
  // (ie. preparing for a boss fight)
  introduced: Vec<u8>,
}

struct GameTaskStatus {
  introduced: bool,
  completed: bool,
}

fn get_saves_highest_milestone(
  path: &PathBuf,
  milestones: &Vec<MilestoneCriteria>,
) -> Option<(String, i32)> {
  // Read the file's bytes and generate a list of all completed tasks
  let mut tasks: HashMap<u8, GameTaskStatus> = HashMap::new();
  let save_bytes = std::fs::read(path).unwrap(); // TODO - fix
                                                 // Iterate through bytes in 16 byte chunks
  let mut reading_tasks = false;
  let mut tasks_remaining = 0;
  for chunk in save_bytes.chunks(16) {
    if reading_tasks {
      // Otherwise, it's a task, parse it
      let new_game_task = GameTaskStatus {
        introduced: chunk[0] != 0 && chunk[0] != 1 && chunk[0] != 6 && chunk[0] != 7,
        completed: chunk[0] == 7,
      };
      info!(
        "New task: {}:{}:{}",
        chunk[11], new_game_task.introduced, new_game_task.completed
      );
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
      if tasks.contains_key(&task_id) && tasks[&task_id].introduced {
        return Some((milestone.name.to_owned(), (milestones.len() - index) as i32));
      }
    }
    for task_id in &milestone.completed {
      if tasks.contains_key(&task_id) && tasks[&task_id].completed {
        return Some((milestone.name.to_owned(), (milestones.len() - index) as i32));
      }
    }
  }

  return None;
}

// Returns the most significant milestone in the game the user has achieved
// this is determined by scanning the user's save files
// and displaying a relevant screenshot in the frontend to reflect their progress
//
// Otherwise, it will default to a default picture (geyser)
#[tauri::command]
pub async fn get_furthest_game_milestone(game_name: String) -> Result<String, CommandError> {
  // These milestones are in order and are considered completed
  // TODO - move these to another function
  let jak1_milestones: Vec<MilestoneCriteria> = vec![
    MilestoneCriteria {
      name: "geyser".to_string(),
      completed: vec![],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (village1-yakow 10)
      // (village1-mayor-money 11)
      // (village1-uncle-money 12)
      // (village1-oracle-money1 13)
      // (village1-oracle-money2 14)
      // (beach-ecorocks 15)
      // (village1-buzzer 75)
      name: "sandover".to_string(),
      completed: vec![10, 11, 12, 13, 14, 75],
      introduced: vec![15],
    },
    MilestoneCriteria {
      // (beach-ecorocks 15)
      // (beach-pelican 16)
      // (beach-flutflut 17)
      // (beach-seagull 18)
      // (beach-cannon 19)
      // (beach-buzzer 20)
      // (beach-gimmie 21)
      // (beach-sentinel 22)
      name: "sentinel".to_string(),
      completed: vec![15, 16, 17, 18, 19, 20, 21, 22],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (jungle-eggtop 2)
      // (jungle-lurkerm 3)
      // (jungle-tower 4)
      // (jungle-fishgame 5)
      // (jungle-plant 6)
      // (jungle-buzzer 7)
      // (jungle-canyon-end 8)
      // (jungle-temple-door 9)
      name: "jungle".to_string(),
      completed: vec![2, 3, 4, 5, 6, 7, 8, 9],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (misty-muse 23)
      // (misty-boat 24)
      // (misty-warehouse 25)
      // (misty-cannon 26)
      // (misty-bike 27)
      // (misty-buzzer 28)
      // (misty-bike-jump 29)
      // (misty-eco-challenge 30)
      // (leaving-misty 114)
      name: "misty".to_string(),
      completed: vec![23, 24, 25, 26, 27, 28, 29, 30, 114],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (firecanyon-buzzer 68)
      // (firecanyon-end 69)
      // (firecanyon-assistant 102)
      name: "firecanyon".to_string(),
      completed: vec![68, 69],
      introduced: vec![102],
    },
    MilestoneCriteria {
      // (village2-gambler-money 31)
      // (village2-geologist-money 32)
      // (village2-warrior-money 33)
      // (village2-oracle-money1 34)
      // (village2-oracle-money2 35)
      // (firecanyon-buzzer 68)
      // (firecanyon-end 69)
      // (village2-buzzer 76)
      // (firecanyon-assistant 102)
      name: "village2".to_string(),
      completed: vec![31, 32, 33, 34, 35, 68, 69],
      introduced: vec![76, 102],
    },
    MilestoneCriteria {
      // (rolling-race 52)
      // (rolling-robbers 53)
      // (rolling-moles 54)
      // (rolling-plants 55)
      // (rolling-lake 56)
      // (rolling-buzzer 57)
      // (rolling-ring-chase-1 58)
      // (rolling-ring-chase-2 59)
      name: "basin".to_string(),
      completed: vec![52, 53, 54, 55, 56, 57, 58, 59],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (swamp-billy 36)
      // (swamp-flutflut 37)
      // (swamp-battle 38)
      // (swamp-tether-1 39)
      // (swamp-tether-2 40)
      // (swamp-tether-3 41)
      // (swamp-tether-4 42)
      // (swamp-buzzer 43)
      // (swamp-arm 104)
      name: "swamp".to_string(),
      completed: vec![36, 37, 38, 39, 40, 41, 42, 43, 104],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (sunken-platforms 44)
      // (sunken-pipe 45)
      // (sunken-slide 46)
      // (sunken-room 47)
      // (sunken-sharks 48)
      // (sunken-buzzer 49)
      // (sunken-top-of-helix 50)
      // (sunken-spinning-room 51)
      name: "lpc".to_string(),
      completed: vec![44, 45, 46, 47, 48, 49, 50, 51],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (ogre-boss 86)
      // (village2-levitator 103)
      name: "klaww".to_string(),
      completed: vec![103],
      introduced: vec![86],
    },
    MilestoneCriteria {
      // (ogre-boss 86)
      // (ogre-end 87)
      // (ogre-buzzer 88)
      // (ogre-secret 110)
      name: "mountainpass".to_string(),
      completed: vec![86, 88, 110],
      introduced: vec![87],
    },
    MilestoneCriteria {
      // (village3-extra1 74)
      // (village3-buzzer 77)
      // (village3-miner-money1 96)
      // (village3-miner-money2 97)
      // (village3-miner-money3 98)
      // (village3-miner-money4 99)
      // (village3-oracle-money1 100)
      // (village3-oracle-money2 101)
      // (village3-button 105)
      name: "village3".to_string(),
      completed: vec![74, 77, 96, 97, 98, 99, 100, 101, 105],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (cave-gnawers 78)
      // (cave-dark-crystals 79)
      // (cave-dark-climb 80)
      // (cave-robot-climb 81)
      // (cave-swing-poles 82)
      // (cave-spider-tunnel 83)
      // (cave-platforms 84)
      // (cave-buzzer 85)
      name: "cave".to_string(),
      completed: vec![78, 79, 80, 81, 82, 83, 84, 85],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (snow-eggtop 60)
      // (snow-ram 61)
      // (snow-fort 62)
      // (snow-ball 63)
      // (snow-bunnies 64)
      // (snow-buzzer 65)
      // (snow-bumpers 66)
      // (snow-cage 67)
      name: "snowy".to_string(),
      completed: vec![60, 61, 62, 63, 64, 65, 66, 67],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (lavatube-end 89)
      // (lavatube-buzzer 90)
      // (lavatube-balls 107)
      // (lavatube-start 108)
      // (assistant-village3 115)
      name: "lavatube".to_string(),
      completed: vec![90, 107, 108, 115],
      introduced: vec![89],
    },
    MilestoneCriteria {
      // (citadel-sage-green 70)
      // (citadel-sage-blue 71)
      // (citadel-sage-red 72)
      // (citadel-sage-yellow 73)
      // (lavatube-end 89)
      // (citadel-buzzer 91)
      name: "citadel".to_string(),
      completed: vec![71, 72, 73, 89, 91],
      introduced: vec![70],
    },
    MilestoneCriteria {
      // (citadel-sage-green 70)
      name: "finalboss".to_string(),
      completed: vec![70],
      introduced: vec![],
    },
    MilestoneCriteria {
      // (finalboss-movies 112)
      name: "end".to_string(),
      completed: vec![],
      introduced: vec![112],
    },
  ];

  // TODO - currently only checking Jak 1
  // TODO - move this into a separate module, it would be cool if the launcher had save-game editing features and the like
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
  let game_save_dir = if let Some(config_dir) = config_dir() {
    let expected_dir = config_dir.join("OpenGOAL").join("jak1").join("saves");
    if !expected_dir.exists() {
      return Ok("geyser".to_owned());
    }
    expected_dir
  } else {
    return Ok("geyser".to_owned());
  };

  // Scan the directory recursively for any `*.bin` files
  // Check each save's contents, we don't assume save 0 is the only important one
  let mut highest_milestone_idx = 0;
  let mut furthest_milestone_name = "geyser".to_owned();
  for entry in WalkDir::new(&game_save_dir)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    if let Some(ext) = entry.path().extension() {
      if ext == "bin" {
        info!("Scanning save {}", entry.path().display());
        match get_saves_highest_milestone(&entry.into_path(), &jak1_milestones) {
          Some((name, idx)) => {
            info!("Furthest milestone {} at index {}", name, idx);
            if idx > highest_milestone_idx {
              highest_milestone_idx = idx;
              furthest_milestone_name = name.to_owned();
            }
          }
          None => {}
        }
      }
    }
  }

  return Ok(furthest_milestone_name);
}
