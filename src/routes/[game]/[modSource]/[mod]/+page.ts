import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
import { getModSourcesData } from "$lib/rpc/features";
import type { PageLoad } from "./$types";

export const load = (async ({ params, parent }) => {
  const { config } = await parent();
  const game = params.game;
  const source = params.modSource;
  const mod = params.mod;
  const gameConfig = config.games[game];
  const installedVersion = config.games[game].mods[source]?.[mod.displayName];

  const modSourceData = await getModSourcesData();
  const modInfo: ModInfo = modSourceData[source].mods[mod];

  return { mod, source, game, modInfo, installedVersion };
}) satisfies PageLoad;
