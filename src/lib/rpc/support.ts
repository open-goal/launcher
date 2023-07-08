import { toastStore } from "$lib/stores/ToastStore";
import { saveFilePrompt } from "$lib/utils/file";
import { invoke } from "@tauri-apps/api/tauri";
import { exceptionLog } from "./logging";

export async function generateSupportPackage(): Promise<boolean> {
  try {
    const savePath = await saveFilePrompt(
      "ZIP",
      ["zip"],
      "opengoal-support-package.zip"
    );
    if (savePath !== null) {
      return await invoke("generate_support_package", { userPath: savePath });
    }
  } catch (e) {
    exceptionLog("Unable to generate support package", e);
    toastStore.makeToast("Unable to create support package", "error");
  }
}
