export const ssr = false;
export const prerender = false;
import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import { getAppConfig } from "$lib/rpc/config";
import { launcherConfig } from "$lib/stores/Config";

export async function load() {
  const last = localStorage.getItem("lastGame") as SupportedGame;
  const lastGame = last ?? "jak1";
  // await launcherConfig.init(); // TODO: figure this out last
  return {
    config: await getAppConfig(),
    lastGame,
  };
}
