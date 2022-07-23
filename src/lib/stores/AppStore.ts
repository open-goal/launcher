import { writable } from "svelte/store";

export const InstallStatus = writable({
  status: undefined,
  percent: undefined,
});
export const isInstalling = writable(false);
export const isDecompiling = writable(false);
export const isCompiling = writable(false);
export const gameNeedsReinstall = writable(false);
export const ProcessLogs = writable("");
