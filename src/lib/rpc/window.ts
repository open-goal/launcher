import { invoke } from "@tauri-apps/api/tauri";

export async function openDir(dir: string): Promise<void> {
  try {
    return await invoke("open_dir", { dir });
  } catch (e) {
    console.log(`[OG] Error encountered when trying to open dir - ${dir}`, e);
  }
}
