import { Command } from "@tauri-apps/api/shell";
import { resourceDir } from "@tauri-apps/api/path";
import { os } from "@tauri-apps/api";
import { getHighestSimd } from "$lib/commands";

let debugPath;
let sidecarOptions = {};

export function isInDebugMode() {
  return process.env.NODE_ENV === "development";
}

if (isInDebugMode()) {
  // TODO - this is kind of a total hack
  let path = await resourceDir();
  debugPath = path.split("launcher")[0].split("?\\")[1];
  // debugPath += "\\launcher\\bundle-test\\data";\
  debugPath += "launcher\\src-tauri\\data\\"
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
  throw new Error('UNSUPPORTED AVX');
}

/**
 * @param {String} version
 * @returns {Promise<Boolean>}
 */
export async function isOpenGLVersionSupported(version) {
  if ((await os.platform()) === "darwin") {
    throw new Error("Unsupported OS!");
    // return RequirementStatus.Unknown;
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
  throw new Error('UNSUPPORTED OPENGL VERSION');
}

/**
 * @param {String} filePath
 * @returns {Promise<Boolean>}
 */
export async function extractAndValidateISO(filePath) {
  let command;
  if (isInDebugMode()) {
    console.log(debugPath);
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

  const output = await command.execute();
  if (output.code === 0) {
    return true;
  }
  throw new Error(`Extractor exited with code: ${output.code}`);
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
  throw new Error(`Decompiler exited with code: ${output.code}`);
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
  throw new Error(`Compiler exited with code: ${output.code}`);
}
