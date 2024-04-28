import { toastStore } from "$lib/stores/ToastStore";
import { invoke } from "@tauri-apps/api/tauri";

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

export async function getModDetails(
  game: string,
  modId: string,
): Promise<Object> {
  let modsDict = await getModDict(game);
  if (modsDict.hasOwnProperty(modId)) {
    return modsDict[modId];
  } else {
    console.error(`${game} not supported for ${modId}, couldn't load details`);
  }
  return null;
}
