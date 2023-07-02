import { toastStore } from "$lib/stores/ToastStore";
import { invoke } from "@tauri-apps/api/tauri";
import { exceptionLog } from "./logging";
import { getModLists } from "$lib/utils/mods";

export type VersionFolders = null | "official" | "unofficial" | "devel";

export async function listDownloadedVersions(
  folder: VersionFolders
): Promise<string[]> {
  try {
    return await invoke("list_downloaded_versions", { versionFolder: folder });
  } catch (e) {
    exceptionLog("Unable to list out downloaded versions", e);
    return [];
  }
}

export async function listUnofficialDownloadedVersions(
  versionPath: String
): Promise<string[]> {
  try {
    return await invoke("list_downloaded_versions", { versionFolder: `unofficial/${versionPath}` });
  } catch (e) {
    exceptionLog("Unable to list out downloaded versions", e);
    return [];
  }
}

export async function downloadOfficialVersion(
  version: String,
  url: String
): Promise<boolean> {
  try {
    await invoke("download_version", {
      version: version,
      versionFolder: "official",
      url: url,
    });
  } catch (e) {
    exceptionLog("Unable to download official version", e);
    toastStore.makeToast(e, "error");
    return false;
  }
  return true;
}

export async function downloadUnofficialVersion(
  version: String,
  versionFolder: String,
  url: String
): Promise<boolean> {
  try {
    await invoke("download_version", {
      version: version,
      versionFolder: `unofficial/${versionFolder}`,
      url: url,
    });
  } catch (e) {
    exceptionLog("Unable to download unofficial version", e);
    toastStore.makeToast(e, "error");
    return false;
  }
  return true;
}

export async function removeVersion(
  version: String,
  versionFolder: String
): Promise<boolean> {
  try {
    await invoke("remove_version", {
      version: version,
      versionFolder: versionFolder,
    });
  } catch (e) {
    exceptionLog("Unable to remove version", e);
    toastStore.makeToast("Unable to remove version", "error");
    return false;
  }
  return true;
}

export async function openVersionFolder(folder: VersionFolders) {
  console.log(`opening version folder: '${folder}'`);
  try {
    return await invoke("go_to_version_folder", { versionFolder: folder });
  } catch (e) {
    exceptionLog("Unable to open version folder", e);
    toastStore.makeToast("Unable to open version folder", "error");
  }
}

export async function openUnofficialVersionFolder(folder: String) {
  console.log(`opening unofficial folder: 'unofficial\\${folder}'`);
  try {
    return await invoke("go_to_version_folder", { versionFolder: `unofficial\\${folder}` });
  } catch (e) {
    exceptionLog("Unable to open version folder", e);
    toastStore.makeToast("Unable to open version folder", "error");
  }
}

export async function getActiveVersion(): Promise<string | null> {
  try {
    return await invoke("get_active_tooling_version", {});
  } catch (e) {
    exceptionLog("Unable to get active version", e);
    return null;
  }
}

export async function getActiveVersionFolder(): Promise<VersionFolders> {
  try {
    return await invoke("get_active_tooling_version_folder", {});
  } catch (e) {
    exceptionLog("Unable to get active version type", e);
    return null;
  }
}

export async function ensureActiveVersionStillExists(): Promise<boolean> {
  try {
    return await invoke("ensure_active_version_still_exists", {});
  } catch (e) {
    exceptionLog("Unable to check or remove broken active version", e);
    return false;
  }
}