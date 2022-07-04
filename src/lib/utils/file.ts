import { copyDirectory } from "$lib/rpc/commands";
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
import { InstallStatus, ProcessLogs } from "../stores/AppStore";

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
    // NOTE - this isn't case sensitive!
    await readDir(path);
    return true;
  } catch (err) {
    return false;
  }
}

export async function filePrompt(): Promise<string> {
  // TODO - shouldn't be ISO specific in this function
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

// TODO - move this stuff into a separate file, this isn't generic file util stuff anymore

export async function dataDirectoryExists(): Promise<boolean> {
  return await dirExists(await join(await appDir(), "data"));
}

export async function isDataDirectoryUpToDate(): Promise<boolean> {
  const resourceDirPath = await resourceDir();
  const appDirPath = await appDir();
  // There should be a `metadata.json` which will help us know if the directory is out of date
  // aka, does the app have updated files compared to what the user has in their appDir.
  const userMetaPath = await join(appDirPath, "data", "metadata.json");
  const appMetaPath = await join(resourceDirPath, "data", "metadata.json");
  if (!(await fileExists(userMetaPath))) {
    console.log(
      `[Launcher]: Couldn't locate user's metadata file at '${userMetaPath}'`
    );
    return false;
  }
  // If it's there, read it in and check the version, compare with the app's
  const userMetaVersion = JSON.parse(await readTextFile(userMetaPath)).version;
  const appMetaVersion = JSON.parse(await readTextFile(appMetaPath)).version;
  if (userMetaVersion != appMetaVersion) {
    console.log(
      `[Launcher]: User version ${userMetaVersion} does not match app version ${appMetaVersion}`
    );
    return false;
  }
  // NOTE - the user can of course mess up their directory more, but we can only hold their hands so much
  // TODO - better to add some sort of "verify local data" feature in the app imo
  return true;
}

export async function copyDataDirectory(): Promise<boolean> {
  const resourceDirPath = await resourceDir();
  const appDirPath = await appDir();

  let src = `${resourceDirPath.replaceAll("\\\\?\\", "")}data`;
  let dst = `${appDirPath}data`;

  try {
    await copyDirectory(src, dst);
    return true;
  } catch (e) {
    return false;
  }
}

// TODO - move this to a logging file and replace all `console.logs` in the entire project

export async function clearInstallLogs(supportedGame: SupportedGame) {
  ProcessLogs.set(null);
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
  } else {
    contents = await readTextFile(fullPath);
  }
  contents += text;
  ProcessLogs.update(() => contents);
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
  } else {
    contents = await readTextFile(fullPath);
  }
  contents += text;
  ProcessLogs.update(() => contents);
  await writeFile({ contents: contents, path: fullPath });
}
