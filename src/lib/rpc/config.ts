import { toastStore } from "$lib/stores/ToastStore";
import { invoke } from "@tauri-apps/api/tauri";
import { errorLog, exceptionLog } from "./logging";
import type { VersionFolders } from "./versions";

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

export async function isAVXRequirementMet(): Promise<boolean | undefined> {
  try {
    return await invoke("is_avx_requirement_met", {});
  } catch (e) {
    exceptionLog("Unable to check if AVX requirement was met", e);
    return undefined;
  }
}

export async function isOpenGLRequirementMet(): Promise<boolean | undefined> {
  try {
    const result = await invoke("is_opengl_requirement_met", {});
    if (typeof result === "boolean") {
      return result;
    }
    return undefined;
  } catch (e) {
    exceptionLog("Unable to check if OpenGL requirement was met", e);
    return undefined;
  }
}

export async function setOpenGLRequirementMet(val: boolean): Promise<void> {
  try {
    return await invoke("set_opengl_requirement_met", { requirementMet: val });
  } catch (e) {
    exceptionLog("Unable to set OpenGL requirement", e);
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

export async function getBackgroundVideoDisabled(): Promise<boolean> {
  try {
    return await invoke("get_background_disabled");
  } catch (e) {
    exceptionLog("Unable to check if background is disabled with", e);
    return false;
  }
}

export async function setBackgroundVideoDisabled(val: boolean): Promise<void> {
  try {
    return await invoke("set_background_disabled", { disabled: val });
  } catch (e) {
    exceptionLog("Unable to set background disabled", e);
  }
}
