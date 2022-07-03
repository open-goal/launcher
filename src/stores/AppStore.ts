import { writable } from "svelte/store";

export const InstallStatus = writable({
  status: undefined,
  percent: undefined,
});
export const isInstalling = writable(false);

export const dataFilesOutOfDate = writable(false);

export const Console = writable();
