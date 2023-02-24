import { open, save } from "@tauri-apps/api/dialog";
import { readDir, readTextFile } from "@tauri-apps/api/fs";

export async function fileExists(path: string): Promise<boolean> {
  try {
    await readTextFile(path);
    return true;
  } catch (err) {
    return false;
  }
}

export async function dirExists(path: string): Promise<boolean> {
  try {
    // NOTE - this isn't case sensitive!
    await readDir(path);
    return true;
  } catch (err) {
    return false;
  }
}

export async function filePrompt(
  extensions: string[],
  name: string
): Promise<string | null> {
  const path = await open({
    multiple: false,
    directory: false,
    filters: [{ extensions: extensions, name: name }],
  });

  if (Array.isArray(path) || path === null) {
    return null;
  }

  return path;
}

export async function saveFilePrompt(
  fileType: string,
  fileExtensions: string[]
): Promise<string | null> {
  return await save({
    filters: [{ name: fileType, extensions: fileExtensions }],
  });
}

export async function isoPrompt(): Promise<string | undefined> {
  const path = await filePrompt(["ISO", "iso"], "Jak ISO File");
  if (path === null) {
    return undefined;
  }
  return path;
}

export async function folderPrompt(title: string): Promise<string | undefined> {
  const path = await open({
    title: title,
    multiple: false,
    directory: true,
  });

  if (Array.isArray(path) || path === null) {
    return undefined;
  }

  return path;
}
