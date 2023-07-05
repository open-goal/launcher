import { invoke_rpc } from "./rpc";

interface InstallationOutput {
  msg: string | null;
  success: boolean;
}

function failed(msg: string): InstallationOutput {
  return { success: false, msg };
}

export async function updateDataDirectory(
  gameName: string
): Promise<InstallationOutput> {
  return await invoke_rpc("update_data_directory", { gameName }, () =>
    failed("An unexpected error occurred")
  );
}

export async function getEndOfLogs(): Promise<string> {
  return await invoke_rpc("get_end_of_logs", {}, () => "");
}

export async function extractAndValidateISO(
  pathToIso: string,
  gameName: string
): Promise<InstallationOutput> {
  return await invoke_rpc(
    "extract_and_validate_iso",
    { pathToIso, gameName },
    () => failed("An unexpected error occurred")
  );
}

export async function runDecompiler(
  pathToIso: string,
  gameName: string,
  truncateLogs: boolean = false
): Promise<InstallationOutput> {
  return await invoke_rpc(
    "run_decompiler",
    { pathToIso, gameName, truncateLogs },
    () => failed("An unexpected error occurred")
  );
}

export async function runCompiler(
  pathToIso: string,
  gameName: string,
  truncateLogs: boolean = false
): Promise<InstallationOutput> {
  return await invoke_rpc(
    "run_compiler",
    { pathToIso, gameName, truncateLogs },
    () => failed("An unexpected error occurred")
  );
}

export async function launchGame(
  gameName: string,
  inDebug: boolean
): Promise<void> {
  return await invoke_rpc("launch_game", { gameName, inDebug }, () => {});
}

export async function openREPL(gameName: string): Promise<void> {
  return await invoke_rpc("open_repl", { gameName }, () => {});
}
