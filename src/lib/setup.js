import { Command } from "@tauri-apps/api/shell";
import { resourceDir } from "@tauri-apps/api/path";
import { os } from "@tauri-apps/api";
import { getHighestSimd } from "$lib/commands";

export class InstallationStatus {
  static Pending = Symbol("pending");
  static InProgress = Symbol("inprogress");
  static Failed = Symbol("failed");
  static Success = Symbol("success");
}

export class RequirementStatus {
  static Unknown = Symbol("unknown");
  static Met = Symbol("met");
  static Failed = Symbol("failed");
  static Checking = Symbol("checking");
}

export function isInDebugMode() {
  return process.env.NODE_ENV === "development";
}

let debugPath;
let sidecarOptions = {};
if (isInDebugMode()) {
  // TODO - this is kind of a total hack
  let path = await resourceDir();
  debugPath = path.split("launcher")[0].split("?\\")[1];
  debugPath += "\\launcher\\bundle-test\\data";
  sidecarOptions = { cwd: "bin" };
}

export async function isAVXSupported() {
  let highestSIMD = await getHighestSimd();
  if (highestSIMD === undefined) {
    return RequirementStatus.Unknown;
  }
  if (highestSIMD.toLowerCase().startsWith("avx")) {
    return true;
  }
  return false;
}

export async function isOpenGLVersionSupported(version) {
  if ((await os.platform()) === "darwin") {
    return RequirementStatus.Unknown;
  }
  // Otherwise, query for the version
  let command = Command.sidecar(
    "bin/glewinfo",
    ["-version", version],
    sidecarOptions
  );
  try {
    const output = await command.execute();
    if (output.code === 0) {
      return true;
    }
    return false;
  } catch (e) {
    throw new Error('ERROR MESSAGE');
  }
}

/**
 * @param {String} filePath
 * @returns {Promise<Boolean>}
 */
export async function extractAndValidateISO(filePath) {
  let command;
  if (isInDebugMode()) {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--extract", "--proj-path", debugPath],
      sidecarOptions
    );
  } else {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--extract", "--validate"],
      sidecarOptions
    );
  }

  console.log(command);
  const output = await command.execute();
  if (output.code === 0) {
    return true;
  }
  throw new Error("ERROR MESSAGE");
}

/**
 * @param {String} filePath
 * @returns {Promise<Boolean>}
 */
export async function decompileGameData(filePath) {
  let command;
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
  if (output.code === 0) {
    return true;
  }
  throw new Error("ERROR MESSAGE");
}

/**
 * @param {String} filePath
 * @returns {Promise<Boolean>}
 */
export async function compileGame(filePath) {
  let command;
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
  if (output.code === 0) {
    return true;
  }
  throw new Error("ERROR MESSAGE");
}
