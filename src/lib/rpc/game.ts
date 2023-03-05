import { invoke } from "@tauri-apps/api/tauri";

export async function uninstallGame(gameName: string): Promise<void> {
  try {
    return await invoke("uninstall_game", {
      gameName: gameName,
    });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function resetGameSettings(gameName: string): Promise<void> {
  try {
    return await invoke("reset_game_settings", {
      gameName: gameName,
    });
  } catch (e) {
    console.log("TODO AH!");
  }
}
