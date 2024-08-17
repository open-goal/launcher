import { toastStore } from "$lib/stores/ToastStore";
import { invoke_rpc } from "./rpc";

// TODO - just make this a generic interface for both binaries/feature jobs
interface FeatureJobOutput {
  msg: string | null;
  success: boolean;
}

function failed(msg: string): FeatureJobOutput {
  toastStore.makeToast(msg, "error");
  return { success: false, msg };
}

export async function listExtractedTexturePackInfo(
  gameName: string,
): Promise<any> {
  return await invoke_rpc(
    "list_extracted_texture_pack_info",
    {
      gameName: gameName,
    },
    () => [],
  );
}

export async function extractNewTexturePack(
  gameName: string,
  pathToZip: string,
): Promise<boolean | undefined> {
  return await invoke_rpc(
    "extract_new_texture_pack",
    {
      gameName: gameName,
      zipPath: pathToZip,
    },
    () => undefined,
  );
}

export async function updateTexturePackData(
  gameName: string,
): Promise<FeatureJobOutput> {
  return await invoke_rpc(
    "update_texture_pack_data",
    {
      gameName: gameName,
    },
    () => failed("Failed to delete texture packs"),
    undefined,
    () => {
      return { success: true, msg: null };
    },
  );
}

export async function deleteTexturePacks(
  gameName: string,
  packs: string[],
): Promise<FeatureJobOutput> {
  return await invoke_rpc(
    "delete_texture_packs",
    {
      gameName: gameName,
      packs: packs,
    },
    () => failed("Failed to delete texture packs"),
    undefined,
    () => {
      return { success: true, msg: null };
    },
  );
}

export async function addModSource(url: string): Promise<boolean> {
  // Check that the URL is valid, easiest to do this on the client-side
  try {
    const sourceResp = await fetch(url);
    if (sourceResp.status !== 200) {
      toastStore.makeToast(
        `Mod source unreachable - Status ${sourceResp.status}`,
        "error",
      );
      return false;
    }
    // TODO - ensure it doesnt have the same name as another already added source!
  } catch (e) {
    toastStore.makeToast(`Mod source unreachable`, "error");
    return false;
  }

  return await invoke_rpc(
    "add_mod_source",
    {
      url: url,
    },
    () => false,
    "Unable to add mod source",
    () => true,
  );
}

export async function removeModSource(modSourceIndex: number): Promise<void> {
  await invoke_rpc(
    "remove_mod_source",
    {
      modSourceIndex: modSourceIndex,
    },
    () => {
      toastStore.makeToast("Unable to remove mod source", "error");
    },
  );
}

export interface ModSource {
  url: string;
}

export async function getModSources(): Promise<ModSource[]> {
  return await invoke_rpc("get_mod_sources", {}, () => []);
}

export async function extractNewMod(
  gameName: string,
  zipPath: string,
  modSource: string,
): Promise<boolean> {
  // TODO - error handling
  return await invoke_rpc(
    "extract_new_mod",
    { gameName, zipPath, modSource },
    () => false,
  );
}

export async function downloadAndExtractNewMod(
  gameName: string,
  downloadUrl: string,
  modName: string,
  sourceName: string,
): Promise<boolean> {
  // TODO - error handling
  return await invoke_rpc(
    "download_and_extract_new_mod",
    { gameName, downloadUrl, modName, sourceName },
    () => false,
  );
}

export async function baseGameIsoExists(gameName: string): Promise<boolean> {
  return await invoke_rpc("base_game_iso_exists", { gameName }, () => false);
}

interface InstallationOutput {
  msg: string | null;
  success: boolean;
}

export async function extractIsoForModInstall(
  gameName: string,
  modName: string | undefined,
  sourceName: string | undefined,
  pathToIso: string,
): Promise<InstallationOutput> {
  return await invoke_rpc(
    "extract_iso_for_mod_install",
    { gameName, modName, sourceName, pathToIso },
    () => failed("Failed to extract and validate ISO"),
  );
}

export async function decompileForModInstall(
  gameName: string,
  modName: string | undefined,
  sourceName: string | undefined,
): Promise<InstallationOutput> {
  return await invoke_rpc(
    "decompile_for_mod_install",
    { gameName, modName, sourceName },
    () => failed("Failed to decompile"),
  );
}

export async function compileForModInstall(
  gameName: string,
  modName: string | undefined,
  sourceName: string | undefined,
): Promise<InstallationOutput> {
  return await invoke_rpc(
    "compile_for_mod_install",
    { gameName, modName, sourceName },
    () => failed("Failed to compile"),
  );
}

export async function saveModInstallInfo(
  gameName: string,
  modName: string | undefined,
  sourceName: string | undefined,
  versionName: string | undefined,
): Promise<InstallationOutput> {
  return await invoke_rpc(
    "save_mod_install_info",
    { gameName, modName, sourceName, versionName },
    () => failed("Failed to save mod install info"),
  );
}

export async function getInstalledMods(
  gameName: string,
): Promise<Record<string, Record<string, string>>> {
  return await invoke_rpc("get_installed_mods", { gameName }, () => {
    let ret: Record<string, Record<string, string>> = {};
    return ret;
  });
}

export async function launchMod(
  gameName: string,
  inDebug: boolean,
  modName: string,
  sourceName: string,
): Promise<void> {
  return await invoke_rpc(
    "launch_mod",
    { gameName, inDebug, modName, sourceName },
    () => {},
    "_mirror_",
  );
}

export async function getLocalModThumbnailBase64(
  gameName: string,
  modName: string,
): Promise<string> {
  return await invoke_rpc(
    "get_local_mod_thumbnail_base64",
    { gameName, modName },
    () => "",
    "_mirror_",
  );
}

export async function getLocalModCoverBase64(
  gameName: string,
  modName: string,
): Promise<string> {
  return await invoke_rpc(
    "get_local_mod_cover_base64",
    { gameName, modName },
    () => "",
    "_mirror_",
  );
}

export async function uninstallMod(
  gameName: string,
  modName: string,
  sourceName: string,
): Promise<void> {
  return await invoke_rpc(
    "uninstall_mod",
    { gameName, modName, sourceName },
    () => {},
    "_mirror_",
  );
}

export async function resetModSettings(
  gameName: string,
  modName: string,
  sourceName: string,
): Promise<void> {
  return await invoke_rpc(
    "reset_mod_settings",
    { gameName, modName, sourceName },
    () => {},
    "Unable to reset mod settings",
  );
}

export async function getLaunchModString(
  gameName: string,
  modName: string,
  sourceName: string,
): Promise<string> {
  return await invoke_rpc(
    "get_launch_mod_string",
    { gameName, modName, sourceName },
    () => "_mirror_",
  );
}

export async function openREPLForMod(
  gameName: string,
  modName: string,
  sourceName: string,
): Promise<void> {
  return await invoke_rpc(
    "open_repl_for_mod",
    { gameName, modName, sourceName },
    () => {},
    "Unable to open REPL for mod",
  );
}
