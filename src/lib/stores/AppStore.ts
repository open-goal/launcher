import { writable } from "svelte/store";

export const InstallationProgress = writable({
  currentStep: 0,
  steps: [
    {
      status: "queued",
    },
    {
      status: "queued",
    },
    {
      status: "queued",
    },
    {
      status: "queued",
    },
  ],
});

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

export const isInstalling = writable(false);
export const isDecompiling = writable(false);
export const isCompiling = writable(false);
export const gameNeedsReinstall = writable(false);
export const ProcessLogs = writable("");
