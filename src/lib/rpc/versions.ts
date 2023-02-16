import { invoke } from "@tauri-apps/api/tauri";

export async function listDownloadedOfficialVersions(): Promise<string[]> {
  try {
    return await invoke("list_downloaded_official_versions", {});
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
