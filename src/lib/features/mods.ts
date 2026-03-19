import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";
import type { ModVersion } from "$lib/rpc/bindings/ModVersion";
import thumbnailPlaceholder from "$assets/images/mod-thumbnail-placeholder.webp";
import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import { platform } from "@tauri-apps/plugin-os";

export function getModAssetUrl(
  version: ModVersion,
): string | null | undefined {
  return version.assets[platform()];
}

export function isModNotFiltered(
  modDisplayName: string,
  modTags: string[],
  modFilter: string,
): boolean {
  return (
    modFilter === "" ||
    modDisplayName.toLowerCase().includes(modFilter.toLowerCase()) ||
    modTags.join(",").toLowerCase().includes(modFilter.toLowerCase())
  );
}

export function getModDisplayName(
  modName: string,
  modSourceName: string,
  modSourceData: Record<string, ModSourceData>,
): string {
  const source = Object.values(modSourceData).find(
    (s) => s.sourceName === modSourceName && modName in s.mods,
  );
  return source?.mods[modName]?.displayName ?? modName;
}

export function getModTags(
  modName: string,
  modSourceName: string,
  modSourceData: Record<string, ModSourceData>,
): string[] {
  const source = Object.values(modSourceData).find(
    (s) => s.sourceName === modSourceName && modName in s.mods,
  );
  return source?.mods[modName]?.tags ?? [];
}

export function getModThumbnailImage(
  activeGame: SupportedGame,
  modInfo: ModInfo | undefined,
): string {
  if (!modInfo) {
    return thumbnailPlaceholder;
  }
  // Prefer pre-game-config if available
  if (
    modInfo.perGameConfig !== null &&
    modInfo.perGameConfig.hasOwnProperty(activeGame) &&
    modInfo.perGameConfig[activeGame]?.thumbnailArtUrl
  ) {
    return modInfo.perGameConfig[activeGame].thumbnailArtUrl;
  } else if (modInfo.thumbnailArtUrl !== null) {
    return modInfo.thumbnailArtUrl;
  }
  return thumbnailPlaceholder;
}
