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

async function isoPrompt(): Promise<string> {
  InstallStatus.update(() => SETUP_SUCCESS.awaitingISO);
  const path = await filePrompt(["ISO", "iso"], "Jak ISO File");
  if (path === null) {
    InstallStatus.update(() => SETUP_ERROR.noISO);
    throw new Error("No ISO File Selected!");
  }

  return path;
}

/**
 * @param {String} filePath
 * @returns {Promise<Boolean>}
 */
export async function extractAndValidateISO(
  filePath: string
): Promise<boolean> {
  let command: Command;

  InstallStatus.update(() => SETUP_SUCCESS.extractingISO);
  const appDirPath = await appDir();
  command = Command.sidecar("bin/extractor", [
    filePath,
    "--extract",
    "--validate",
    "--proj-path",
    `${appDirPath}data`,
  ]);

  const output = await command.execute();
  if (output.stdout) {
    installLog.info(output.stdout, {
      game: SupportedGame.Jak1,
    });
    ProcessLogs.update((currLogs) => currLogs + output.stdout);
  }
  if (output.stderr) {
    installLog.error(output.stderr, {
      game: SupportedGame.Jak1,
    });
    ProcessLogs.update((currLogs) => currLogs + output.stderr);
  }
  if (output.code === 0) {
    return true;
  }

  handleErrorCode(output.code, "Extraction");
}

/**
 * @param {String} filePath
 * @returns {Promise<Boolean>}
 */
export async function decompileGameData(filePath: string): Promise<boolean> {
  let command: Command;
  InstallStatus.update(() => SETUP_SUCCESS.decompiling);
  isDecompiling.update(() => true);
  const appDirPath = await appDir();
  command = Command.sidecar("bin/extractor", [
    filePath,
    "--decompile",
    "--proj-path",
    `${appDirPath}data`,
  ]);

  const output = await command.execute();
  if (output.stdout) {
    installLog.info(output.stdout, {
      game: SupportedGame.Jak1,
    });
    ProcessLogs.update((currLogs) => currLogs + output.stdout);
  }
  if (output.stderr) {
    installLog.error(output.stderr, {
      game: SupportedGame.Jak1,
    });
    ProcessLogs.update((currLogs) => currLogs + output.stderr);
  }
  isDecompiling.update(() => false);
  if (output.code === 0) {
    return true;
  }
  handleErrorCode(output.code, "Decompiler");
}

/**
 * @param {String} filePath
 * @returns {Promise<Boolean>}
 */
export async function compileGame(filePath: string): Promise<Boolean> {
  let command: Command;
  InstallStatus.update(() => SETUP_SUCCESS.compiling);
  isCompiling.update(() => true);
  const appDirPath = await appDir();
  command = Command.sidecar(
    "bin/extractor",
    [filePath, "--compile", "--proj-path", `${appDirPath}data`],
    sidecarOptions
  );

  const output = await command.execute();
  if (output.stdout) {
    installLog.info(output.stdout, {
      game: SupportedGame.Jak1,
    });
    ProcessLogs.update((currLogs) => currLogs + output.stdout);
  }
  if (output.stderr) {
    installLog.error(output.stderr, {
      game: SupportedGame.Jak1,
    });
    ProcessLogs.update((currLogs) => currLogs + output.stderr);
  }
  isCompiling.update(() => false);
  if (output.code === 0) {
    return true;
  }
  handleErrorCode(output.code, "Compiler");
}

export async function fullInstallation(game: SupportedGame): Promise<boolean> {
  let isoPath: string | string[];
  isInstalling.update(() => true);
  try {
    isoPath = await isoPrompt();
    ProcessLogs.update(() => "");
    await extractAndValidateISO(isoPath);
    await decompileGameData(isoPath);
    await compileGame(isoPath);
    await launcherConfig.setInstallStatus(game, true);
    isInstalling.update(() => false);
    InstallStatus.update(() => SETUP_SUCCESS.ready);
    await launcherConfig.setGameInstallVersion(game);
    gameNeedsReinstall.update(() => false);
    return true;
  } catch (err) {
    installLog.error("unexpected error encountered", {
      error: err,
    });
    let errStatus = {
      status: err,
      percent: undefined,
    };
    InstallStatus.update(() => errStatus);
    isInstalling.update(() => false);
    return false;
  }
}

export async function recompileGame(game: SupportedGame): Promise<boolean> {
  const isoPath = await join(
    await appDir(),
    "data",
    "iso_data",
    getInternalName(game)
  );
  // TODO - probably should check the dir exists
  isInstalling.update(() => true);
  try {
    // decompile & compile game
    await decompileGameData(isoPath);
    await compileGame(isoPath);
    // update settings.json with latest tools version from metadata.json
    await launcherConfig.setGameInstallVersion(game);
    await launcherConfig.setInstallStatus(game, true);
    gameNeedsReinstall.update(() => false);
    isInstalling.update(() => false);
    return true;
  } catch (err) {
    installLog.error("unexpected error encountered", {
      error: err,
    });
    let errStatus = {
      status: err,
      percent: undefined,
    };
    InstallStatus.update(() => errStatus);
    isInstalling.update(() => false);
    return false;
  }
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
