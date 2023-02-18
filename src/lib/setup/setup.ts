import { Command } from "@tauri-apps/api/shell";
import { appDir, join } from "@tauri-apps/api/path";
import { os } from "@tauri-apps/api";
import { getHighestSimd } from "$lib/rpc/commands";
import { isInstalling } from "../stores/AppStore";
import { getInternalName, SupportedGame } from "$lib/constants";
import { resolveErrorCode } from "./setup_errors";
import { installLog, log } from "$lib/utils/log";
import { removeDir } from "@tauri-apps/api/fs";

export interface InstallationStatus {}

export async function isAVXSupported() {
  const highestSIMD = await getHighestSimd();
  if (highestSIMD === undefined) {
    return true;
  }
  if (highestSIMD.toLowerCase().startsWith("avx")) {
    return true;
  }
  return false;
}

/**
 * @param {String} version
 * @returns {Promise<Boolean>}
 */
export async function isOpenGLVersionSupported(
  version: string
): Promise<boolean> {
  if ((await os.platform()) === "darwin") {
    // TODO - log!
    return false;
  }
  // Otherwise, query for the version
  let command = Command.sidecar("bin/glewinfo", ["-version", version]);
  const output = await command.execute();
  if (output.code === 0) {
    return true;
  }
  log.error("opengl requirement check failed", {
    version: version,
    statusCode: output.code,
    stdout: output.stdout,
    stderr: output.stderr,
  });
  return false;
}

export async function checkRequirements(): Promise<void> {
  try {
    const isAVX = await isAVXSupported();
    const isOpenGL = await isOpenGLVersionSupported("4.3");
    console.log(`avx - ${isAVX} opengl - ${isOpenGL}`);
    // TODO - fix
    // await launcherConfig.setRequirementsMet(isAVX, isOpenGL);
  } catch (err) {
    // await launcherConfig.setRequirementsMet(false, false);
  }
}

async function handleErrorCode(code: number, stepName: string) {
  isInstalling.update(() => false);
  const explaination = await resolveErrorCode(code);
  if (explaination === undefined) {
    throw new Error(`${stepName} exited with unexpected code: ${code}`);
  }
  throw new Error(explaination);
}
