import { toastStore } from "$lib/stores/ToastStore";
import { invoke } from "@tauri-apps/api/tauri";
import { errorLog, exceptionLog } from "./logging";
import type { VersionFolders } from "./versions";
import { locale } from "svelte-i18n";

export async function oldDataDirectoryExists(): Promise<boolean> {
  try {
    return await invoke("has_old_data_directory", {});
  } catch (e) {
    exceptionLog("Unable to check if old data directory exists", e);
    return false;
  }
}

export async function deleteOldDataDirectory(): Promise<void> {
  try {
    await invoke("delete_old_data_directory", {});
  } catch (e) {
    exceptionLog("Unable to delete old data directory", e);
  }
}

export async function resetLauncherSettingsToDefaults(): Promise<boolean> {
  try {
    await invoke("reset_to_defaults", {});
    return true;
  } catch (e) {
    exceptionLog("Unable to reset launcher settings to defaults", e);
    toastStore.makeToast("Unable to reset settings", "error");
    return false;
  }
}

export async function getInstallationDirectory(): Promise<string | null> {
  try {
    return await invoke("get_install_directory", {});
  } catch (e) {
    exceptionLog("Unable to fetch install directory", e);
    return null;
  }
}

export async function setInstallationDirectory(
  newInstallDir: string
): Promise<string | null> {
  try {
    // TODO - not insanely crazy about this pattern (message in the response instead of the error)
    // consider changing it
    const errMsg: string = await invoke("set_install_directory", {
      newDir: newInstallDir,
    });
    if (errMsg !== null) {
      errorLog("Unable to set install directory");
      toastStore.makeToast(errMsg, "error");
    }
    return errMsg;
  } catch (e) {
    exceptionLog("Unable to set install directory", e);
    toastStore.makeToast("Invalid installation directory", "error");
    return "Unexpected error occurred";
  }
}

export async function isAVXRequirementMet(
  force: boolean
): Promise<boolean | undefined> {
  try {
    return await invoke("is_avx_requirement_met", { force: force });
  } catch (e) {
    exceptionLog("Unable to check if AVX requirement was met", e);
    return undefined;
  }
}

export async function isOpenGLRequirementMet(
  force: boolean
): Promise<boolean | undefined> {
  try {
    const result = await invoke("is_opengl_requirement_met", { force: force });
    if (typeof result === "boolean") {
      return result;
    }
    return undefined;
  } catch (e) {
    exceptionLog("Unable to check if OpenGL requirement was met", e);
    return undefined;
  }
}

export async function finalizeInstallation(gameName: string): Promise<void> {
  try {
    return await invoke("finalize_installation", { gameName: gameName });
  } catch (e) {
    exceptionLog("Unable to finalize installation", e);
  }
}

export async function isGameInstalled(gameName: string): Promise<boolean> {
  try {
    return await invoke("is_game_installed", { gameName: gameName });
  } catch (e) {
    exceptionLog("Unable to check if game was installed", e);
    return false;
  }
}

export async function getInstalledVersion(gameName: string): Promise<String> {
  try {
    return await invoke("get_installed_version", { gameName: gameName });
  } catch (e) {
    exceptionLog("Unable to check what version the game was installed with", e);
    return null;
  }
}

export async function getInstalledVersionFolder(
  gameName: string
): Promise<String> {
  try {
    return await invoke("get_installed_version_folder", { gameName: gameName });
  } catch (e) {
    exceptionLog(
      "Unable to check what version type the game was installed with",
      e
    );
    return null;
  }
}

export async function saveActiveVersionChange(
  folder: VersionFolders,
  newVersion: String
): Promise<boolean> {
  try {
    await invoke("save_active_version_change", {
      versionFolder: folder,
      newActiveVersion: newVersion,
    });
    return true;
  } catch (e) {
    exceptionLog("Unable to save version change", e);
    toastStore.makeToast("Couldn't save version change", "error");
    return false;
  }
}

export async function getLocale(): Promise<string | null> {
  try {
    return await invoke("get_locale", {});
  } catch (e) {
    exceptionLog("Unable to get locale", e);
    return "en-US";
  }
}

export async function setLocale(locale_string: string): Promise<void> {
  try {
    await invoke("set_locale", { locale: locale_string });
    locale.set(locale_string);
  } catch (e) {
    exceptionLog("Unable to set locale", e);
  }
}

export async function setBypassRequirements(bypass: boolean): Promise<void> {
  try {
    await invoke("set_bypass_requirements", { bypass: bypass });
  } catch (e) {
    exceptionLog("Unable to set bypress requirements", e);
  }
}

export async function getBypassRequirements(): Promise<boolean> {
  try {
    return await invoke("get_bypass_requirements", {});
  } catch (e) {
    exceptionLog("Unable to get bypress requirements setting", e);
    return false;
  }
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
