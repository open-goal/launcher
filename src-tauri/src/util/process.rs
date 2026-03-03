use anyhow::{Context, Result};
use std::process::ExitStatus;

use tokio::{
  io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
  sync::mpsc,
};

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
) -> Result<ExitStatus> {
  let stdout = child.stdout.take().context("Child stdout was not piped")?;
  let stderr = child.stderr.take().context("Child stderr was not piped")?;
  let mut stdout_lines = BufReader::new(stdout).lines();
  let mut stderr_lines = BufReader::new(stderr).lines();

  let (tx, mut rx) = mpsc::channel::<String>(200);
  let app = app_handle.clone();

  tokio::spawn(async move {
    while let Some(log) = rx.recv().await {
      let _ = app.emit("log_update", LogPayload { logs: log });
    }
  });

  loop {
    tokio::select! {
      line = stdout_lines.next_line() => {
        if let Some(line) = line.context("Failed reading stdout")? {
          handle_line(log_file, &tx, line).await?;
        }
      }
      line = stderr_lines.next_line() => {
        if let Some(line) = line.context("Failed reading stderr")? {
          handle_line(log_file, &tx, line).await?;
        }
      }
      status = child.wait() => {
        drop(tx);
        log_file.flush().await.context("Failed flushing log file")?;
        return Ok(status.context("Failed waiting for child process")?);
      }
    }
  }
}

async fn handle_line(
  log_file: &mut tokio::fs::File,
  tx: &mpsc::Sender<String>,
  line: String,
) -> Result<()> {
  if line.trim().is_empty() {
    return Ok(());
  }
  let _ = tx.try_send(line.clone());
  log_file.write_all(line.as_bytes()).await?;
  log_file.write_all(b"\n").await?;
  Ok(())
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
