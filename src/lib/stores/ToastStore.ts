import { writable } from "svelte/store";

export type ToastLevel = "error" | "warn" | "info" | undefined;

export interface ToastMessage {
  msg: string;
  level: ToastLevel;
  interval?: any;
}

type ToastArray = ToastMessage[];
const messageArray: ToastArray = [];

function createToastStore() {
  const { subscribe, update } = writable<ToastArray>(messageArray);

  return {
    subscribe,
    removeToast: () =>
      update((val) => {
        val = val.slice(1);
        return val;
      }),
    makeToast: (msg: string, level: ToastLevel) =>
      update((val) => {
        val.push({ msg, level });
        return val;
      }),
  };
}

export const toastStore = createToastStore();
