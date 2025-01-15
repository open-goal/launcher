import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
import type { ModVersion } from "$lib/rpc/bindings/ModVersion";

export function isVersionSupportedOnPlatform(
  userPlatform: string,
  version: ModVersion,
): boolean {
  return (
    Object.hasOwn(version.assets, userPlatform) &&
    version.assets[userPlatform] !== null
  );
}

export function getModAssetUrl(
  userPlatform: string,
  version: ModVersion,
): string | undefined {
  return version.assets[userPlatform];
}

export function getModAssetUrlFromLatestVersion(
  userPlatform: string,
  modInfo: ModInfo,
): string | undefined {
  if (modInfo.versions.length == 0) {
    return undefined;
  }
  return modInfo.versions[0].assets[userPlatform];
}

export function isLatestVersionOfModSupportedOnCurrentPlatform(
  userPlatform: string,
  modInfo: ModInfo,
): boolean {
  if (!(modInfo.versions.length > 0)) {
    return false;
  }

  return (
    Object.hasOwn(modInfo.versions[0].assets, userPlatform) &&
    modInfo.versions[0].assets[userPlatform] !== null
  );
}
