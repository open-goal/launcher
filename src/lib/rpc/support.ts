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
    return await invoke("generate_support_package", { userPath: savePath });
  } catch (e) {
    exceptionLog("Unable to generate support package", e);
    // TODO - definitely need some kind of toast here
  }
}
