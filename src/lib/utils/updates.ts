import { checkUpdate } from "@tauri-apps/api/updater";
import { UpdateStore } from "$lib/stores/AppStore";

export async function handleCheckUpdate() {
  const { shouldUpdate, manifest } = await checkUpdate();
  if (shouldUpdate) {
    const { changes } = JSON.parse(manifest.body);
    const { jak_project, launcher } = changes;
    UpdateStore.update(
      (UpdateStore) => (UpdateStore = { shouldUpdate, jak_project, launcher })
    );
    console.log("Update Available");
  } else {
    UpdateStore.update(
      (UpdateStore) => (UpdateStore.shouldUpdate = shouldUpdate)
    );
    console.log("Up to date");
  }
}
