import { Store } from 'tauri-plugin-store-api';
import { renderControls } from '../../main';
import { createDir, readTextFile, writeFile } from '@tauri-apps/api/fs';
import { appDir, join } from '@tauri-apps/api/path';

const store = new Store('settings.json');

// TODO - a general -- does a file exist function -- would be nice

(async function initStore() {
  const path = await join(await appDir(), 'settings.json');
  try {
    await readTextFile(path);
  } catch (error) {
    await createDir(await appDir(), { recursive: true });
    const initSettings = {
      "Jak 1": { "installed": false },
      "Jak 2": { "installed": false },
      "Jak 3": { "installed": false }
    };
    const x = await writeFile({ contents: JSON.stringify(initSettings, null, 2), path: path });
    console.log('Created settings.json file');
    // once again probably not a good idea to use this render function here
    renderControls();
  }
})();

export async function getInstallStatus(game) {
  await store.load();
  const { installed } = await store.get(game);
  return installed;
}

export async function setInstallStatus() {
  await store.load();
  await store.set("Jak 1", { "installed": true });
  await store.save();
  // calling this render function from here is probably a bad idea but im not sure how else to rerender the page
  renderControls();
  return;
}
