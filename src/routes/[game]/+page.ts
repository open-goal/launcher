import { configDir, join } from "@tauri-apps/api/path";
import type { PageLoad } from "./$types";

export const load = (async ({ parent, params }) => {
  const game = params.game;
  const { config } = await parent();
  const gameDataDir = await join(
    config.installationDir,
    "active",
    game,
    "data",
  );
  const savesDir = await join(await configDir(), "OpenGOAL", game, "saves");
  const extractedAssetsDir = await join(gameDataDir, "decompiler_out", game);
  const settingsDir = await join(
    await configDir(),
    "OpenGOAL",
    game,
    "settings",
  );
  return { gameDataDir, extractedAssetsDir, savesDir, settingsDir };
}) satisfies PageLoad;
