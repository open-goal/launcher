import { writable, derived, get, type Updater } from "svelte/store";
import { browser } from "$app/environment";
import { invoke_rpc as invoke } from "$lib/rpc/rpc";
import { invoke as invoke_tauri } from "@tauri-apps/api/core";
import { getAppConfig } from "$lib/rpc/config";
import type { LauncherConfig } from "$lib/rpc/bindings/LauncherConfig";

type Status = "idle" | "loading" | "saving" | "error";

function createLauncherConfigStore() {
  const base = writable<LauncherConfig>({
    version: 1,
    locale: null,
    install_path: null,
  });
  const status = writable<Status>("idle");
  let initialized = false;

  // Debounce + coalesce writes
  let pendingPatch: Partial<LauncherConfig> | null = null;
  let debTimer: ReturnType<typeof setTimeout> | null = null;
  let inFlush = false;

  async function init() {
    if (!browser || initialized) return;
    status.set("loading");
    try {
      // adjust to your command name
      const cfg = await getAppConfig();
      base.set(cfg);
      listenForExternalChanges();
      handleCloseToFlush();
      initialized = true;
      status.set("idle");
      return cfg;
    } catch (e) {
      console.error("Failed to load settings", e);
      status.set("error");
    }
  }

  function patch(p: Partial<LauncherConfig>) {
    // optimistic: update the in-memory store
    base.update((curr) => ({ ...curr, ...p }));
    queuePersist(p);
  }

  function set(next: LauncherConfig) {
    base.set(next);
    queuePersist(next);
  }

  function update(fn: Updater<LauncherConfig>) {
    const next = fn(get(base));
    set(next);
  }

  function queuePersist(p: Partial<LauncherConfig> | LauncherConfig) {
    pendingPatch = { ...(pendingPatch ?? {}), ...p };
    if (debTimer) clearTimeout(debTimer);
    debTimer = setTimeout(flushPersist, 150);
  }

  async function flushPersist() {
    if (inFlush || !pendingPatch) return;
    inFlush = true;

    const patchToSend = pendingPatch;
    pendingPatch = null;

    const prev = get(base);
    status.set("saving");

    try {
      // Prefer one-shot patch command if you have it:
      // const updated = await invoke<LauncherConfig>('update_settings_patch', { patch: patchToSend });

      // Otherwise, fan out to your existing per-key updater:
      const keys = Object.keys(patchToSend) as (keyof LauncherConfig)[];
      for (const key of keys) {
        // Adjust the command/args to your API.
        await invoke(
          "update_setting_value",
          {
            key,
            value: patchToSend[key] as unknown,
          },
          () => {},
        );
      }
      // read back canonical config from disk (or trust in-memory if your writer is canonical)
      const updated = await getAppConfig();
      base.set(updated);
      status.set("idle");
    } catch (e) {
      console.error("Failed to persist settings", e);
      base.set(prev); // rollback
      status.set("error");
    } finally {
      inFlush = false;
    }
  }

  // If backend changes settings.json out-of-band, stay in sync.
  function listenForExternalChanges() {
    import("@tauri-apps/api/event").then(({ listen }) => {
      listen<LauncherConfig>("settingsUpdated", (e) => {
        base.set(e.payload);
      });
    });
  }

  // Ensure we don’t lose a pending patch on window close.
  function handleCloseToFlush() {
    import("@tauri-apps/api/event").then(({ listen }) => {
      listen("tauri://close-requested", async () => {
        if (debTimer) clearTimeout(debTimer);
        await flushPersist();
        // Let Tauri continue closing.
        await invoke_tauri("tauri://close");
      });
    });
  }

  return {
    subscribe: base.subscribe,
    status,
    init,
    patch,
    set,
    update,
    refresh: async () => {
      const cfg = await getAppConfig();
      base.set(cfg);
    },
  };
}

export const launcherConfig = createLauncherConfigStore();

// Handy per-key writable “slice” that persists on set().
export function keyStore<K extends keyof LauncherConfig>(key: K) {
  const read = derived(launcherConfig, (c) => c?.[key]);
  return {
    subscribe: read.subscribe,
    set: (val: LauncherConfig[K]) =>
      launcherConfig.patch({ [key]: val } as Partial<LauncherConfig>),
  };
}
