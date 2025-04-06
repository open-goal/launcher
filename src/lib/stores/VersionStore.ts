import { writable } from "svelte/store";

export interface VersionStoreIFace {
  activeVersionName: string | null;
}

export const VersionStore = writable<VersionStoreIFace>({
  activeVersionName: null,
});

export const isMinVCCRuntime = writable<Boolean>();
