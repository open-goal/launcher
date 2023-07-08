import { toastStore } from "$lib/stores/ToastStore";
import { invoke } from "@tauri-apps/api/tauri";

export interface ModVersion {
  version: string;
  description: string;
  supportedgames: Array<string>;
  windowsurl: string;
  linuxsurl: string;
  installedgames: Array<string>;
}

export interface ModConfig {
  identifier: string;
  name: string;
  description: string;
  contributors: Array<string>;
  tags: Array<string>;
  versions: Array<ModVersion>;
}

export interface ModList {
  identifier: string;
  url: string | undefined;
  mods: Array<ModConfig>;
}

export async function addModList(
  url: String,
  identifier: String,
  mods_json: String,
): Promise<boolean> {
  try {
    console.log("adding ", mods_json);
    await invoke("add_mod_list", {
      url: url,
      identifier: identifier,
      modsJson: mods_json,
    });
  } catch (e) {
    console.error("Unable to add mod list: ", e);
    toastStore.makeToast("Unable to add mod list", "error");
    return false;
  }
  return true;
}

export async function removeModList(
  identifier: String,
): Promise<boolean> {
  try {
    await invoke("remove_mod_list", {
      identifier: identifier
    });
  } catch (e) {
    console.error("Unable to remove mod list", e);
    toastStore.makeToast("Unable to remove mod list", "error");
    return false;
  }
  return true;
}

export async function getModLists(): Promise<Array<ModList>> {
  try {
    return await invoke("get_mod_lists");
  } catch (e) {
    console.error("Unable to add mod list", e);
    toastStore.makeToast("Unable to get mod lists", "error");
    return null;
  }
}

export async function downloadModList(modList: ModList): Promise<any> {
  const resp = await fetch(modList.url);
  // TODO - handle error
  const modListJson = await resp.json();

  console.log(modListJson);
}

export async function getModDict(game_name: string): Promise<Object> {
  let modLists = await getModLists();
  console.log("found modlists: ", modLists);
  
  let modDict = {};
  for (let i in modLists) {
    let list = modLists[i];
    for (let j in list.mods) {
      let mod = list.mods[j];
      let supportsGame = false;
      for (let k in mod.versions) {
        let vers = mod.versions[k];
        if (vers.supportedgames.indexOf(game_name) != -1) {
          supportsGame = true;
        }
      }
      let tmp_id = list.identifier + "$$" + mod.identifier;
      modDict[tmp_id] = mod;
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