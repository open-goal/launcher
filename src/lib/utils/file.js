import { open } from "@tauri-apps/api/dialog";
import { readTextFile } from "@tauri-apps/api/fs";

export async function fileExists(path) {
  try {
    readTextFile(path);
    return true;
  } catch (err) {
    return false;
  }
}

export async function filePrompt(title) {
  // TODO - pull strings out into args
  const path = await open({
    multiple: false,
    directory: false,
    filters: [{ extensions: ["ISO"], name: "Jak ISO File" }],
  });

  if (path) {
    return path;
  }
  return null;
}
