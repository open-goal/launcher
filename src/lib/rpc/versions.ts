import { invoke } from "@tauri-apps/api/tauri";

export enum VersionFolders {
  OFFICIAL = "official",
  UNOFFICIAL = "unofficial",
  DEVEL = "devel",
}

export async function listDownloadedVersions(
  folder: VersionFolders
): Promise<string[]> {
  try {
    return await invoke("list_downloaded_versions", { versionFolder: folder });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function downloadOfficialVersion(
  version: String,
  url: String
): Promise<string[]> {
  try {
    return await invoke("download_official_version", { version: version, url });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function openVersionFolder(folder: VersionFolders) {
  try {
    return await invoke("go_to_version_folder", { versionFolder: folder });
  } catch (e) {
    console.log("TODO AH!");
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
    console.log("TODO AH!");
  }
}

export async function getActiveVersion() {
  try {
    return await invoke("get_active_version", {});
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function getActiveVersionFolder() {
  try {
    return await invoke("get_active_version_folder", {});
  } catch (e) {
    console.log("TODO AH!");
  }
}
