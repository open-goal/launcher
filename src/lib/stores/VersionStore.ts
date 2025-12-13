import { writable } from "svelte/store";

export const isMinVCCRuntime = writable<boolean>(false);

export const isMinMacOSVersion = writable<boolean | undefined>(false);
