import { filePromptNoFilters } from "$lib/utils/file-dialogs";
import type { SupportedGame } from "./bindings/SupportedGame";
import { invoke_rpc, invoke_rpc2 } from "./rpc";

export async function updateDataDirectory(
  gameName: string,
): Promise<string | null> {
  return await invoke_rpc2("update_data_directory", { args: { gameName } });
}

export async function extractAndValidateISO(
  pathToIso: string,
  gameName: string,
): Promise<string | null> {
  return await invoke_rpc2("extract_and_validate_iso", {
    args: { pathToIso, gameName },
  });
}

export async function runDecompiler(
  pathToIso: string | null,
  gameName: string,
  truncateLogs: boolean = false,
  useDecompSettings: boolean = false,
): Promise<string | null> {
  return await invoke_rpc2("run_decompiler", {
    args: {
      pathToIso,
      gameName,
      truncateLogs,
      useDecompSettings,
    },
  });
}

export async function runCompiler(
  pathToIso: string | null,
  gameName: string,
  truncateLogs: boolean = false,
): Promise<string | null> {
  return await invoke_rpc2("run_compiler", {
    args: {
      pathToIso,
      gameName,
      truncateLogs,
    },
  });
}

export async function getLaunchGameString(gameName: string): Promise<string> {
  return await invoke_rpc("get_launch_game_string", { gameName });
}

export async function launchGame(
  gameName: SupportedGame,
  inDebug: boolean,
): Promise<void> {
  return await invoke_rpc("launch_game", {
    gameName,
    inDebug,
    executableLocation: null,
  });
}

export async function launchGameWithCustomExecutable(
  gameName: string,
): Promise<void> {
  // Get custom executable location
  const customExecutable = await filePromptNoFilters("Select custom 'gk'");
  if (customExecutable !== null) {
    return await invoke_rpc("launch_game", {
      gameName,
      inDebug: false,
      executableLocation: customExecutable,
    });
  }
}

export async function openREPL(gameName: string): Promise<void> {
  return await invoke_rpc("open_repl", { gameName });
}
