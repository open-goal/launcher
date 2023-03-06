import { invoke } from "@tauri-apps/api/tauri";
import { exceptionLog } from "./logging";

interface InstallationOutput {
  msg: string | null;
  success: boolean;
}

export async function updateDataDirectory(
  gameName: string
): Promise<InstallationOutput> {
  try {
    return await invoke("update_data_directory", {
      gameName: gameName,
    });
  } catch (e) {
    exceptionLog(
      "Unexpected error encountered when updating data directory",
      e
    );
    return {
      msg: "An unexpected error occurred",
      success: false,
    };
  }
}

export async function getEndOfLogs(): Promise<string> {
  try {
    return await invoke("get_end_of_logs", {});
  } catch (e) {
    exceptionLog(
      "Unexpected error encountered when tail'ing extractor logs",
      e
    );
    return "";
  }
}

export async function extractAndValidateISO(
  pathToIso: string,
  gameName: string
): Promise<InstallationOutput> {
  try {
    return await invoke("extract_and_validate_iso", {
      pathToIso: pathToIso,
      gameName: gameName,
    });
  } catch (e) {
    exceptionLog(
      "Unexpected error encountered when extracing and validating the ISO",
      e
    );
    return {
      msg: "An unexpected error occurred",
      success: false,
    };
  }
}

export async function runDecompiler(
  pathToIso: string,
  gameName: string,
  truncateLogs: boolean = false
): Promise<InstallationOutput> {
  try {
    return await invoke("run_decompiler", {
      pathToIso: pathToIso,
      gameName: gameName,
      truncateLogs: truncateLogs,
    });
  } catch (e) {
    exceptionLog("Unexpected error encountered when running the decompiler", e);
    return {
      msg: "An unexpected error occurred",
      success: false,
    };
  }
}

export async function runCompiler(
  pathToIso: string,
  gameName: string,
  truncateLogs: boolean = false
): Promise<InstallationOutput> {
  try {
    return await invoke("run_compiler", {
      pathToIso: pathToIso,
      gameName: gameName,
      truncateLogs: truncateLogs,
    });
  } catch (e) {
    exceptionLog("Unexpected error encountered when running the compiler", e);
    return {
      msg: "An unexpected error occurred",
      success: false,
    };
  }
}

export async function launchGame(
  gameName: string,
  inDebugMode: boolean
): Promise<void> {
  try {
    return await invoke("launch_game", {
      gameName: gameName,
      inDebug: inDebugMode,
    });
  } catch (e) {
    exceptionLog("Unexpected error encountered when launching the game", e);
    // TODO - toast
  }
}

export async function openREPL(gameName: string): Promise<void> {
  try {
    return await invoke("open_repl", {
      gameName: gameName,
    });
  } catch (e) {
    exceptionLog("Unexpected error encountered when opening the REPL", e);
    // TOOD - toast
  }
}
