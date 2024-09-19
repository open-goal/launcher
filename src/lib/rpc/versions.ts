import { invoke_rpc } from "./rpc";

export type VersionFolders = null | "official";

export async function listDownloadedVersions(
  versionFolder: VersionFolders,
): Promise<string[]> {
  return await invoke_rpc(
    "list_downloaded_versions",
    { versionFolder },
    () => [],
  );
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
