import { getModSourcesData } from "$lib/rpc/cache";
import type { ModInfo } from "../ModInfo";

export async function getModInfo(modName: string, modSource: string): Promise<ModInfo> {
    const modSourceData = await getModSourcesData();

    const foundMod: ModInfo | undefined = Object.values(modSourceData).find(
        (source) =>
            source.sourceName === modSource && source.mods.hasOwnProperty(modName),
    )?.mods[modName];

    if (foundMod) {
        return { ...foundMod, name: modName, source: modSource };
    } else {
        // TODO: this should be a temporary fix until we land on a permanent solution (e.g. caching mod metadata)
        // couldn't find mod in modlists (user might be offline, or mod/list abandoned)
        // just create a dummy entry so they can at least launch game
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
        };
    }
}