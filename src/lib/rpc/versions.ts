import { getLatestOfficialRelease, type ReleaseInfo } from "$lib/utils/github";
import { getAutoUninstallOldVersions } from "./config";
import { invoke_rpc } from "./rpc";

export async function listDownloadedVersions(): Promise<string[]> {
  return await invoke_rpc(
    "list_downloaded_versions",
    { versionFolder: "official" },
    () => [],
  );
}

export async function downloadOfficialVersion(
  release: ReleaseInfo,
): Promise<boolean> {
  const { version, downloadUrl } = release;
  return await invoke_rpc(
    "download_version",
    { version, downloadUrl, versionFolder: "official" },
    () => false,
    "Unable to download official version",
    () => true,
  );
}

export async function downloadLatestOfficialVersion(): Promise<boolean> {
  const { version, downloadUrl } = await getLatestOfficialRelease();
  return await invoke_rpc(
    "download_version",
    { version, downloadUrl, versionFolder: "official" },
    () => false,
    "Unable to download official version",
    () => true,
  );
}

export async function removeVersion(release: ReleaseInfo): Promise<boolean> {
  const { version } = release;
  return await invoke_rpc(
    "remove_version",
    { version },
    () => false,
    "Unable to remove version",
    () => true,
  );
}

export async function removeOldVersions(): Promise<boolean> {
  let shouldRemove = await getAutoUninstallOldVersions();
  if (shouldRemove) {
    let downloadedVersions = await listDownloadedVersions();
    let latestRelease = await getLatestOfficialRelease();
    downloadedVersions = downloadedVersions.filter(
      (v) => v !== latestRelease?.version,
    );
    downloadedVersions.forEach((v) => {
      removeVersion(v);
    });
    return false;
  }
  return false;
}

export async function getActiveVersion(): Promise<string | null> {
  return await invoke_rpc(
    "get_setting_value",
    { key: "active_version" },
    () => null,
  );
}

export async function ensureActiveVersionStillExists(): Promise<boolean> {
  return await invoke_rpc(
    "ensure_active_version_still_exists",
    {},
    () => false,
    "Error checking that active version exists",
  );
}
