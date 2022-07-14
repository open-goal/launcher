import { invoke } from "@tauri-apps/api/tauri";

export async function getHighestSimd(): Promise<string> {
  try {
    return await invoke("get_highest_simd");
  } catch (e) {
    if (e === "AVXNotSupported") {
      return "noavx";
    } else {
      return undefined;
    }
  }
}

export async function openDir(dir: string): Promise<void> {
  try {
    return await invoke("open_dir", { dir });
  } catch (e) {
    console.log(e);
  }
}

export async function closeSplashScreen() {
  try {
    invoke("close_splashscreen");
  } catch (e) {
    console.log(e);
  }
}

export async function copyDirectory(source: string, destination: string) {
  try {
    return await invoke("copy_dir", { dirSrc: source, dirDest: destination });
  } catch (e) {
    console.log(e);
  }
}
