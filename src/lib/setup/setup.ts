import { Command } from "@tauri-apps/api/shell";
import { appDir, join } from "@tauri-apps/api/path";
import { os } from "@tauri-apps/api";
import { getHighestSimd } from "$lib/rpc/commands";
import {
  gameNeedsReinstall,
  InstallStatus,
  isCompiling,
  isDecompiling,
  isInstalling,
} from "../stores/AppStore";
import {
  getInternalName,
  SETUP_ERROR,
  SETUP_SUCCESS,
  SupportedGame,
} from "$lib/constants";
import { filePrompt } from "$lib/utils/file";
import { launcherConfig } from "$lib/config";
import { resolveErrorCode } from "./setup_errors";
import { installLog, log } from "$lib/utils/log";
import { ProcessLogs } from "$lib/stores/AppStore";
import { removeDir } from "@tauri-apps/api/fs";

let sidecarOptions = {};

export interface InstallationStatus {}

export function isInDebugMode() {
  return process.env.NODE_ENV === "development";
}

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
    await launcherConfig.setRequirementsMet(isAVX, isOpenGL);
  } catch (err) {
    await launcherConfig.setRequirementsMet(false, false);
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

export async function decompileFromFile(activeGame: SupportedGame) {
  const isoPath = await join(
    await appDir(),
    "data",
    "iso_data",
    getInternalName(activeGame)
  );
  await decompileGameData(isoPath);
}

export async function compileFromFile(activeGame: SupportedGame) {
  const isoPath = await join(
    await appDir(),
    "data",
    "iso_data",
    getInternalName(activeGame)
  );
  await compileGame(isoPath);
}

export async function uninstallGame(game: SupportedGame) {
  const dataDir = await join(await appDir(), "data");
  try {
    const t0 = await join(dataDir, "decompiler_out", getInternalName(game));
    const t1 = await join(dataDir, "iso_data", getInternalName(game));
    const t2 = await join(dataDir, "out", getInternalName(game));
    const targets = [t0, t1, t2];
    for (const target of targets) {
      console.log("Deleting folder: ", target);
      await removeDir(target, { recursive: true });
    }
  } catch (error) {
    console.error(error);
  }
}
