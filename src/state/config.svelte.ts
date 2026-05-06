import { getLauncherConfig } from "$lib/rpc/config";
import type { LauncherConfig } from "$lib/rpc/bindings/LauncherConfig";
import { listen } from "@tauri-apps/api/event";

export const config = $state<Partial<LauncherConfig>>({});

async function refresh() {
  const next = await getLauncherConfig();
  Object.assign(config, next);
}

export async function initConfig() {
  await refresh();
  return listen("config:saved", refresh);
}
