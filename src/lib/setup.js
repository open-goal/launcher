import { Command } from '@tauri-apps/api/shell';
import { resourceDir } from '@tauri-apps/api/path';

// TODO - is this set to `production` properly in release mode?
function isInDebugMode() {
  return process.env.NODE_ENV === "development";
}

// TODO - this is kind of a total hack
let debugPath;
if (isInDebugMode()) {
  let path = await resourceDir();
  debugPath = path.split("launcher")[0].split("?\\")[1]
  debugPath += "\\launcher\\bundle-test\\data";
}

/**
 * @param {String} filePath
 * @returns {Promise<ChildProcess>}
 */
export async function extractISO(filePath) {
  let command;
  if (isInDebugMode()) {
    command = Command.sidecar('bin/extractor', [filePath, '-e', debugPath], { cwd: 'bin' })
  } else {
    command = Command.sidecar('bin/extractor', [filePath, '-e'], { cwd: 'bin' });
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
    command = Command.sidecar('bin/extractor', [filePath, '-v', debugPath], { cwd: 'bin' })
  } else {
    command = Command.sidecar('bin/extractor', [filePath, '-v'], { cwd: 'bin' });
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
    command = Command.sidecar('bin/extractor', [filePath, '-d', debugPath], { cwd: 'bin' })
  } else {
    command = Command.sidecar('bin/extractor', [filePath, '-d'], { cwd: 'bin' });
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
    command = Command.sidecar('bin/extractor', [filePath, '-c', debugPath], { cwd: 'bin' })
  } else {
    command = Command.sidecar('bin/extractor', [filePath, '-c'], { cwd: 'bin' });
  }

  return await command.execute();
}
