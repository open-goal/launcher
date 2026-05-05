import { getLauncherConfig } from "$lib/rpc/config";
import type { LauncherConfig } from "$lib/rpc/bindings/LauncherConfig";

export const config = $state<LauncherConfig | null>(await getLauncherConfig());
