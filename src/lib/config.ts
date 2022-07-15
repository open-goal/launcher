import { createDir, readTextFile, writeFile } from "@tauri-apps/api/fs";
import { appDir, join } from "@tauri-apps/api/path";
import { Store } from "tauri-plugin-store-api";
import { SupportedGame } from "./constants";
import { fileExists } from "./utils/file";
import { log } from "./utils/log";

class GameConfig {
  isInstalled: boolean = false;
  version: string = null;
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
    log.info("settings file not found or could not be loaded!");
    await createDir(await appDir(), { recursive: true });
    await writeFile({
      contents: JSON.stringify(new LauncherConfig(), null, 2),
      path: path,
    });
    log.info("settings file initialized");
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
  // TODO: create a proper type for gameConfigs - exists with 'LauncherConfig'
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
  // TODO: create a proper type for gameConfigs - 'LauncherConfig'
  let gameConfigs: object = await store.get("games");
  // NOTE: Do we need this conditional? Considering we generate the store file this condition should never happen.
  if (gameConfigs == null || !(supportedGame in gameConfigs)) {
    return;
  }
  gameConfigs[supportedGame].isInstalled = installed;
  await store.set("games", gameConfigs);
  await store.save();
}

export async function setRequirementsMet(
  avx: boolean = null,
  openGL: boolean = null
) {
  await store.load();
  await store.set("requirements", { avx, openGL });
  await store.save();
  return;
}

/**
 * Checks the user config file to see if avx and openGL requirements are met.
 */
export async function areRequirementsMet(): Promise<boolean> {
  if ((await isAVXRequirementMet()) && (await isOpenGLRequirementMet())) {
    return true;
  }
  return false;
}

export async function isAVXRequirementMet(): Promise<boolean> {
  await store.load();
  let requirements = await store.get("requirements");
  if (!requirements["avx"]) {
    log.error("requirement false - AVX unsupported");
    return false;
  }
  return true;
}

export async function isOpenGLRequirementMet(): Promise<boolean> {
  await store.load();
  let requirements = await store.get("requirements");
  if (!requirements["openGL"]) {
    log.error("requirement false - OpenGL unsupported");
    return false;
  }
  return true;
}

export async function getGameInstallVersion(
  game: SupportedGame
): Promise<String> {
  // TODO - this can fail on first time startup from splash (where the config is init)
  // no idea why yet
  await store.load();
  let games: GameConfig = await store.get("games");
  const { version } = games[game];
  return version;
}

export async function setGameInstallVersion(game: SupportedGame) {
  const version = await getLatestToolsVersion();
  await store.load();
  let games: GameConfig = await store.get("games");
  games[game].version = version;
  await store.set("games", games);
  return await store.save();
}

export async function getLatestToolsVersion(): Promise<String> {
  const appDirPath = await appDir();
  const userMetaPath = await join(appDirPath, "data", "metadata.json");
  const data = await readTextFile(userMetaPath);
  const { version } = JSON.parse(data);
  return version;
}

export async function shouldUpdateGameInstall(
  game: SupportedGame
): Promise<Boolean> {
  const installVersion = await getGameInstallVersion(game);
  if (installVersion === null || installVersion === undefined) {
    return false;
  }
  const toolsVersion = await getLatestToolsVersion();

  if (installVersion === toolsVersion) {
    return false;
  }

  log.warn("Tools version is different than install verison", {
    tools: toolsVersion,
    installed: installVersion,
  });
  return true;
}
