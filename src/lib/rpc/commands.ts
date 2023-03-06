import { invoke } from "@tauri-apps/api/tauri";
import { appDir, join } from "@tauri-apps/api/path";
import { exceptionLog } from "./logging";

export async function closeSplashScreen() {
  try {
    invoke("close_splashscreen");
  } catch (e) {
    exceptionLog(
      "Unexpected error encountered when closing the splash screen",
      e
    );
  }
}
