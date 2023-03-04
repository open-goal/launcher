import { invoke } from "@tauri-apps/api/tauri";
import { appDir, join } from "@tauri-apps/api/path";

export async function closeSplashScreen() {
  try {
    invoke("close_splashscreen");
  } catch (e) {}
}

export async function extractTextures(texturesArray: Array<String>) {
  try {
    return await invoke("extract_textures", { texturesArray });
  } catch (e) {}
}

export async function getAllTexturePacks() {
  const textureZipDir = await join(await appDir(), "data/texture_zips/");
  try {
    return await invoke("get_all_texture_packs", { dir: textureZipDir });
  } catch (e) {}
}
