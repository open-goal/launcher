<script lang="ts">
  import type { LayoutProps } from "./$types";
  import "../app.css";
  import { page } from "$app/state";
  import { _ } from "svelte-i18n";
  import Sidebar from "../components/Sidebar.svelte";
  import Header from "../components/Header.svelte";
  import { listen } from "@tauri-apps/api/event";
  import type { LauncherConfig } from "$lib/rpc/bindings/LauncherConfig";
  import { launcherConfig } from "$lib/stores/Config";
  import { isInDebugMode } from "$lib/utils/common";

  let { data, children }: LayoutProps = $props();
  let revokeSpecificActions = false;

  listen<LauncherConfig>("settingsUpdated", (e) => {
    // launcherConfig.set(e.payload);
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

<div class="flex h-screen flex-col">
  <!-- HEADER -->
  <!-- I dislike passing the config using props and would rather use a global store, but im just experimenting right now -->
  <Header config={data.config}></Header>

  <div class="flex">
    <!-- I dislike this conditional, but i don't know how else to approach this... -->
    {#if !page.route.id?.includes("setup")}
      <Sidebar config={data.config}></Sidebar>
    {/if}

    <!-- BODY -->
    <main class="flex-1 overflow-hidden">
      {@render children()}
    </main>
  </div>
</div>
