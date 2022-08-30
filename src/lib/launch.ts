import { Command } from "@tauri-apps/api/shell";
import { appDir, resourceDir } from "@tauri-apps/api/path";

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
  command.spawn();
}

export async function openREPL() {
  const appDirPath = await appDir();
  let command: Command = Command.sidecar("bin/goalc", [
    "-proj-path",
    `${appDirPath}data`,
  ]);
  command.spawn();
}
