import { getModSourcesData } from "$lib/rpc/cache";
import { getLocallyPersistedModInfo } from "$lib/rpc/features";
import type { ModInfo } from "../ModInfo";
import type { SupportedGame } from "../SupportedGame";

export async function getModInfo(
  activeGame: SupportedGame,
  modName: string,
  modSource: string,
): Promise<ModInfo> {
  const modSourceData = await getModSourcesData();

  const foundMod: ModInfo | undefined = Object.values(modSourceData).find(
    (source) =>
      source.sourceName === modSource && source.mods.hasOwnProperty(modName),
  )?.mods[modName];

  if (foundMod) {
    return { ...foundMod, name: modName, source: modSource };
  } else {
    // If we could not get the information from the live mod source, then we have two fallbacks paths
    // - grab the local metadata file that is colocated along with the mod
    //   - this is also nice because it allows locally installed mods to test the same metadata as well
    const persistedMetadata = await getLocallyPersistedModInfo(
      activeGame,
      modName,
      modSource,
    );
    if (persistedMetadata) {
      return { ...persistedMetadata, metadataOffline: true };
    }
    // - if not, return sensible defaults
    return {
      name: modName,
      source: modSource,
      displayName: modName,
      description: "",
      authors: [],
      tags: [],
      supportedGames: [],
      versions: [],
      websiteUrl: null,
      perGameConfig: null,
      coverArtUrl: null,
      thumbnailArtUrl: null,
      externalLink: null,
      metadataOffline: true,
    };
  }
}
