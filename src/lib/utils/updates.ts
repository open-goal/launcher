import { checkUpdate } from "@tauri-apps/api/updater";
import { UpdateStore } from '$lib/stores/AppStore';

export async function handleCheckUpdate() {
  const { shouldUpdate, manifest } = await checkUpdate();
  if (shouldUpdate) {
    UpdateStore.update((UpdateStore) => UpdateStore = { shouldUpdate, manifest });
    console.log('Update Available');
  } else {
    UpdateStore.set({ shouldUpdate: false, manifest: {} });
    console.log('Up to date');
  }
}