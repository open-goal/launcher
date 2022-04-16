import { open } from '@tauri-apps/api/dialog';
import { createDir, readTextFile, writeFile } from '@tauri-apps/api/fs';
import { dirname, join, logDir } from '@tauri-apps/api/path';

export async function fileExists(path) {
  try {
    await readTextFile(path);
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

export async function clearInstallLogs(supportedGame, text) {
  const dir = await logDir();
  let fileName = `${supportedGame.name}-install.log`;
  let fullPath = await join(dir, fileName);
  if (await fileExists(fullPath)) {
    await writeFile({contents: "", path: fullPath});
  }
  fileName = `${supportedGame.name}-install-errors.log`;
  fullPath = await join(dir, fileName);
  if (await fileExists(fullPath)) {
    await writeFile({contents: "", path: fullPath});
  }
}

export async function appendToInstallLog(supportedGame, text) {
  const dir = await logDir();
  const fileName = `${supportedGame.name}-install.log`;
  const fullPath = await join(dir, fileName);
  console.log(`[OG]: Writing logs to ${fullPath}`);
  let contents = "";
  if (await fileExists(fullPath)) {
    contents = await readTextFile(fullPath);
  } else {
    await createDir(await dirname(fullPath), { recursive: true });
  }
  contents += text;
  await writeFile({contents: contents, path: fullPath});
}

export async function appendToInstallErrorLog(supportedGame, text) {
  const dir = await logDir();
  const fileName = `${supportedGame.name}-install-errors.log`;
  const fullPath = await join(dir, fileName);
  console.log(`[OG]: Writing logs to ${fullPath}`);
  let contents = "";
  if (await fileExists(fullPath)) {
    contents = await readTextFile(fullPath);
  } else {
    await createDir(await dirname(fullPath), { recursive: true });
  }
  contents += text;
  await writeFile({contents: contents, path: fullPath});
}
