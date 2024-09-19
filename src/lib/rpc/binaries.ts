import { filePrompt } from "$lib/utils/file-dialogs";
import { invoke_rpc } from "./rpc";

interface InstallationOutput {
  msg: string | null;
  success: boolean;
}

function failed(msg: string): InstallationOutput {
  return { success: false, msg };
}

export async function updateDataDirectory(
  gameName: string,
): Promise<InstallationOutput> {
  return await invoke_rpc("update_data_directory", { gameName }, () =>
    failed("Failed to update data directory"),
  );
}

export async function getEndOfLogs(): Promise<string> {
  return await invoke_rpc("get_end_of_logs", {}, () => "");
}

export async function extractAndValidateISO(
  pathToIso: string,
  gameName: string,
): Promise<InstallationOutput> {
  return await invoke_rpc(
    "extract_and_validate_iso",
    { pathToIso, gameName },
    () => failed("Failed to extract and validate ISO"),
  );
}

export async function runDecompiler(
  pathToIso: string,
  gameName: string,
  truncateLogs: boolean = false,
): Promise<InstallationOutput> {
  return await invoke_rpc(
    "run_decompiler",
    { pathToIso, gameName, truncateLogs },
    () => failed("Failed to run decompiler"),
  );
}

export async function runCompiler(
  pathToIso: string,
  gameName: string,
  truncateLogs: boolean = false,
): Promise<InstallationOutput> {
  return await invoke_rpc(
    "run_compiler",
    { pathToIso, gameName, truncateLogs },
    () => failed("Failed to run compiler"),
  );
}

export async function getLaunchGameString(gameName: string): Promise<string> {
  return await invoke_rpc(
    "get_launch_game_string",
    { gameName },
    () => "_mirror_",
  );
}

export async function launchGame(
  gameName: string,
  inDebug: boolean,
): Promise<void> {
  return await invoke_rpc(
    "launch_game",
    { gameName, inDebug, executableLocation: null },
    () => {},
    "_mirror_",
  );
}

export async function launchGameWithCustomExecutable(
  gameName: string,
): Promise<void> {
  // Get custom executable location
  const customExecutable = await filePrompt(["exe"], "executables", "pick exe");
  if (customExecutable !== null) {
    return await invoke_rpc(
      "launch_game",
      { gameName, inDebug: false, executableLocation: customExecutable},
      () => {},
      "_mirror_",
    );
  }
}

export async function openREPL(gameName: string): Promise<void> {
  return await invoke_rpc(
    "open_repl",
    { gameName },
    () => {},
    "Unable to open REPL",
  );
}
