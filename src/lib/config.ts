import { createDir, writeFile } from "@tauri-apps/api/fs";
import { appDir, join } from "@tauri-apps/api/path";
import { Store } from "tauri-plugin-store-api";
import { SupportedGame } from "./constants";
import { fileExists } from "./utils/file";

class GameConfig {
  isInstalled: boolean = false;
}

// TODO: LINK REQUIREMENTS TO CHECK REQUIREMENTS FUNCTION TO AVOID RUNNING FUNCTION IF REQUIREMENTS ARE MET
class LauncherConfig {
  version = "1.0";
  requirements: { avx: boolean | null; openGL: boolean | null } = {
    avx: null,
    openGL: null,
  };
  games = {
    [SupportedGame.Jak1]: new GameConfig(),
    [SupportedGame.Jak2]: new GameConfig(),
    [SupportedGame.Jak3]: new GameConfig(),
    [SupportedGame.JakX]: new GameConfig(),
  };
  lastActiveGame: SupportedGame;
}

const store = new Store("settings.json");

/**
 * Checks the version to enable safe config operations
 * @param {*} version "<major>.<minor>"
 * @returns True if majors match, and expected minor greater than or equal to stored.  False otherwise, or if no version can be found
 */
async function validVersion(version: string): Promise<boolean> {
  let [major, minor] = version.split(".");
  await store.load();
  if (!(await store.has("version"))) {
    return false;
  }
  let [storedMajor, storedMinor]: string[] = (await store.get("version")).split(
    "."
  );
  if (major != storedMajor) {
    return false;
  }
  if (parseInt(minor) < parseInt(storedMinor)) {
    return false;
  }
  return true;
}

export async function initConfig() {
  const path = await join(await appDir(), "settings.json");
  let configExists = await fileExists(path);
  if (!configExists) {
    console.log("[Launcher]: Settings file not found or could not be loaded!");
    await createDir(await appDir(), { recursive: true });
    await writeFile({
      contents: JSON.stringify(new LauncherConfig(), null, 2),
      path: path,
    });
    console.log("[Launcher]: Settings file initialized");
  }
}

/**
 * If a game is installed or not
 * @param {string} supportedGame
 * @returns {Promise<boolean>}
 */
export async function getInstallStatus(
  supportedGame: SupportedGame
): Promise<boolean> {
  await store.load();
  if (!(await validVersion("1.0"))) {
    return false;
  }
  // TODO: create a proper type for gameConfigs
  const gameConfigs: object = await store.get("games");
  if (gameConfigs == null || !(supportedGame in gameConfigs)) {
    return false;
  }
  return gameConfigs[supportedGame].isInstalled;
}

/**
 * The last game that was considered active in the launcher
 * @returns {Promise<SupportedGame | null>}
 */
export async function getLastActiveGame(): Promise<SupportedGame> {
  await store.load();
  if (!(await validVersion("1.0"))) {
    return null;
  }

  const lastActiveGame: SupportedGame = await store.get("lastActiveGame");
  return lastActiveGame;
}

/**
 * @param {string} supportedGame
 * @param {boolean} installed
 * @returns
 */
export async function setInstallStatus(
  supportedGame: SupportedGame,
  installed: boolean
): Promise<void> {
  await store.load();
  if (!(await validVersion("1.0"))) {
    return;
  }
  // TODO: create a proper type for gameConfigs
  let gameConfigs: object = await store.get("games");
  // NOTE: Do we need this conditional? Considering we generate the store file this condition should never happen.
  if (gameConfigs == null || !(supportedGame in gameConfigs)) {
    return;
  }
  gameConfigs[supportedGame].isInstalled = installed;
  await store.set("games", gameConfigs);
  await store.save();
}
