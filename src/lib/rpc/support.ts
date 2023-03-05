import { filePrompt, saveFilePrompt } from "$lib/utils/file";
import { invoke } from "@tauri-apps/api/tauri";

export async function generateSupportPackage(): Promise<boolean> {
  try {
    const savePath = await saveFilePrompt(
      "ZIP",
      ["zip"],
      "opengoal-support-package.zip"
    );
    return await invoke("generate_support_package", { userPath: savePath });
  } catch (e) {
    console.log("[OG] Problem generating support package", e);
  }
}
