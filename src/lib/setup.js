import { Command } from "@tauri-apps/api/shell";
import { resourceDir } from "@tauri-apps/api/path";

export class InstallationStatus {
  static Pending = Symbol("pending");
  static InProgress = Symbol("inprogress");
  static Failed = Symbol("failed");
  static Success = Symbol("success");
}

// TODO - is this set to `production` properly in release mode?
export function isInDebugMode() {
  return process.env.NODE_ENV === "development";
}

// TODO - this is kind of a total hack
let debugPath;
if (isInDebugMode()) {
  let path = await resourceDir();
  debugPath = path.split("launcher")[0].split("?\\")[1];
  debugPath += "\\launcher\\bundle-test\\data";
}

/**
 * @param {String} filePath
 * @returns {Promise<ChildProcess>}
 */
export async function extractISO(filePath) {
  let command;
  if (isInDebugMode()) {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--extract", "--proj-path", debugPath],
      { cwd: "bin" }
    );
  } else {
    command = Command.sidecar("bin/extractor", [filePath, "--extract"], {
      cwd: "bin",
    });
  }

  return await command.execute();
}

/**
 * @param {String} filePath
 * @returns {Promise<ChildProcess>}
 */
export async function validateGameData(filePath) {
  let command;
  if (isInDebugMode()) {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--validate", "--proj-path", debugPath],
      { cwd: "bin" }
    );
  } else {
    command = Command.sidecar("bin/extractor", [filePath, "--validate"], {
      cwd: "bin",
    });
  }

  return await command.execute();
}

/**
 * @param {String} filePath
 * @returns {Promise<ChildProcess>}
 */
export async function decompileGameData(filePath) {
  let command;
  if (isInDebugMode()) {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--decompile", "--proj-path", debugPath],
      { cwd: "bin" }
    );
  } else {
    command = Command.sidecar("bin/extractor", [filePath, "--decompile"], {
      cwd: "bin",
    });
  }

  return await command.execute();
}

/**
 * @param {String} filePath
 * @returns {Promise<ChildProcess>}
 */
export async function compileGame(filePath) {
  let command;
  if (isInDebugMode()) {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--compile", "--proj-path", debugPath],
      { cwd: "bin" }
    );
  } else {
    command = Command.sidecar("bin/extractor", [filePath, "--compile"], {
      cwd: "bin",
    });
  }

  return await command.execute();
}
