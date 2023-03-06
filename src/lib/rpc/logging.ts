// On the rust side, we already have a great library that does things like:
// - rotation
// - coloring
// - formatting
//
// So let's use it and all logs from the frontend can go through there too
//
// This has a secondary benefit -- it means all logs will be in one place and
// ordered roughly as expected so things make sense
//
// This also eliminates the need to ask a user to open their debug console to
// get decent logs (hopefully)

import { invoke } from "@tauri-apps/api/tauri";

async function genericLog(level: string, log: string): Promise<void> {
  try {
    return await invoke("frontend_log", {
      level: level,
      log: log,
    });
  } catch (e) {
    console.log("[OG]: Unexpected error encountered when trying to log", e);
  }
}

export async function debugLog(log: string): Promise<void> {
  genericLog("debug", log);
}

export async function infoLog(log: string): Promise<void> {
  genericLog("info", log);
}

export async function warnLog(log: string): Promise<void> {
  genericLog("warn", log);
}

export async function errorLog(log: string): Promise<void> {
  genericLog("error", log);
}

export async function exceptionLog(log: string, error: Error): Promise<void> {
  genericLog(
    "error",
    `${log} | Exception: ${error.name}:${error.message}, Stack: ${error.stack}, Cause: ${error.cause}`
  );
}
