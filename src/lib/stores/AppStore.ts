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
