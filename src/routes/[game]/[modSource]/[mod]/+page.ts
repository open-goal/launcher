import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
import { getModSourcesData } from "$lib/rpc/features";
import { join } from "@tauri-apps/api/path";
import type { PageLoad } from "./$types";
import { type } from "@tauri-apps/plugin-os";

export const load = (async ({ params, parent }) => {
  const { config } = await parent();
  const game = params.game;
  const source = params.modSource;
  const modName = params.mod;
  const gameConfig = config.games[game];
  const installedVersion = config.games[game].mods[source]?.[modName];

  const modSourceData = await getModSourcesData();
  const modInfo: ModInfo = modSourceData[source].mods[modName];
  const versions = modInfo.versions
    ?.filter((v) => v.assets[type()] !== null)
    .map((v) => ({ version: v.version, url: v.assets?.[type()] }));

  const gameDataDir = installedVersion
    ? await join(
        config.installationDir,
        "features",
        game,
        "mods",
        source,
        modName,
        "data",
      )
    : null;

  const extractedAssetsDir = gameDataDir
    ? await join(gameDataDir, "decompiler_out", game)
    : null;

  return {
    modName,
    source,
    game,
    modInfo,
    installedVersion,
    versions,
    gameDataDir,
    extractedAssetsDir,
  };
}) satisfies PageLoad;
