import { Command } from "@tauri-apps/api/shell";
import { appDir, join, logDir, resourceDir } from "@tauri-apps/api/path";
import { writeFile } from "@tauri-apps/api/fs";

function isInDebugMode() {
  return process.env.NODE_ENV === "development";
}

// NOTE - this is kind of a total hack - likely windows only currently
let debugPath: string;
if (isInDebugMode()) {
  let path = await resourceDir();
  debugPath = path.split("launcher")[0].split("?\\")[1];
  debugPath += "launcher\\src-tauri\\data";
}

export async function launchGame() {
  let command: Command;
  const appDirPath = await appDir();
  command = Command.sidecar("bin/gk", [
    "-boot",
    "-fakeiso",
    "-proj-path",
    `${appDirPath}data`,
  ]);
  // TODO - likely better to move this into Rust so we can pipe the logs instead
  // of waiting for the process to exit
  //
  // TODO - this should take the game-name (the `gk` command needs to eventually anyway!)
  //
  // NOTE - Doing it in rust may also continue to tee logs when the launcher process is terminated
  // but I'm not 100% sure on this
  const gameProcess = await command.execute();
  const dir = await logDir();
  const logFileStdout = await join(dir, "game-stdout.log");
  await writeFile({ contents: gameProcess.stdout, path: logFileStdout });
  const logFileStderr = await join(dir, "game-stderr.log");
  await writeFile({ contents: gameProcess.stderr, path: logFileStderr });
}

export async function launchGameInDebug() {
  let command: Command;
  const appDirPath = await appDir();
  command = Command.sidecar("bin/gk", [
    "-boot",
    "-fakeiso",
    "-debug",
    "-proj-path",
    `${appDirPath}data`,
  ]);
  const gameProcess = await command.execute();
  const dir = await logDir();
  const logFileStdout = await join(dir, "game-stdout.log");
  await writeFile({ contents: gameProcess.stdout, path: logFileStdout });
  const logFileStderr = await join(dir, "game-stderr.log");
  await writeFile({ contents: gameProcess.stderr, path: logFileStderr });
}
