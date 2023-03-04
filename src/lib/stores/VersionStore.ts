import type { VersionFolders } from "$lib/rpc/versions";
import { writable } from "svelte/store";

export interface VersionStoreSelectedInfo {
  official: string | undefined;
  unofficial: string | undefined;
  devel: string | undefined;
}

export interface VersionStoreIFace {
  activeVersionType: VersionFolders;
  activeVersionName: string | undefined;
  selectedVersions: VersionStoreSelectedInfo;
}

export const VersionStore = writable<VersionStoreIFace>({
  activeVersionType: undefined,
  activeVersionName: undefined,
  selectedVersions: {
    official: undefined,
    unofficial: undefined,
    devel: undefined,
  },
});
