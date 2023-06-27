import { toastStore } from "$lib/stores/ToastStore";
import { invoke } from "@tauri-apps/api/tauri";
import type { SupportedGame } from "$lib/constants";

export interface ModVersion {
  version: String;
  description: String;
  games: Array<SupportedGame>;
  windows_bundle_url: String;
  linux_bundle_url: String;
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
): Promise<boolean> {
  try {
    await invoke("add_mod_list", {
      url: url,
      identifier: identifier
    });
  } catch (e) {
    // exceptionLog("Unable to add mod list", e);
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
    // exceptionLog("Unable to remove mod list", e);
    toastStore.makeToast("Unable to remove mod list", "error");
    return false;
  }
  return true;
}

export async function getModLists(): Promise<Array<ModList>> {
  try {
    return await invoke("get_mod_lists");
  } catch (e) {
    // exceptionLog("Unable to add mod list", e);
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