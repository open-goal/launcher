import { writable } from "svelte/store";

export type ToastLevel = "error" | "warn" | "info" | undefined;

export type ToastMessage = {
  id: string;
  msg: string;
  level: ToastLevel;
  duration?: number;
};

function createToastStore() {
  const { subscribe, update } = writable<ToastMessage[]>([]);

  function push(msg: string, level: ToastLevel = "info", duration?: number) {
    const id = crypto.randomUUID?.() ?? Math.random().toString(36).slice(2);
    update((q) => [...q, { id, msg, level, duration }]);
    return id;
  }

  function removeToast(id?: string) {
    // If id omitted, remove the first (queue behavior)
    update((q) => (id ? q.filter((t) => t.id !== id) : q.slice(1)));
  }

  return { subscribe, push, removeToast };
}

export const toastStore = createToastStore();
