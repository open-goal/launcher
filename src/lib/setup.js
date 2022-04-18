import { Command } from "@tauri-apps/api/shell";
import { resourceDir } from "@tauri-apps/api/path";
import { os } from "@tauri-apps/api";
import { getHighestSimd } from "/src/lib/commands";

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

export async function isAVXSupported() {
  let highestSIMD = await getHighestSimd();
  if (highestSIMD === undefined) {
    return RequirementStatus.Unknown;
  }
  if (highestSIMD.toLowerCase().startsWith("avx")) {
    return RequirementStatus.Met;
  }
  return RequirementStatus.Failed;
}

export async function isOpenGLVersionSupported(version) {
  // TODO - glewinfo not pre-compiled to work on linux yet!
  if ((await os.platform()) === "darwin") {
    return RequirementStatus.Unknown;
  }
  // Otherwise, query for the version
  let command = Command.sidecar("bin/glewinfo", ["-version", version], {
    cwd: "bin",
  });
  try {
    let output = await command.execute();
    if (output.code === 0) {
      return RequirementStatus.Met;
    }
    return RequirementStatus.Failed;
  } catch {
    return RequirementStatus.Failed;
  }
}

/**
 * @param {String} filePath
 * @returns {Promise<ChildProcess>}
 */
export async function extractAndValidateISO(filePath) {
  let command;
  if (isInDebugMode()) {
    command = Command.sidecar(
      "bin/extractor",
      [filePath, "--extract", "--proj-path", debugPath],
      { cwd: "bin" }
    );
  } else {
    command = Command.sidecar("bin/extractor", [filePath, "--extract", "--validate"], {
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
