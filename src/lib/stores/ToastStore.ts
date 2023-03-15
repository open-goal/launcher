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
  // TODO - the TTL isn't correct still, look into it
  const { subscribe, set, update } = writable<ToastStore>(storeValue);

  const ttl = 5000;
  let timeoutId: NodeJS.Timer;

  function ttlCheck() {
    return setTimeout(() => {
      update((val) => {
        val.msg = undefined;
        val.level = undefined;
        timeoutId = undefined;
        return val;
      });
    }, ttl);
  }

  return {
    subscribe,
    makeToast: (msg: string, level: ToastLevel) =>
      update((val) => {
        val.msg = msg;
        val.level = level;
        timeoutId = ttlCheck();
        return val;
      }),
  };
}

export const toastStore = createToastStore();
