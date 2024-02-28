import { toastStore } from "$lib/stores/ToastStore";
import { invoke_rpc } from "./rpc";

// TODO - just make this a generic interface for both binaries/feature jobs
interface FeatureJobOutput {
  msg: string | null;
  success: boolean;
}

function failed(msg: string): FeatureJobOutput {
  toastStore.makeToast(msg, "error");
  return { success: false, msg };
}

export async function listExtractedTexturePackInfo(
  gameName: string,
): Promise<any> {
  return await invoke_rpc(
    "list_extracted_texture_pack_info",
    {
      gameName: gameName,
    },
    () => [],
  );
}

export async function extractNewTexturePack(
  gameName: string,
  pathToZip: string,
): Promise<boolean | undefined> {
  return await invoke_rpc(
    "extract_new_texture_pack",
    {
      gameName: gameName,
      zipPath: pathToZip,
    },
    () => undefined,
  );
}

export async function updateTexturePackData(
  gameName: string,
): Promise<FeatureJobOutput> {
  return await invoke_rpc(
    "update_texture_pack_data",
    {
      gameName: gameName,
    },
    () => failed("Failed to delete texture packs"),
    undefined,
    () => {
      return { success: true, msg: null };
    },
  );
}

export async function deleteTexturePacks(
  gameName: string,
  packs: string[],
): Promise<FeatureJobOutput> {
  return await invoke_rpc(
    "delete_texture_packs",
    {
      gameName: gameName,
      packs: packs,
    },
    () => failed("Failed to delete texture packs"),
    undefined,
    () => {
      return { success: true, msg: null };
    },
  );
}
