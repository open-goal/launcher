import { invoke } from "@tauri-apps/api/tauri";

export async function getInstallationDirectory(): Promise<string | null> {
  try {
    return await invoke("get_install_directory", {});
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function setInstallationDirectory(
  newInstallDir: string
): Promise<void> {
  try {
    await invoke("set_install_directory", { newDir: newInstallDir });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function isAVXRequirementMet(): Promise<boolean> {
  try {
    return await invoke("is_avx_requirement_met", {});
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function isOpenGLRequirementMet(): Promise<boolean> {
  try {
    return await invoke("is_opengl_requirement_met", {});
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function finalizeInstallation(gameName: string): Promise<void> {
  try {
    return await invoke("finalize_installation", { gameName: gameName });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function isGameInstalled(gameName: string): Promise<boolean> {
  try {
    return await invoke("is_game_installed", { gameName: gameName });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function getInstalledVersion(gameName: string): Promise<String> {
  try {
    return await invoke("get_installed_version", { gameName: gameName });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function getInstalledVersionFolder(
  gameName: string
): Promise<String> {
  try {
    return await invoke("get_installed_version_folder", { gameName: gameName });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function getActiveVersion(gameName: string): Promise<String> {
  try {
    return await invoke("get_active_version", { gameName: gameName });
  } catch (e) {
    console.log("TODO AH!");
  }
}

export async function getActiveVersionFolder(
  gameName: string
): Promise<String> {
  try {
    return await invoke("get_active_version_folder", { gameName: gameName });
  } catch (e) {
    console.log("TODO AH!");
  }
}
