import { invoke } from "@tauri-apps/api/tauri";
import { exceptionLog } from "./logging";

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
    return await invoke("set_install_directory", { newDir: newInstallDir });
  } catch (e) {
    exceptionLog("Unable to set install directory", e);
    return "Unexpected error occurred";
  }
}

export async function isAVXRequirementMet(): Promise<boolean> {
  try {
    return await invoke("is_avx_requirement_met", {});
  } catch (e) {
    exceptionLog("Unable to check if AVX requirement was met", e);
    // TODO - third condition for if we couldn't check
  }
}

export async function isOpenGLRequirementMet(): Promise<boolean> {
  try {
    return await invoke("is_opengl_requirement_met", {});
  } catch (e) {
    exceptionLog("Unable to check if OpenGL requirement was met", e);
    // TODO - third condition for if we couldn't check
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
