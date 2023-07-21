import { toastStore } from "$lib/stores/ToastStore";
import { locale as svelteLocale } from "svelte-i18n";
import { errorLog } from "./logging";
import { invoke_rpc } from "./rpc";
import type { VersionFolders } from "./versions";

export async function oldDataDirectoryExists(): Promise<boolean> {
  return await invoke_rpc("has_old_data_directory", {}, () => false);
}

export async function deleteOldDataDirectory(): Promise<void> {
  return await invoke_rpc("delete_old_data_directory", {}, () => {});
}

export async function resetLauncherSettingsToDefaults(): Promise<boolean> {
  const success = await invoke_rpc(
    "reset_to_defaults",
    {},
    () => false,
    "Unable to reset settings"
  );
  return success != false;
}

export async function getInstallationDirectory(): Promise<string | null> {
  return await invoke_rpc("get_install_directory", {}, () => null);
}

export async function setInstallationDirectory(
  newDir: string
): Promise<string | null> {
  // TODO - not insanely crazy about this pattern (message in the response instead of the error)
  // consider changing it
  const errMsg = await invoke_rpc(
    "set_install_directory",
    { newDir },
    () => "Unexpected error occurred",
    "Invalid installation directory"
  );

  if (errMsg !== null) {
    // for RPC errors the log and toast are done by invoke_rpc
    // but this is a successful RPC, so we need to do it here
    errorLog("Unable to set install directory");
    toastStore.makeToast(errMsg, "error");
  }

  return errMsg;
}

export async function isAVXRequirementMet(
  force: boolean
): Promise<boolean | undefined> {
  return await invoke_rpc("is_avx_requirement_met", { force }, () => undefined);
}

export async function isOpenGLRequirementMet(
  force: boolean
): Promise<boolean | undefined> {
  const result = await invoke_rpc(
    "is_opengl_requirement_met",
    { force },
    () => undefined
  );

  if (typeof result !== "boolean") {
    return undefined;
  }

  return result;
}

export async function finalizeInstallation(gameName: string): Promise<void> {
  return await invoke_rpc("finalize_installation", { gameName }, () => {});
}

export async function isGameInstalled(gameName: string): Promise<boolean> {
  return await invoke_rpc("is_game_installed", { gameName }, () => false);
}

export async function getInstalledVersion(gameName: string): Promise<String> {
  return invoke_rpc("get_installed_version", { gameName }, () => null);
}

export async function getInstalledVersionFolder(
  gameName: string
): Promise<String> {
  return invoke_rpc("get_installed_version_folder", { gameName }, () => null);
}

export async function saveActiveVersionChange(
  versionFolder: VersionFolders,
  newActiveVersion: String
): Promise<boolean> {
  return invoke_rpc(
    "save_active_version_change",
    { versionFolder, newActiveVersion },
    () => false,
    "Couldn't save active version change"
  );
}

export async function getLocale(): Promise<string | null> {
  return await invoke_rpc("get_locale", {}, () => "en-US");
}

export async function setLocale(locale: string): Promise<void> {
  return await invoke_rpc(
    "set_locale",
    { locale },
    () => {},
    null, // no toast
    () => {
      svelteLocale.set(locale);
    }
  );
}

export async function setBypassRequirements(bypass: boolean): Promise<void> {
  return await invoke_rpc("set_bypass_requirements", { bypass }, () => {});
}

export async function getBypassRequirements(): Promise<boolean> {
  return await invoke_rpc("get_bypass_requirements", {}, () => false);
}

export async function getEnabledTexturePacks(
  gameName: string
): Promise<string[]> {
  try {
    return await invoke("get_enabled_texture_packs", { gameName: gameName });
  } catch (e) {
    exceptionLog("Unable to get currently enabled texture packs", e);
    return [];
  }
}

export async function cleanupEnabledTexturePacks(
  gameName: string,
  cleanupList: string[]
): Promise<void> {
  try {
    return await invoke("cleanup_enabled_texture_packs", {
      gameName: gameName,
      cleanupList: cleanupList,
    });
  } catch (e) {
    exceptionLog("Unable to cleanup currently enabled texture packs", e);
  }
}

export async function setEnabledTexturePacks(
  gameName: string,
  packs: string[]
): Promise<void> {
  try {
    return await invoke("set_enabled_texture_packs", {
      gameName: gameName,
      packs: packs,
    });
  } catch (e) {
    exceptionLog("Unable to set currently enabled texture packs", e);
  }
}
