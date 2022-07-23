import { createDir, readTextFile, writeFile } from "@tauri-apps/api/fs";
import { appDir, join } from "@tauri-apps/api/path";
import { SupportedGame } from "./constants";
import { fileExists } from "./utils/file";
import { log } from "./utils/log";

interface Serializable<T> {
  deserialize(input: Object): T;
}

class GameConfig implements Serializable<GameConfig> {
  isInstalled: boolean = false;
  version: string = null;

  deserialize(json: JSON): GameConfig {
    this.isInstalled = json["isInstalled"];
    this.version = json["version"];
    return this;
  }
}

class GameRequirements implements Serializable<GameRequirements> {
  avx: boolean | null = null;
  openGL: boolean | null = null;

  deserialize(json: JSON): GameRequirements {
    this.avx = json["avx"];
    this.openGL = json["openGL"];
    return this;
  }
}

export class LauncherConfig implements Serializable<LauncherConfig> {
  version: string | null = "1.0";
  requirements: GameRequirements = new GameRequirements();
  games = {
    [SupportedGame.Jak1]: new GameConfig(),
    [SupportedGame.Jak2]: new GameConfig(),
    [SupportedGame.Jak3]: new GameConfig(),
    [SupportedGame.JakX]: new GameConfig(),
  };
  lastActiveGame: SupportedGame | null;

  private _loaded: boolean = false;

  deserialize(json: JSON): LauncherConfig {
    this.version = json["version"];
    this.requirements = new GameRequirements().deserialize(
      json["requirements"]
    );
    this.games[SupportedGame.Jak1] = new GameConfig().deserialize(
      json["games"][SupportedGame.Jak1]
    );
    this.games[SupportedGame.Jak2] = new GameConfig().deserialize(
      json["games"][SupportedGame.Jak2]
    );
    this.games[SupportedGame.Jak3] = new GameConfig().deserialize(
      json["games"][SupportedGame.Jak3]
    );
    this.games[SupportedGame.JakX] = new GameConfig().deserialize(
      json["games"][SupportedGame.JakX]
    );
    this.lastActiveGame = json["lastActiveGame"];
    return this;
  }

  private async loadConfigFromFile() {
    if (this._loaded) {
      return;
    }
    const configPath = await join(await appDir(), "settings.json");
    const configExists = await fileExists(configPath);
    if (!configExists) {
      console.log(
        `[Launcher]: Settings file not found at '${configPath}, initializing with defaults!`
      );
      await createDir(await appDir(), { recursive: true });
      await writeFile({
        contents: JSON.stringify(this, null, 2),
        path: configPath,
      });
      console.log("[Launcher]: Settings file initialized");
    } else {
      const data = await readTextFile(configPath);
      this.deserialize(JSON.parse(data));
    }
    this._loaded = true;
  }

  private async saveConfigToFile() {
    if (!this._loaded) {
      log.info("config not loaded when trying to save, initializing it first!");
      await this.loadConfigFromFile();
      return;
    }
    const path = await join(await appDir(), "settings.json");
    await writeFile({
      contents: JSON.stringify(this, null, 2),
      path: path,
    });
    log.info("settings file initialized");
  }

  /**
   * Checks the version to enable safe config operations
   *
   * Assumes the config has been loaded before calling to reduce boilerplate
   *
   * @param {*} version "<major>.<minor>"
   * @returns True if majors match, and expected minor greater than or equal to stored.  False otherwise, or if no version can be found
   */
  private validVersion(requiredVersion: string): boolean {
    const [requiredMajor, requiredMinor] = requiredVersion.split(".");
    if (this.version === null) {
      return false;
    }
    const [storedMajor, storedMinor] = this.version.split(".");
    if (requiredMajor != storedMajor) {
      return false;
    }
    if (parseInt(requiredMinor) < parseInt(storedMinor)) {
      return false;
    }
    return true;
  }

  // GETTERS

  /**
   * If a game is installed or not
   * @param {string} supportedGame
   * @returns {Promise<boolean>}
   */
  async getInstallStatus(supportedGame: SupportedGame): Promise<boolean> {
    await this.loadConfigFromFile();
    if (!this.validVersion("1.0")) {
      return false;
    }
    const gameConfigs = this.games;
    if (gameConfigs === null || !(supportedGame in gameConfigs)) {
      return false;
    }
    return gameConfigs[supportedGame].isInstalled;
  }

  /**
   * The last game that was considered active in the launcher
   * @returns {Promise<SupportedGame | null>}
   */
  async getLastActiveGame(): Promise<SupportedGame | null> {
    await this.loadConfigFromFile();
    if (!this.validVersion("1.0")) {
      return null;
    }
    return this.lastActiveGame;
  }

  /**
   * Checks the user config file to see if avx and openGL requirements are met.
   */
  async areRequirementsMet(): Promise<boolean> {
    await this.loadConfigFromFile();
    if (!this.validVersion("1.0")) {
      return false;
    }
    return this.requirements.avx && this.requirements.openGL;
  }

  async isAVXRequirementMet(): Promise<boolean> {
    await this.loadConfigFromFile();
    if (!this.validVersion("1.0")) {
      log.error("requirement false - AVX unsupported");
      return false;
    }
    return this.requirements.avx;
  }

  async isOpenGLRequirementMet(): Promise<boolean> {
    await this.loadConfigFromFile();
    if (!this.validVersion("1.0")) {
      log.error("requirement false - OpenGL 4.3 unsupported");
      return false;
    }
    return this.requirements.openGL;
  }

  async getGameInstallVersion(game: SupportedGame): Promise<String> {
    await this.loadConfigFromFile();
    if (!this.validVersion("1.0")) {
      return null;
    }
    return this.games[game].version;
  }

  private async getLatestProjectBinaryVersion(): Promise<string | null> {
    // TODO - make a LauncherMetadata class similar to this
    const appDirPath = await appDir();
    const userMetaPath = await join(appDirPath, "data", "metadata.json");
    // TODO - ensure it exists!
    const data = await readTextFile(userMetaPath);
    const { version } = JSON.parse(data);
    return version;
  }

  async shouldUpdateGameInstall(game: SupportedGame): Promise<Boolean> {
    const installVersion = await this.getGameInstallVersion(game);
    if (installVersion === null || installVersion === undefined) {
      return false;
    }
    const toolsVersion = await this.getLatestProjectBinaryVersion();

    if (installVersion === toolsVersion) {
      return false;
    }

    log.warn("Tools version is different than install verison", {
      tools: toolsVersion,
      installed: installVersion,
    });
    return true;
  }

  // SETTERS

  /**
   * @param {string} supportedGame
   * @param {boolean} installed
   * @returns
   */
  async setInstallStatus(
    supportedGame: SupportedGame,
    installed: boolean
  ): Promise<void> {
    this.games[supportedGame].isInstalled = installed;
    await this.saveConfigToFile();
  }

  async setRequirementsMet(
    avx: boolean = null,
    openGL: boolean = null
  ): Promise<void> {
    this.requirements.avx = avx;
    this.requirements.openGL = openGL;
    await this.saveConfigToFile();
  }

  async setGameInstallVersion(game: SupportedGame) {
    const version = await this.getLatestProjectBinaryVersion();
    this.games[game].version = version;
    await this.saveConfigToFile();
  }
}

// Initialize with defaults
export let launcherConfig: LauncherConfig = new LauncherConfig();
