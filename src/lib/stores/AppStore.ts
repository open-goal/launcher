import { SupportedGame } from "$lib/constants";
import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
import { writable } from "svelte/store";

export const UpdateStore = writable({
  launcher: {
    updateAvailable: false,
    versionNumber: undefined,
    changeLog: [],
    date: undefined,
  },
  selectedTooling: {
    updateAvailable: false,
    versionNumber: undefined,
  },
});

export const activeGame = writable<SupportedGame>(undefined);

export const modInfoStore = writable<ModInfo | undefined>(undefined);
