import { invoke_rpc } from "./rpc";

export async function uninstallGame(gameName: string): Promise<void> {
  // TODO - don't assume success
  return await invoke_rpc("uninstall_game", { gameName }, () => {});
}

export async function resetGameSettings(gameName: string): Promise<void> {
  return await invoke_rpc("reset_game_settings", { gameName }, () => {});
}
