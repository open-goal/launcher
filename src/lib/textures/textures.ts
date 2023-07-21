import { filePrompt } from "$lib/utils/file-dialogs";
import { path } from "@tauri-apps/api";
import { copyFile } from "@tauri-apps/api/fs";
import { appDir, join } from "@tauri-apps/api/path";

export async function texturePackPrompt(): Promise<string> {
  try {
    const path = await filePrompt(
      ["ZIP", "zip"],
      "Texture Pack Zip File",
      "Select a Texture Pack"
    );
    await copyTexturePackToZipFolder(path);
    return path;
  } catch (error) {
    console.error(error);
  }
}

async function copyTexturePackToZipFolder(pathToPack) {
  // split the pack name from the pathToPack, append it the dest path
  const packName = await path.basename(pathToPack);
  const textureZipDir = await join(
    await appDir(),
    "data/texture_zips",
    `${packName}`
  );
  try {
    await copyFile(pathToPack, textureZipDir, {});
  } catch (err) {
    console.log(err);
  }
}
