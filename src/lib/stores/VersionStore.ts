import type { VersionFolders } from "$lib/rpc/versions";
import { writable } from "svelte/store";

export interface VersionStoreSelectedInfo {
  official: string | null;
}

export interface VersionStoreIFace {
  activeVersionType: VersionFolders;
  activeVersionName: string | null;
  selectedVersions: VersionStoreSelectedInfo;
}

export const VersionStore = writable<VersionStoreIFace>({
  activeVersionType: null,
  activeVersionName: null,
  selectedVersions: {
    official: null,
  },
});
