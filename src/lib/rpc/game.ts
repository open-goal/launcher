import { invoke_rpc } from "./rpc";
import { toastStore } from "$lib/stores/ToastStore";

export async function uninstallGame(gameName: string): Promise<boolean> {
  return await invoke_rpc("uninstall_game", { gameName });
}

export async function resetGameSettings(gameName: string): Promise<void> {
  return await invoke_rpc("reset_game_settings", { gameName }, () => {
    toastStore.makeToast(`${gameName} settings reset`, "info");
  });
}

export async function getFurthestGameMilestone(
  gameName: string,
): Promise<String> {
  return await invoke_rpc("get_furthest_game_milestone", { gameName });
}
