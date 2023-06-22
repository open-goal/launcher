import { toastStore } from "$lib/stores/ToastStore";
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

export async function getModDict(game: string): Promise<Object> {
  // TODO: load this from remote/file
  let rawModList = [
    {
      "id": "randomizer",
      "name": "Checkpoint Randomizer",
      "desc": "Warp to a random checkpoint on various triggers (cells, flies, orbs, eco, death).",
      "contributors": [
        "ZedB0T",
        "MikeGamePro",
        "barg034"
      ],
      "tags": [
        "gameplay-mod",
        "rng",
        "challenge"
      ],
      "repo": "OpenGOAL-Unofficial-Mods/opengoal-randomizer-mod-pack",
      "URL": "https://github.com/OpenGOAL-Unofficial-Mods/opengoal-randomizer-mod-pack/",
      "games": ["jak1"]
    },
    {
      "id": "flutflut_legacy",
      "name": "The FlutFlut Legacy",
      "desc": "Play the whole game on FlutFlut.",
      "contributors": [
        "ZedB0T",
        "barg034"
      ],
      "tags": [
        "gameplay-mod",
        "challenge"
      ],
      "repo": "OpenGOAL-Unofficial-Mods/flutflut-legacy",
      "URL": "https://github.com/OpenGOAL-Unofficial-Mods/flutflut-legacy",
      "games": ["jak1"]
    },
    {
      "id": "orb_hunt",
      "name": "Orb Hunt",
      "desc": "Jak 1 but collectables and other things have been moved around!",
      "contributors": [
        "barg034"
      ],
      "tags": [
        "gameplay-mod",
        "challenge"
      ],
      "repo": "dallmeyer/opengoal-orbhunt",
      "URL": "https://github.com/dallmeyer/opengoal-orbhunt",
      "games": ["jak1", "jak2"]
    }
  ];

  let modDict = {};
  for (const m of rawModList) {
    if (m["games"].indexOf(game) != -1) {
      // add mod to list if supported for the current game
      modDict[m["id"]] = m;
    } else {
      console.log(`${game} not supported for ${m["id"]}, skipping`);
    }
  }

  return modDict;
}

export async function getModDetails(game: string, modId: string): Promise<Object> {
  let modsDict = await getModDict(game);
  if (modsDict.hasOwnProperty(modId)) {
    return modsDict[modId];
  } else {
    console.error(`${game} not supported for ${modId}, couldn't load details`);
  }
  return null;
}