import { log } from "$lib/utils/log";
import { appDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";
import { appDir, join } from "@tauri-apps/api/path";

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
    log.error(e);
  }
}

export async function closeSplashScreen() {
  try {
    invoke("close_splashscreen");
  } catch (e) {
    log.error(e);
  }
}

export async function copyDirectory(source: string, destination: string) {
  try {
    return await invoke("copy_dir", { dirSrc: source, dirDest: destination });
  } catch (e) {
    log.error(e);
  }
}

export async function extractTextures(texturesArray: Array<String>) {
  try {
    return await invoke("extract_textures", { texturesArray });
  } catch (e) {
    log.error(e);
  }
}

export async function getAllTexturePacks() {
  const textureZipDir = await join(await appDir(), "data/texture_zips/");
  try {
    return await invoke("get_all_texture_packs", { dir: textureZipDir });
  } catch (e) {
    log.error(e);
  }
}

export async function openREPL() {
  const appDirPath = await appDir();
  try {
    return await invoke("open_repl", {
      projPath: `${appDirPath}data`,
      currDir: appDirPath,
    });
  } catch (e) {
    log.error(e);
  }
}
