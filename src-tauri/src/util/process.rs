use anyhow::Result;
use std::process::ExitStatus;

use tokio::{
  io::{AsyncBufReadExt, AsyncWriteExt},
  sync::mpsc,
};

use crate::commands::CommandError;

use super::file::create_dir;
use tauri::{Emitter, Manager};

pub async fn create_log_file(
  app_handle: &tauri::AppHandle,
  file_name: String,
  append: bool,
) -> Result<tokio::fs::File> {
  let log_path = app_handle.path().app_log_dir()?;
  let file_path = log_path.join(file_name);
  create_dir(&log_path)?;

  tokio::fs::OpenOptions::new()
    .read(true)
    .create(true)
    .append(append)
    .write(!append)
    .truncate(!append)
    .open(file_path)
    .await
    .map_err(Into::into)
}

#[derive(Clone, serde::Serialize)]
struct LogPayload {
  logs: String,
}

pub async fn watch_process(
  log_file: &mut tokio::fs::File,
  child: &mut tokio::process::Child,
  app_handle: &tauri::AppHandle,
) -> Result<ExitStatus, CommandError> {
  let stdout = child.stdout.take().unwrap();
  let stderr = child.stderr.take().unwrap();

  let mut stdout_reader = tokio::io::BufReader::new(stdout).lines();
  let mut stderr_reader = tokio::io::BufReader::new(stderr).lines();
  let (log_sender, mut log_receiver) = mpsc::channel::<String>(200);
  let app_handle_clone = app_handle.clone();

  tokio::spawn(async move {
    while let Some(log) = log_receiver.recv().await {
      let _ = app_handle_clone.emit("log_update", LogPayload { logs: log });
    }
  });

  let process_status: ExitStatus;

  loop {
    tokio::select! {
        Ok(Some(line)) = stdout_reader.next_line() => {
          let formatted_line = format!("{line}\n").trim().to_string();
          if formatted_line != "\n" {
            log_sender.try_send(formatted_line.clone()).ok();
            log_file.write_all(formatted_line.as_bytes()).await?;
            log_file.flush().await?;
          }
        },
        Ok(Some(line)) = stderr_reader.next_line() => {
          let formatted_line = format!("{line}\n").trim().to_string();
          if formatted_line != "\n" {
            log_sender.try_send(formatted_line.clone()).ok();
            log_file.write_all(formatted_line.as_bytes()).await?;
            log_file.flush().await?;
          }
        },
        status = child.wait() => {
          process_status = status?;
          drop(log_sender);
          break;
        }
    }
  }
  Ok(process_status)
}

pub fn create_std_log_file(
  app_handle: &tauri::AppHandle,
  file_name: String,
  append: bool,
) -> Result<std::fs::File> {
  let log_path = app_handle.path().app_log_dir()?;
  let file_path = log_path.join(&file_name);
  create_dir(&log_path)?;

  std::fs::OpenOptions::new()
    .create(true)
    .append(append)
    .write(!append)
    .truncate(!append)
    .open(file_path)
    .map_err(Into::into)
}
