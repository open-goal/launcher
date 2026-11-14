import { getLatestOfficialRelease } from "$lib/utils/github";
import { invoke_rpc } from "./rpc";
import { config } from "/src/state/config.svelte";

export async function listDownloadedVersions(): Promise<string[]> {
  return await invoke_rpc("list_downloaded_versions", {
    versionFolder: "official",
  });
}

export async function downloadOfficialVersion(
  version: string,
  url: string,
): Promise<void> {
  return await invoke_rpc("download_version", {
    version,
    url,
    versionFolder: "official",
  });
}

export async function removeVersion(version: string): Promise<void> {
  return await invoke_rpc("remove_version", { version });
}

export async function removeOldVersions(): Promise<void> {
  if (!config?.deletePreviousVersions) return;

  const downloadedVersions = await listDownloadedVersions();
  const latestRelease = await getLatestOfficialRelease();
  if (!latestRelease?.version) return;

  const oldVersions = downloadedVersions.filter(
    (version) => version !== latestRelease?.version,
  );
  await Promise.all(oldVersions.map(removeVersion));
}

export async function ensureActiveVersionStillExists(): Promise<boolean> {
  return await invoke_rpc("ensure_active_version_still_exists");
}
