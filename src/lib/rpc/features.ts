import { toastStore } from "$lib/stores/ToastStore";
import type { ModInfo } from "./bindings/ModInfo";
import type { ModSourceData } from "./bindings/ModSourceData";
import { errorLog } from "./logging";
import { invoke_rpc, invoke_rpc2 } from "./rpc";
import { unwrapFunctionStore, format } from "svelte-i18n";

const $format = unwrapFunctionStore(format);

export async function listExtractedTexturePackInfo(
  gameName: string,
): Promise<any> {
  return await invoke_rpc("list_extracted_texture_pack_info", {
    gameName: gameName,
  });
}

export async function extractNewTexturePack(
  gameName: string,
  pathToZip: string,
): Promise<string | null> {
  return await invoke_rpc2("extract_new_texture_pack", {
    args: {
      gameName: gameName,
      zipPath: pathToZip,
    },
  });
}

export async function updateTexturePackData(
  gameName: string,
): Promise<string | null> {
  return await invoke_rpc2("update_texture_pack_data", {
    args: {
      gameName: gameName,
    },
  });
}

export async function deleteTexturePacks(
  gameName: string,
  packs: string[],
): Promise<string | null> {
  return await invoke_rpc2("delete_texture_packs", {
    args: {
      gameName: gameName,
      packs: packs,
    },
  });
}

// TODO: refactor, this function is doing too much we CAN and SHOULD handle the verification on the backend.
export async function addModSource(
  url: string,
  currentSources: Record<string, ModSourceData>,
): Promise<string | null> {
  // Check that the URL is valid, easiest to do this on the client-side
  try {
    const sourceResp = await fetch(url);
    if (sourceResp.status !== 200) {
      toastStore.makeToast(
        `${$format("toasts_modSourceUnreachable")} - Status ${sourceResp.status}`,
        "error",
      );
      return "invalid mod source";
    }
  } catch (e) {
    errorLog(`Unable to add mod source: ${e}`);
    toastStore.makeToast(`${$format("toasts_modSourceUnreachable")}`, "error");
    return "Unable to add mod source";
  }

  return await invoke_rpc2("update_setting_value", {
    args: {
      key: "add_mod_source",
      val: url,
    },
  });
}

export async function removeModSource(modSource: string): Promise<void> {
  await invoke_rpc("update_setting_value", {
    key: "remove_mod_source",
    val: modSource,
  });
}

export async function getModSourceUrls(): Promise<string[]> {
  return await invoke_rpc("get_setting_value", { key: "mod_sources" });
}

export async function extractNewMod(
  gameName: string,
  bundlePath: string,
  modSource: string,
): Promise<string | null> {
  return await invoke_rpc2("extract_new_mod", {
    args: { gameName, bundlePath, modSource },
  });
}

/** extract the file into `install_dir/features/<gameName>/<sourceName>/<modName>` */
export async function downloadAndExtractNewMod(
  gameName: string,
  downloadUrl: string,
  modName: string,
  sourceName: string,
): Promise<string | null> {
  return await invoke_rpc2("download_and_extract_new_mod", {
    args: { gameName, downloadUrl, modName, sourceName },
  });
}

export async function getLocallyPersistedModInfo(
  gameName: string,
  modName: string,
  sourceName: string,
): Promise<ModInfo | undefined> {
  return await invoke_rpc("get_locally_persisted_mod_info", {
    gameName,
    modName,
    sourceName,
  });
}

export async function baseGameIsoExists(gameName: string): Promise<boolean> {
  return await invoke_rpc("base_game_iso_exists", { gameName });
}

export async function extractIsoForModInstall(
  gameName: string,
  modName: string | undefined,
  sourceName: string | undefined,
  pathToIso: string,
): Promise<string | null> {
  return await invoke_rpc2("extract_iso_for_mod_install", {
    args: { gameName, modName, sourceName, pathToIso },
  });
}

export async function decompileForModInstall(
  gameName: string,
  modName: string | undefined,
  sourceName: string | undefined,
): Promise<string | null> {
  return await invoke_rpc2("decompile_for_mod_install", {
    args: { gameName, modName, sourceName },
  });
}

export async function compileForModInstall(
  gameName: string,
  modName: string | undefined,
  sourceName: string | undefined,
): Promise<string | null> {
  return await invoke_rpc2("compile_for_mod_install", {
    args: { gameName, modName, sourceName },
  });
}

export async function saveModInstallInfo(
  gameName: string,
  modName: string | undefined,
  sourceName: string | undefined,
  versionName: string | undefined,
): Promise<string | null> {
  return await invoke_rpc2("save_mod_install_info", {
    args: { gameName, modName, sourceName, versionName },
  });
}

export async function getInstalledModsByGame(
  gameName: string,
): Promise<Record<string, Record<string, string>>> {
  return await invoke_rpc("get_setting_value", {
    key: "installed_mods",
    gameName,
  });
}

export async function launchMod(
  gameName: string,
  inDebug: boolean,
  modName: string,
  sourceName: string,
): Promise<void> {
  return await invoke_rpc("launch_mod", {
    gameName,
    inDebug,
    modName,
    sourceName,
  });
}

export async function getLocalModThumbnailBase64(
  gameName: string,
  modName: string,
): Promise<string> {
  return await invoke_rpc("get_local_mod_thumbnail_base64", {
    gameName,
    modName,
  });
}

export async function uninstallMod(
  gameName: string,
  modName: string,
  sourceName: string,
): Promise<void> {
  return await invoke_rpc("uninstall_mod", { gameName, modName, sourceName });
}

export async function resetModSettings(
  gameName: string,
  modName: string,
  sourceName: string,
): Promise<void> {
  return await invoke_rpc("reset_mod_settings", {
    gameName,
    modName,
    sourceName,
  });
}

export async function getLaunchModString(
  gameName: string,
  modName: string,
  sourceName: string,
): Promise<string> {
  return await invoke_rpc("get_launch_mod_string", {
    gameName,
    modName,
    sourceName,
  });
}

export async function openREPLForMod(
  gameName: string,
  modName: string,
  sourceName: string,
): Promise<void> {
  return await invoke_rpc("open_repl_for_mod", {
    gameName,
    modName,
    sourceName,
  });
}
