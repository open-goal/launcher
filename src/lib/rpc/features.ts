import { invoke } from "@tauri-apps/api";
import { exceptionLog } from "./logging";

export async function listExtractedTexturePackInfo(
  gameName: string
): Promise<any> {
  try {
    return await invoke("list_extracted_texture_pack_info", {
      gameName: gameName,
    });
  } catch (e) {
    exceptionLog("Unable to list out extracted texture packs", e);
    return [];
  }
}

export async function extractNewTexturePack(
  gameName: string,
  pathToZip: string
): Promise<boolean | undefined> {
  try {
    return await invoke("extract_new_texture_pack", {
      gameName: gameName,
      zipPath: pathToZip,
    });
  } catch (e) {
    exceptionLog("Unable to extract texture pack", e);
    return undefined;
  }
}

export async function updateTexturePackData(gameName: string): Promise<void> {
  try {
    return await invoke("update_texture_pack_data", {
      gameName: gameName,
    });
  } catch (e) {
    exceptionLog("Unable to update texture pack data for game", e);
  }
}
