import { invoke_rpc } from "./rpc";

export type VersionFolders = null | "official" | "unofficial" | "devel";

export async function listDownloadedVersions(
  versionFolder: VersionFolders,
): Promise<string[]> {
  return await invoke_rpc(
    "list_downloaded_versions",
    { versionFolder },
    () => [],
  );
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
  url: String,
): Promise<boolean> {
  return await invoke_rpc(
    "download_version",
    { version, url, versionFolder: "official" },
    () => false,
    "Unable to download official version",
    () => true,
  );
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
  versionFolder: String,
): Promise<boolean> {
  return await invoke_rpc(
    "remove_version",
    { version, versionFolder },
    () => false,
    "Unable to remove version",
    () => true,
  );
}

export async function openVersionFolder(versionFolder: VersionFolders) {
  return await invoke_rpc(
    "go_to_version_folder",
    { versionFolder },
    () => {},
    "Unable to open version folder",
  );
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
  return await invoke_rpc("get_active_tooling_version", {}, () => null);
}

export async function getActiveVersionFolder(): Promise<VersionFolders> {
  return await invoke_rpc("get_active_tooling_version_folder", {}, () => null);
}

export async function ensureActiveVersionStillExists(): Promise<boolean> {
  return await invoke_rpc(
    "ensure_active_version_still_exists",
    {},
    () => false,
    "Error checking that active version exists",
  );
}