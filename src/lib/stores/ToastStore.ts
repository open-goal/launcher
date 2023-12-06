import { writable } from "svelte/store";

export type ToastLevel = "error" | "warn" | "info" | undefined;

interface ToastStore {
  msg: string | undefined;
  level: ToastLevel;
  interval: any;
}

const storeValue: ToastStore = {
  msg: undefined,
  level: undefined,
  interval: undefined,
};

function createToastStore() {
  const { subscribe, update } = writable<ToastStore>(storeValue);

  return {
    subscribe,
    makeToast: (msg: string, level: ToastLevel) =>
      update((val) => {
        val.msg = msg;
        val.level = level;
        return val;
      }),

    reset: () =>
      update((val) => {
        val.msg = undefined;
        val.level = undefined;
        return val;
      }),
  };
}

export const toastStore = createToastStore();
