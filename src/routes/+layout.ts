export const ssr = false;
export const prerender = true;
import { getAppConfig } from "$lib/rpc/config";

export async function load() {
  return {
    config: await getAppConfig(),
  };
}
