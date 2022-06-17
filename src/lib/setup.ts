import { Command } from "@tauri-apps/api/shell";
import { resourceDir } from "@tauri-apps/api/path";
import { os } from "@tauri-apps/api";
import { getHighestSimd } from "$lib/commands";
import { InstallStatus } from "../stores/InstallStore";
import { SETUP_SUCCESS, SETUP_ERROR, SupportedGame } from "$lib/constants";
import { appendToInstallErrorLog, appendToInstallLog } from "$lib/utils/file";
import { setRequirementsMet } from "./config";

let debugPath: string;
let sidecarOptions = {};

export function isInDebugMode() {
  return process.env.NODE_ENV === "development";
}

if (isInDebugMode()) {
  // NOTE - this is kind of a total hack
  let path = await resourceDir();
  debugPath = path.split("launcher")[0].split("?\\")[1];
  debugPath += "launcher\\src-tauri\\data\\";
  sidecarOptions = { cwd: "bin" };
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
  let command = Command.sidecar(
    "bin/glewinfo",
    ["-version", version],
    sidecarOptions
  );
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

/**
 * @param {String} filePath
 * @returns {Promise<Boolean>}
 */
export async function extractAndValidateISO(
  filePath: string
): Promise<boolean> {
  let command: Command;

  InstallStatus.update(() => SETUP_SUCCESS.extractingISO);
  if (isInDebugMode()) {
    console.log(filePath);
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--extract", "--validate", "--proj-path", debugPath],
      sidecarOptions
    );
  } else {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--extract", "--validate"],
      sidecarOptions
    );
  }

  const output = await command.execute();
  if (output.stdout)
    console.log(output.stdout),
      await appendToInstallLog(SupportedGame.Jak1, output.stdout);
  if (output.stderr)
    console.log(output.stderr),
      await appendToInstallErrorLog(SupportedGame.Jak1, output.stdout);
  if (output.code === 0) {
    return true;
  }
  throw new Error(`Extractor exited with code: ${output.code}`);
}

/**
 * @param {String} filePath
 * @returns {Promise<Boolean>}
 */
export async function decompileGameData(filePath: string): Promise<boolean> {
  let command: Command;
  InstallStatus.update(() => SETUP_SUCCESS.decompiling);
  if (isInDebugMode()) {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--decompile", "--proj-path", debugPath],
      sidecarOptions
    );
  } else {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--decompile"],
      sidecarOptions
    );
  }

  const output = await command.execute();
  if (output.stdout)
    console.log(output.stdout),
      await appendToInstallLog(SupportedGame.Jak1, output.stdout);
  if (output.stderr)
    console.log(output.stderr),
      await appendToInstallErrorLog(SupportedGame.Jak1, output.stdout);
  if (output.code === 0) {
    return true;
  }
  throw new Error(`Decompiler exited with code: ${output.code}`);
}

/**
 * @param {String} filePath
 * @returns {Promise<Boolean>}
 */
export async function compileGame(filePath: string): Promise<Boolean> {
  let command: Command;
  InstallStatus.update(() => SETUP_SUCCESS.compiling);
  if (isInDebugMode()) {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--compile", "--proj-path", debugPath],
      sidecarOptions
    );
  } else {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--compile"],
      sidecarOptions
    );
  }

  const output = await command.execute();
  if (output.stdout)
    console.log(output.stdout),
      await appendToInstallLog(SupportedGame.Jak1, output.stdout);
  if (output.stderr)
    console.log(output.stderr),
      await appendToInstallErrorLog(SupportedGame.Jak1, output.stdout);
  if (output.code === 0) {
    InstallStatus.update(() => SETUP_SUCCESS.ready);
    return true;
  }
  throw new Error(`Compiler exited with code: ${output.code}`);
}
