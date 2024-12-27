import { writable } from "svelte/store";

export const UpdateStore = writable({
  launcher: {
    updateAvailable: false,
    versionNumber: "",
    changeLog: [],
    date: "",
  },
  selectedTooling: {
    updateAvailable: false,
    versionNumber: "",
  },
});
