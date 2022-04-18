import { createDir, writeFile } from "@tauri-apps/api/fs";
import { appDir, join } from "@tauri-apps/api/path";
import { Store } from "tauri-plugin-store-api";
import { fileExists } from "./utils/file";

export class SupportedGame {
  static Jak1 = new SupportedGame("Jak 1");
  static Jak2 = new SupportedGame("Jak 2");
  static Jak3 = new SupportedGame("Jak 3");
  static JakX = new SupportedGame("Jak X");

  constructor(name) {
    this.name = name;
  }

  static fromName(name) {
    switch (name) {
      case this.Jak1.name:
        return this.Jak1;
      case this.Jak2.name:
        return this.Jak2;
      case this.Jak3.name:
        return this.Jak3;
      case this.JakX.name:
        return this.JakX;
      default:
        return null;
    }
  }

  static allGames = [this.Jak1, this.Jak2, this.Jak3, this.JakX];
}

class GameConfig {
  isInstalled = false;
  isActive = false;

  static createActive() {
    let val = new GameConfig();
    val.isActive = true;
    return val;
  }
}

class LauncherConfig {
  version = "1.0";
  games = {
    [SupportedGame.Jak1.name]: GameConfig.createActive(),
    [SupportedGame.Jak2.name]: new GameConfig(),
    [SupportedGame.Jak3.name]: new GameConfig(),
    [SupportedGame.JakX.name]: new GameConfig(),
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
 * @param {SupportedGame} supportedGame
 * @returns {Promise<boolean>}
 */
export async function getInstallStatus(supportedGame) {
  await store.load();
  if (!(await validVersion("1.0"))) {
    return false;
  }
  const gameConfigs = await store.get("games");
  if (gameConfigs == null || !(supportedGame.name in gameConfigs)) {
    return false;
  }
  return gameConfigs[supportedGame.name].isInstalled;
}

/**
 * The last game that was considered active in the launcher
 * @returns {Promise<SupportedGame | null>}
 */
export async function getLastActiveGame() {
  await store.load();
  if (!(await validVersion("1.0"))) {
    return null;
  }
  // Look through all games, find first
  for (const game of SupportedGame.allGames) {
    const gameConfig = await store.get(game.name);
    if (gameConfig.isActive) {
      return game;
    }
  }
}

/**
 * @param {SupportedGame} supportedGame
 * @param {boolean} installed
 * @returns
 */
export async function setInstallStatus(supportedGame, installed) {
  await store.load();
  if (!(await validVersion("1.0"))) {
    return;
  }
  let gameConfigs = await store.get("games");
  if (gameConfigs == null || !(supportedGame.name in gameConfigs)) {
    return;
  }
  gameConfigs[supportedGame.name].isInstalled = installed;
  await store.set("games", gameConfigs);
  await store.save();
}
