import type { PageLoad } from "./$types";
import { appLogDir } from "@tauri-apps/api/path";

export const load = (async () => {
  const appLogDirPath = await appLogDir();
  return { appLogDirPath };
}) satisfies PageLoad;
