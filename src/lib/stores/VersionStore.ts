import { writable } from "svelte/store";

export interface VersionStoreIFace {
  activeVersionName: string | null;
}

export const VersionStore = writable<VersionStoreIFace>({
  activeVersionName: null,
});

export const isMinVCCRuntime = writable<boolean>(false);

export const isMinMacOSVersion = writable<boolean | undefined>(false);
