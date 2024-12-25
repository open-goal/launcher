use std::{process::ExitStatus, sync::Arc, time::Duration};

use tokio::{
  io::{AsyncBufReadExt, AsyncWriteExt},
  sync::Mutex,
};

use crate::commands::CommandError;

use super::file::create_dir;
use tauri::{Emitter, Manager};

pub async fn create_log_file(
  app_handle: &tauri::AppHandle,
  name: String,
  append: bool,
) -> Result<tokio::fs::File, CommandError> {
  let log_path = &match app_handle.path().app_log_dir() {
    Ok(path) => path,
    Err(_) => {
      return Err(CommandError::Installation(
        "Could not determine path to save installation logs".to_owned(),
      ))
    }
  };
  create_dir(log_path)?;
  let mut file_options = tokio::fs::OpenOptions::new();
  file_options.read(true);
  file_options.create(true);
  if append {
    file_options.append(true);
  } else {
    file_options.write(true).truncate(true);
  }
  let file = file_options.open(log_path.join(name)).await?;
  Ok(file)
}

#[derive(Clone, serde::Serialize)]
struct LogPayload {
  logs: String,
}

pub async fn watch_process(
  log_file: &mut tokio::fs::File,
  child: &mut tokio::process::Child,
  app_handle: &tauri::AppHandle,
) -> Result<Option<ExitStatus>, CommandError> {
  let stdout = child.stdout.take().unwrap();
  let stderr = child.stderr.take().unwrap();

  let mut stdout_reader = tokio::io::BufReader::new(stdout).lines();
  let mut stderr_reader = tokio::io::BufReader::new(stderr).lines();
  let combined_buffer = Arc::new(Mutex::new(String::new()));

  let mut interval = tokio::time::interval(Duration::from_millis(25));

  let mut process_status = None;
  loop {
    let buffer_clone = Arc::clone(&combined_buffer);
    tokio::select! {
        Ok(Some(line)) = stdout_reader.next_line() => {
          let formatted_line = format!("{line}\n");
          log_file.write_all(formatted_line.as_bytes()).await?;
          if formatted_line != "\n" {
            let mut buf = buffer_clone.lock().await;
            buf.push_str(&formatted_line);
          }
        },
        Ok(Some(line)) = stderr_reader.next_line() => {
          let formatted_line = format!("{line}\n");
          log_file.write_all(formatted_line.as_bytes()).await?;
          if formatted_line != "\n" {
            let mut buf = buffer_clone.lock().await;
            buf.push_str(&formatted_line);
          }
        },
        _ = interval.tick() => {
          log_file.flush().await?;
          {
            let mut buf = buffer_clone.lock().await;
            let _ = app_handle.emit("log_update", LogPayload { logs: buf.clone() });
            buf.clear();
          }
        },
        // Wait for the child process to finish
        status = child.wait() => {
          let mut buf = buffer_clone.lock().await;
          let _ = app_handle.emit("log_update", LogPayload { logs: buf.clone() });
          buf.clear();
          process_status = Some(status?);
          break;
        }
    }
  }
  Ok(process_status)
}

pub fn create_std_log_file(
  app_handle: &tauri::AppHandle,
  name: String,
  append: bool,
) -> Result<std::fs::File, CommandError> {
  let log_path = &match app_handle.path().app_log_dir() {
    Ok(path) => path,
    Err(_) => {
      return Err(CommandError::Installation(
        "Could not determine path to save installation logs".to_owned(),
      ))
    }
  };
  create_dir(log_path)?;
  let mut file_options = std::fs::OpenOptions::new();
  file_options.create(true);
  if append {
    file_options.append(true);
  } else {
    file_options.write(true).truncate(true);
  }
  let file = file_options.open(log_path.join(name))?;
  Ok(file)
}
