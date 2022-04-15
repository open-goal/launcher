import { createDir, readTextFile, writeFile } from '@tauri-apps/api/fs';
import { appDir, join } from '@tauri-apps/api/path';
import { Store } from 'tauri-plugin-store-api';

export class SupportedGame {
  static Jak1 = new SupportedGame("Jak 1")
  static Jak2 = new SupportedGame("Jak 2")
  static Jak3 = new SupportedGame("Jak 3")
  static JakX = new SupportedGame("Jak X")

  constructor(name) {
    this.name = name
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
}

const key_lastActiveGame = "lastActiveGame";
const store = new Store('settings.json');

export async function initConfig() {
  const path = await join(await appDir(), 'settings.json');
  try {
    await readTextFile(path);
  } catch (error) {
    console.log('[Launcher]: Settings file not found or could not be loaded!');
    await createDir(await appDir(), { recursive: true });
    const initSettings = {
      [SupportedGame.Jak1.name]: { "installed": false },
      [SupportedGame.Jak2.name]: { "installed": false },
      [SupportedGame.Jak3.name]: { "installed": false },
      [key_lastActiveGame]: SupportedGame.Jak1.name
    };
    await writeFile({ contents: JSON.stringify(initSettings, null, 2), path: path });
    console.log('[Launcher]: Settings file initialized');
  }
};

/**
 *
 * @param {SupportedGame} supportedGame
 * @returns {Promise<boolean>}
 */
export async function getInstallStatus(supportedGame) {
  await store.load();
  const val = await store.get(supportedGame.name);
  if (val == null || !'installed' in val) {
    return false;
  }
  return val.installed;
}

/**
 * @returns {Promise<SupportedGame>}
 */
export async function getLastActiveGame() {
  await store.load();
  const game = await store.get(key_lastActiveGame);
  if (game == null) {
    return SupportedGame.Jak1;
  }
  return SupportedGame.fromName(game);
}

/**
 * @param {SupportedGame} supportedGame
 * @param {boolean} installed
 * @returns
 */
export async function setInstallStatus(supportedGame, installed) {
  await store.load();
  await store.set(supportedGame.name, { "installed": installed });
  await store.save();
}
