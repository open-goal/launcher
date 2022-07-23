import { copyDirectory } from "$lib/rpc/commands";
import { readTextFile } from "@tauri-apps/api/fs";
import { join, appDir, resourceDir } from "@tauri-apps/api/path";
import { dirExists, fileExists } from "./file";
import { log } from "./log";

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
    log.warn("couldn't locate user's metadata file at path", {
      path: userMetaPath,
    });
    return false;
  }
  // If it's there, read it in and check the version, compare with the app's
  const userMetaVersion = JSON.parse(await readTextFile(userMetaPath)).version;
  const appMetaVersion = JSON.parse(await readTextFile(appMetaPath)).version;
  if (userMetaVersion != appMetaVersion) {
    log.warn("user version does not match app version", {
      userVersion: userMetaVersion,
      appVersion: appMetaVersion,
    });
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
