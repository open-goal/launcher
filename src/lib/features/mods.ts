import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
import type { ModVersion } from "$lib/rpc/bindings/ModVersion";

export function isVersionSupportedOnPlatform(
  userPlatform: string,
  version: ModVersion,
): boolean {
  if (userPlatform === "linux") {
    return (
      Object.keys(version.assets).includes("linux") &&
      version.assets["linux"] !== null
    );
  } else if (userPlatform == "darwin") {
    return (
      Object.keys(version.assets).includes("macos") &&
      version.assets["macos"] !== null
    );
  }
  return (
    Object.keys(version.assets).includes("windows") &&
    version.assets["windows"] !== null
  );
}

export function getModAssetUrl(
  userPlatform: string,
  version: ModVersion,
): string | undefined {
  if (userPlatform === "linux" && version.assets["linux"] !== null) {
    return version.assets["linux"];
  } else if (userPlatform == "darwin" && version.assets["macos"] !== null) {
    return version.assets["macos"];
  } else if (userPlatform == "win32" && version.assets["windows"] !== null) {
    return version.assets["windows"];
  }
  return undefined;
}

export function getModAssetUrlFromLatestVersion(
  userPlatform: string,
  modInfo: ModInfo,
): string | undefined {
  if (!(modInfo.versions.length > 0)) {
    return undefined;
  }
  if (
    userPlatform === "linux" &&
    modInfo.versions[0].assets["linux"] !== null
  ) {
    return modInfo.versions[0].assets["linux"];
  } else if (
    userPlatform == "darwin" &&
    modInfo.versions[0].assets["macos"] !== null
  ) {
    return modInfo.versions[0].assets["macos"];
  } else if (
    userPlatform == "win32" &&
    modInfo.versions[0].assets["windows"] !== null
  ) {
    return modInfo.versions[0].assets["windows"];
  }
  return undefined;
}

export function isLatestVersionOfModSupportedOnCurrentPlatform(
  userPlatform: string,
  modInfo: ModInfo,
): boolean {
  if (!(modInfo.versions.length > 0)) {
    return false;
  }
  if (userPlatform === "linux") {
    return (
      Object.keys(modInfo.versions[0].assets).includes("linux") &&
      modInfo.versions[0].assets["linux"] !== null
    );
  } else if (userPlatform == "darwin") {
    return (
      Object.keys(modInfo.versions[0].assets).includes("macos") &&
      modInfo.versions[0].assets["macos"] !== null
    );
  }
  return (
    Object.keys(modInfo.versions[0].assets).includes("windows") &&
    modInfo.versions[0].assets["windows"] !== null
  );
}
