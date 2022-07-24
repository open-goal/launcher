import { open } from "@tauri-apps/api/dialog";
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
