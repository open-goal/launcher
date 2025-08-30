<script lang="ts">
  import type { LayoutProps } from "./$types";
  import "../app.css";
  import { page } from "$app/state";
  import { _ } from "svelte-i18n";
  import Sidebar from "../components/Sidebar.svelte";
  import Header from "../components/Header.svelte";
  import { listen } from "@tauri-apps/api/event";
  import type { LauncherConfig } from "$lib/rpc/bindings/LauncherConfig";

  let { data, children }: LayoutProps = $props();

  listen<LauncherConfig>("settingsUpdated", (e) => {
    // console.log(e.payload);
    // launcherConfig.set(e.payload);
  });
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
