import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";
import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import { getModSources, getModSourcesData } from "$lib/rpc/features";
import type { PageLoad } from "./$types";
import { platform } from "@tauri-apps/plugin-os";

const isModVisible = (mod: ModInfo): boolean => {
  return (
    mod.versions?.some((v) => {
      const url = v.assets?.[platform()];
      return typeof url === "string" && url.length > 0;
    }) ?? false
  );
};

const filterSourceData = (
  sourceData: Record<string, ModSourceData>,
): Record<string, ModSourceData> => {
  const entries = Object.entries(sourceData).map(([url, src]) => {
    const mods = Object.fromEntries(
      Object.entries(src.mods).filter(([, mod]) => isModVisible(mod)),
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

  return Object.fromEntries(
    entries.filter(([, src]) => Object.keys(src.mods).length > 0),
  );
};

const gamesForMod = (mod: ModInfo): SupportedGame[] => {
  const top = (mod as any).supportedGames as SupportedGame[] | undefined;
  const perVersion =
    mod.versions?.flatMap(
      (v: any) => (v.supportedGames as SupportedGame[] | undefined) ?? [],
    ) ?? [];

  const all = new Set<SupportedGame>([...(top ?? []), ...perVersion]);
  return ["jak1", "jak2", "jak3"].filter((g) =>
    all.has(g as SupportedGame),
  ) as SupportedGame[];
};

const groupModsByGame = (sourceData: Record<string, ModSourceData>) => {
  const grouped: Record<GameKey, ModInfo[]> = { jak1: [], jak2: [], jak3: [] };

  for (const src of Object.values(sourceData)) {
    for (const [name, mod] of Object.entries(src.mods)) {
      for (const g of gamesForMod(mod)) {
        grouped[g].push({ ...mod, source: src.sourceName, name: name });
      }
    }
  }
  return grouped;
};

export const load = (async ({ params, parent }) => {
  const { config } = await parent();
  const installedMods = {
    jak1: config.games.jak1?.mods,
    jak2: config.games.jak2?.mods,
    jak3: config.games.jak3?.mods,
  };
  const [sources, rawSourceData] = await Promise.all([
    getModSources(),
    getModSourcesData(),
  ]);
  const sourceData = filterSourceData(
    rawSourceData as Record<string, ModSourceData>,
  );
  const modsByGame = groupModsByGame(sourceData);
  return { sources, sourceData, installedMods, modsByGame };
}) satisfies PageLoad;
