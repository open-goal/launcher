
import { readTextFile } from '@tauri-apps/api/fs';

export async function fileExists(path) {
  try {
    readTextFile(path);
    return true;
  } catch (err) {
    return false;
  }
}
