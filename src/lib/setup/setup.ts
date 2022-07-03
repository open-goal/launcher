import { Command } from "@tauri-apps/api/shell";
import { appDir } from "@tauri-apps/api/path";
import { os } from "@tauri-apps/api";
import { getHighestSimd } from "$lib/commands";
import { InstallStatus, isInstalling } from "../../stores/InstallStore";
import { SETUP_SUCCESS, SETUP_ERROR, SupportedGame } from "$lib/constants";
import { appendToInstallErrorLog, appendToInstallLog } from "$lib/utils/file";
import { setRequirementsMet } from "../config";
import { BaseDirectory, copyFile } from "@tauri-apps/api/fs";
import { resolveErrorCode } from "./setup_errors";

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

export async function saveISO(filePath: string): Promise<any> {
  const appDirPath = await appDir();
  await copyFile(filePath, `${appDirPath}/jak.iso`, { dir: BaseDirectory.App });
  return;
}

async function handleErrorCode(code: number, stepName: string) {
  isInstalling.update(() => false);
  const explaination = await resolveErrorCode(code);
  if (explaination === undefined) {
    throw new Error(`${stepName} exited with unexpected code: ${code}`);
  }
  throw new Error(explaination);
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
    await appendToInstallLog(SupportedGame.Jak1, output.stdout);
  }
  if (output.stderr) {
    await appendToInstallErrorLog(SupportedGame.Jak1, output.stdout);
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
    await appendToInstallLog(SupportedGame.Jak1, output.stdout);
  }
  if (output.stderr) {
    await appendToInstallErrorLog(SupportedGame.Jak1, output.stdout);
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
    await appendToInstallLog(SupportedGame.Jak1, output.stdout);
  }
  if (output.stderr) {
    await appendToInstallErrorLog(SupportedGame.Jak1, output.stdout);
  }
  if (output.code === 0) {
    InstallStatus.update(() => SETUP_SUCCESS.ready);
    return true;
  }
  handleErrorCode(output.code, "Compiler");
}
