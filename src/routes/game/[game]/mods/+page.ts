import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";
import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import {
  getInstalledMods,
  getModSources,
  getModSourcesData,
} from "$lib/rpc/features";
import type { PageLoad } from "./$types";
import { platform } from "@tauri-apps/plugin-os";

const isModVisible = (mod: ModInfo, game: SupportedGame): boolean => {
  // game support: root OR per-game block OR any version OR latest version (implicitly covered by "any")
  const supportsGame =
    mod.supportedGames?.includes(game) ||
    (mod.perGameConfig && game in mod.perGameConfig) ||
    (mod.versions?.some((v) => v.supportedGames?.includes(game)) ?? false);

  if (!supportsGame) return false;

  // platform support: any version with a non-empty asset URL for current platform
  return (
    mod.versions?.some((v) => {
      const url = v.assets?.[platform()];
      return typeof url === "string" && url.length > 0;
    }) ?? false
  );
};

const filterSourceData = (
  sourceData: Record<string, ModSourceData>,
  game: SupportedGame,
): Record<string, ModSourceData> => {
  const entries = Object.entries(sourceData).map(([url, src]) => {
    const mods = Object.fromEntries(
      Object.entries(src.mods).filter(([, mod]) => isModVisible(mod, game)),
    );

    // prune versions to only those that have assets for this platform
    for (const m of Object.values(mods)) {
      m.versions = (m.versions ?? []).filter((v) => {
        const a = v.assets?.[platform()];
        return typeof a === "string" && a.length > 0;
      });
    }

    return [url, { ...src, mods }] as const;
  });

  // drop sources with no remaining mods
  return Object.fromEntries(
    entries.filter(([, src]) => Object.keys(src.mods).length > 0),
  );
};

export const load = (async ({ params }) => {
  const game = params.game as SupportedGame;
  const [sources, rawSourceData, installedMods] = await Promise.all([
    getModSources(),
    getModSourcesData(),
    getInstalledMods(game),
  ]);
  const sourceData = filterSourceData(
    rawSourceData as Record<string, ModSourceData>,
    game,
  );
  console.log(sourceData);
  return { sources, sourceData, installedMods };
}) satisfies PageLoad;
