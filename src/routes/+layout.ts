export const ssr = false;
export const prerender = true;
import { getAppConfig } from "$lib/rpc/config";
import { launcherConfig } from "$lib/stores/Config";

export async function load() {
  // await launcherConfig.init(); // TODO: figure this out last
  return {
    config: await getAppConfig(),
  };
}
