import { SETUP_SUCCESS, SETUP_ERROR, SupportedGame } from "$lib/constants";
import { open } from "@tauri-apps/api/dialog";
import { createDir, readTextFile, writeFile } from "@tauri-apps/api/fs";
import { dirname, join, logDir } from "@tauri-apps/api/path";
import { InstallStatus, Console } from '../../stores/InstallStore';


export async function fileExists(path: string): Promise<boolean> {
  try {
    await readTextFile(path);
    return true;
  } catch (err) {
    return false;
  }
}

export async function filePrompt(): Promise<string | string[]> {
  // TODO - pull strings out into args
  InstallStatus.update(() => SETUP_SUCCESS.awaitingISO);
  const path = await open({
    multiple: false,
    directory: false,
    filters: [{ extensions: ["ISO", "iso"], name: "Jak ISO File" }],
  });

  if (path) {
    return path;
  }
  InstallStatus.update(() => SETUP_ERROR.noISO);
  throw new Error("No ISO File Selected!");
}

export async function clearInstallLogs(supportedGame: SupportedGame) {
  Console.set(null);
  const dir = await logDir();
  let fileName = `${supportedGame}-install.log`;
  let fullPath = await join(dir, fileName);
  if (await fileExists(fullPath)) {
    await writeFile({ contents: "", path: fullPath });
  }
  fileName = `${supportedGame}-install-errors.log`;
  fullPath = await join(dir, fileName);
  if (await fileExists(fullPath)) {
    await writeFile({ contents: "", path: fullPath });
  }
}

export async function appendToInstallLog(supportedGame: SupportedGame, text: string) {
  const dir = await logDir();
  const fileName = `${supportedGame}-install.log`;
  const fullPath = await join(dir, fileName);
  console.log(`[OG]: Writing logs to ${fullPath}`);
  let contents: string;
  if (!(await fileExists(fullPath))) {
    await createDir(await dirname(fullPath), { recursive: true });
  }
  contents = await readTextFile(fullPath);
  contents += text;
  Console.update(() => contents);
  await writeFile({ contents: contents, path: fullPath });
}

export async function appendToInstallErrorLog(supportedGame: SupportedGame, text: string) {
  const dir = await logDir();
  const fileName = `${supportedGame}-install-errors.log`;
  const fullPath = await join(dir, fileName);
  console.log(`[OG]: Writing logs to ${fullPath}`);
  let contents: string;
  if (!(await fileExists(fullPath))) {
    await createDir(await dirname(fullPath), { recursive: true });
  }
  contents = await readTextFile(fullPath);
  contents += text;
  Console.update(() => contents);
  await writeFile({ contents: contents, path: fullPath });
}
