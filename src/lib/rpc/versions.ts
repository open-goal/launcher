import { invoke } from "@tauri-apps/api/tauri";

export type VersionFolders = undefined | "official" | "unofficial" | "devel";

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
): Promise<boolean> {
  try {
    await invoke("download_version", {
      version: version,
      versionFolder: "official",
      url: url,
    });
  } catch (e) {
    // TODO - toast notification
    console.log(
      "[OG] Problem encountered when attempting to download version: ",
      e
    );
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
    console.log(
      "[OG] Problem encountered when attempting to remove version: ",
      e
    );
    return false;
  }
  return true;
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
    console.log("Could not save active version change", e);
  }
}

export async function getActiveVersion(): Promise<string> {
  try {
    return await invoke("get_active_version", {});
  } catch (e) {
    console.log("[OG] Could not determine active version");
    return undefined;
  }
}

export async function getActiveVersionFolder(): Promise<VersionFolders> {
  try {
    return await invoke("get_active_version_folder", {});
  } catch (e) {
    console.log("[OG] Could not determine active version folder");
    return undefined;
  }
}
