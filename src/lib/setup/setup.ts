import { Command } from "@tauri-apps/api/shell";
import { appDir, join } from "@tauri-apps/api/path";
import { os } from "@tauri-apps/api";
import { getHighestSimd } from "$lib/rpc/commands";
import { InstallStatus, isInstalling } from "../stores/AppStore";
import { SETUP_ERROR, SETUP_SUCCESS, SupportedGame } from "$lib/constants";
import { filePrompt } from "$lib/utils/file";
import {
  setGameInstallVersion,
  setInstallStatus,
  setRequirementsMet,
} from "../config";
import { resolveErrorCode } from "./setup_errors";
import { installLog, log } from "$lib/utils/log";

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
  throw new Error("UNSUPPORTED AVX");
}

/**
 * @param {String} version
 * @returns {Promise<Boolean>}
 */
export async function isOpenGLVersionSupported(
  version: string
): Promise<boolean> {
  if ((await os.platform()) === "darwin") {
    throw new Error("Unsupported OS!");
  }
  // Otherwise, query for the version
  let command = Command.sidecar("bin/glewinfo", ["-version", version]);
  const output = await command.execute();
  if (output.code === 0) {
    return true;
  }
  log.error("opengl requirement check failed", {
    statusCode: output.code,
    stdout: output.stdout,
    stderr: output.stderr,
  });
  throw new Error("UNSUPPORTED OPENGL VERSION");
}

export async function checkRequirements(): Promise<Boolean> {
  try {
    await isAVXSupported();
    await isOpenGLVersionSupported("4.3");
    await setRequirementsMet(true, true);
    return true;
  } catch (err) {
    return false;
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
  }
  if (output.stderr) {
    installLog.error(output.stderr, {
      game: SupportedGame.Jak1,
    });
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
  isInstalling.update(() => true);
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
  }
  if (output.stderr) {
    installLog.error(output.stderr, {
      game: SupportedGame.Jak1,
    });
  }
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
  isInstalling.update(() => true);
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
  }
  if (output.stderr) {
    installLog.error(output.stderr, {
      game: SupportedGame.Jak1,
    });
  }
  if (output.code === 0) {
    InstallStatus.update(() => SETUP_SUCCESS.ready);
    return true;
  }
  handleErrorCode(output.code, "Compiler");
}

export async function fullInstallation(game: SupportedGame): Promise<boolean> {
  let isoPath: string | string[];
  isInstalling.update(() => true);
  try {
    isoPath = await isoPrompt();
    await extractAndValidateISO(isoPath);
    await decompileGameData(isoPath);
    await compileGame(isoPath);
    await setInstallStatus(game, true);
    isInstalling.update(() => false);
    await setGameInstallVersion(game);
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

export async function recompileGame(game: SupportedGame) {
  // TODO - this assumes their files are in this folder, this is potentially wrong now!
  // ensure the extractor outputs to a sensible folder (per game)
  // and we are grabbing the right one
  const isoPath = await join(await appDir(), "data", "extracted_iso");
  // TODO - probably should check the dir exists
  isInstalling.update(() => true);
  try {
    // decompile & compile game
    await decompileGameData(isoPath);
    await compileGame(isoPath);
    // update settings.json with latest tools version from metadata.json
    await setGameInstallVersion(game);
    isInstalling.update(() => false);
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
  }
}
