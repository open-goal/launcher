import { invoke } from "@tauri-apps/api/tauri";
import { exceptionLog } from "./logging";

export async function uninstallGame(gameName: string): Promise<void> {
  try {
    return await invoke("uninstall_game", {
      gameName: gameName,
    });
  } catch (e) {
    exceptionLog("Unable to uninstall game", e);
    // TODO - don't assume success
  }
}

export async function resetGameSettings(gameName: string): Promise<void> {
  try {
    return await invoke("reset_game_settings", {
      gameName: gameName,
    });
  } catch (e) {
    exceptionLog("Unable to reset game settings", e);
  }
}
