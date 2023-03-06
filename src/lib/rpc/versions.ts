import { invoke } from "@tauri-apps/api/tauri";
import { exceptionLog } from "./logging";

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
    // TODO - toast?
    exceptionLog("Unable to download official version", e);
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
    // TODO - toast notification
    exceptionLog("Unable to remove version", e);
    return false;
  }
  return true;
}

export async function openVersionFolder(folder: VersionFolders) {
  try {
    return await invoke("go_to_version_folder", { versionFolder: folder });
  } catch (e) {
    exceptionLog("Unable to open version folder", e);
    // TODO - toast
  }
}

export async function saveActiveVersionChange(
  folder: VersionFolders,
  newVersion: String
) {
  try {
    return await invoke("save_active_version_change", {
      versionFolder: folder,
      newActiveVersion: newVersion,
    });
  } catch (e) {
    exceptionLog("Unable to save version change", e);
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
