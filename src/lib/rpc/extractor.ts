import { invoke } from "@tauri-apps/api/tauri";

export async function updateDataDirectory(gameName: string): Promise<void> {
  try {
    return await invoke("update_data_directory", {
      gameName: gameName,
    });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function extractAndValidateISO(
  pathToIso: string,
  gameName: string
): Promise<void> {
  try {
    return await invoke("extract_and_validate_iso", {
      pathToIso: pathToIso,
      gameName: gameName,
    });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function runDecompiler(
  pathToIso: string,
  gameName: string
): Promise<void> {
  try {
    return await invoke("run_decompiler", {
      pathToIso: pathToIso,
      gameName: gameName,
    });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function runCompiler(
  pathToIso: string,
  gameName: string
): Promise<void> {
  try {
    return await invoke("run_compiler", {
      pathToIso: pathToIso,
      gameName: gameName,
    });
  } catch (e) {
    console.log("TODO AH!");
  }
}
