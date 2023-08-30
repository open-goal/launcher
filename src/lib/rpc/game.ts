import { invoke_rpc } from "./rpc";

export async function uninstallGame(gameName: string): Promise<boolean> {
  return await invoke_rpc(
    "uninstall_game",
    { gameName },
    () => false,
    "Unable to uninstall game",
  );
}

export async function resetGameSettings(gameName: string): Promise<void> {
  return await invoke_rpc(
    "reset_game_settings",
    { gameName },
    () => {},
    "Unable to reset game settings",
  );
}

export async function getFurthestGameMilestone(
  gameName: string,
): Promise<String> {
  return await invoke_rpc(
    "get_furthest_game_milestone",
    { gameName },
    () => "geyser", // TODO - default for only jak 1 right now
    "Unable to get furthest game milestone",
  );
}
