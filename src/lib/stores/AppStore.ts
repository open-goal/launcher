import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
import { writable } from "svelte/store";
interface UpdateState {
  selectedTooling: {
    updateAvailable: boolean;
    versionNumber: string | undefined;
  };
}

export const UpdateStore = writable<UpdateState>({
  selectedTooling: {
    updateAvailable: false,
    versionNumber: undefined,
  },
});