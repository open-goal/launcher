import { copyDirectory } from "$lib/commands";
import { SETUP_SUCCESS, SETUP_ERROR, SupportedGame } from "$lib/constants";
import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import {
  createDir,
  readDir,
  readTextFile,
  writeFile,
} from "@tauri-apps/api/fs";
import {
  appDir,
  dataDir,
  dirname,
  join,
  logDir,
  resourceDir,
} from "@tauri-apps/api/path";
import { InstallStatus, Console } from "../../stores/InstallStore";

export async function fileExists(path: string): Promise<boolean> {
  try {
    await readTextFile(path);
    return true;
  } catch (err) {
    return false;
  }
}

export async function dirExists(path: string): Promise<boolean> {
  try {
    await readDir(path);
    return true;
  } catch (err) {
    return false;
  }
}

export async function filePrompt(): Promise<string> {
  // TODO - pull strings out into args
  InstallStatus.update(() => SETUP_SUCCESS.awaitingISO);
  const path = await open({
    multiple: false,
    directory: false,
    filters: [{ extensions: ["ISO", "iso"], name: "Jak ISO File" }],
  });

  if (Array.isArray(path) || path === null) {
    InstallStatus.update(() => SETUP_ERROR.noISO);
    throw new Error("No ISO File Selected!");
  }

  return path;
}

// TODO - we need to copy over something to let us detect when the user updates and we need to copy again
// - this could be as simple as a json file with a version (launcher version)
export async function isDataDirectoryUpToDate(): Promise<boolean> {
  const appDirPath = await appDir();
  console.log(appDirPath);

  return dirExists(`${appDirPath}data`);
}

export async function copyDataDirectory(): Promise<boolean> {
  const resourceDirPath = await resourceDir();
  console.log(resourceDirPath);

  const appDirPath = await appDir();
  console.log(appDirPath);

  let src = `${resourceDirPath.replaceAll("\\\\?\\", "")}data`;
  let dst = `${appDirPath}data`;
  console.log(src);
  console.log(dst);

  try {
    await copyDirectory(src, dst);
    return true;
  } catch (e) {
    return false;
  }
}

export async function clearInstallLogs(supportedGame: SupportedGame) {
  Console.set(null);
  const dir = await logDir();
  let fileName = `${supportedGame}-install.log`;
  let fullPath = await join(dir, fileName);
  if (await fileExists(fullPath)) {
    await writeFile({ contents: "", path: fullPath });
  }
  fileName = `${supportedGame}-install-errors.log`;
  fullPath = await join(dir, fileName);
  if (await fileExists(fullPath)) {
    await writeFile({ contents: "", path: fullPath });
  }
}

export async function appendToInstallLog(
  supportedGame: SupportedGame,
  text: string
) {
  const dir = await logDir();
  const fileName = `${supportedGame}-install.log`;
  const fullPath = await join(dir, fileName);
  console.log(`[OG]: Writing logs to ${fullPath}`);
  let contents: string;
  if (!(await fileExists(fullPath))) {
    await createDir(await dirname(fullPath), { recursive: true });
  }
  contents = await readTextFile(fullPath);
  contents += text;
  Console.update(() => contents);
  await writeFile({ contents: contents, path: fullPath });
}

export async function appendToInstallErrorLog(
  supportedGame: SupportedGame,
  text: string
) {
  const dir = await logDir();
  const fileName = `${supportedGame}-install-errors.log`;
  const fullPath = await join(dir, fileName);
  console.log(`[OG]: Writing logs to ${fullPath}`);
  let contents: string;
  if (!(await fileExists(fullPath))) {
    await createDir(await dirname(fullPath), { recursive: true });
  }
  contents = await readTextFile(fullPath);
  contents += text;
  Console.update(() => contents);
  await writeFile({ contents: contents, path: fullPath });
}
