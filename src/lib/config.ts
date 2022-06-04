import { createDir, writeFile } from "@tauri-apps/api/fs";
import { appDir, join } from "@tauri-apps/api/path";
import { Store } from "tauri-plugin-store-api";
import { SUPPORTED_GAME } from "./constants";
import { fileExists } from "./utils/file";

class GameConfig {
  isInstalled = false;
  isActive = false;

  static createActive() {
    let val = new GameConfig();
    val.isActive = true;
    return val;
  }
}

// TODO: LINK REQUIREMENTS TO CHECK REQUIREMENTS FUNCTION TO AVOID RUNNING FUNCTION IF REQUIREMENTS ARE MET
class LauncherConfig {
  version = "1.0";
  requirements = {
    avx: null,
    openGL: null,
  };
  games = {
    [SUPPORTED_GAME.Jak1]: GameConfig.createActive(),
    [SUPPORTED_GAME.Jak2]: new GameConfig(),
    [SUPPORTED_GAME.Jak3]: new GameConfig(),
    [SUPPORTED_GAME.JakX]: new GameConfig(),
  };
}

const store = new Store("settings.json");

/**
 * Checks the version to enable safe config operations
 * @param {*} version "<major>.<minor>"
 * @returns True if majors match, and expected minor greater than or equal to stored.  False otherwise, or if no version can be found
 */
async function validVersion(version) {
  let [major, minor] = version.split(".");
  await store.load();
  if (!(await store.has("version"))) {
    return false;
  }
  let [storedMajor, storedMinor] = (await store.get("version")).split(".");
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
export async function getInstallStatus(supportedGame) {
  await store.load();
  if (!(await validVersion("1.0"))) {
    return false;
  }
  const gameConfigs = await store.get("games");
  if (gameConfigs == null || !(supportedGame in gameConfigs)) {
    return false;
  }
  return gameConfigs[supportedGame].isInstalled;
}

/**
 * The last game that was considered active in the launcher
 * @returns {Promise<String | null>}
 */
export async function getLastActiveGame() {
  await store.load();
  if (!(await validVersion("1.0"))) {
    return null;
  }
  // Look through all games, find first active game
  for (const game in SUPPORTED_GAME) {
    const gameConfig = await store.get(game);
    if (gameConfig.isActive) {
      return game;
    }
  }
}

/**
 * @param {string} supportedGame
 * @param {boolean} installed
 * @returns
 */
export async function setInstallStatus(supportedGame, installed) {
  await store.load();
  if (!(await validVersion("1.0"))) {
    return;
  }
  let gameConfigs = await store.get("games");
  if (gameConfigs == null || !(supportedGame in gameConfigs)) {
    return;
  }
  gameConfigs[supportedGame].isInstalled = installed;
  await store.set("games", gameConfigs);
  await store.save();
}
