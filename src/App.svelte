<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { Router } from "sv-router";
  import "./router.ts";
  import { isInDebugMode } from "$lib/utils/common";
  import {
    getLocale,
    isMinimumVCCRuntimeInstalled,
    setLocale,
  } from "$lib/rpc/config";
  import { listen } from "@tauri-apps/api/event";
  import {
    toastStore,
    type ToastLevel,
  } from "$lib/stores/ToastStore";
  import { isMinMacOSVersion, isMinVCCRuntime } from "$lib/stores/VersionStore";
  import { isMacOSVersion15OrAbove } from "$lib/rpc/util";

  let revokeSpecificActions = false;
  let toastListener: any = undefined;

  // Events
  onMount(async () => {
    // Set locale from settings
    const locale = await getLocale();
    if (locale !== null) {
      setLocale(locale);
    }

    isMinVCCRuntime.set(await isMinimumVCCRuntimeInstalled());
    isMinMacOSVersion.set(await isMacOSVersion15OrAbove());

    toastListener = await listen(
      "toast_msg",
      (event: { payload: { toast: string; level: ToastLevel } }) => {
        toastStore.makeToast(event.payload.toast, event.payload.level);
      },
    );
  });

  onDestroy(() => {
    if (toastListener !== undefined) {
      toastListener();
    }
  });

  if (!isInDebugMode()) {
    revokeSpecificActions = true;
    // Disable Right Click
    document.addEventListener("contextmenu", (event) => {
      if (revokeSpecificActions) {
        event.preventDefault();
      }
    });
    // Disable Refreshing (F5 / Ctrl+R)
    document.addEventListener("keydown", (e) => {
      if (e.code == "F5") {
        if (revokeSpecificActions) {
          e.preventDefault();
        }
      }
      if (e.code == "KeyR" && e.ctrlKey) {
        if (revokeSpecificActions) {
          e.preventDefault();
        }
      }
      // super secret keybind to reverse the above so we can debug a release version
      // Shift+Ctrl F12
      if (e.code == "F12" && e.ctrlKey && e.shiftKey) {
        revokeSpecificActions = false;
      }
    });
  }
</script>

<Router />
