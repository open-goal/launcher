import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
import { writable } from "svelte/store";

export const UpdateStore = writable({
  selectedTooling: {
    updateAvailable: false,
    versionNumber: undefined,
  },
});

export const activeGame = writable<SupportedGame>(undefined);

export const modInfoStore = writable<ModInfo | undefined>(undefined);
