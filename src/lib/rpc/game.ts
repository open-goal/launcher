import { invoke } from "@tauri-apps/api/tauri";

export async function launchGame(gameName: string): Promise<void> {
  try {
    return await invoke("launch_game", {
      gameName: gameName,
    });
  } catch (e) {
    console.log("TODO AH!");
  }
}
