<script lang="ts">
  import type { Snippet } from "svelte";
  import { isLoading } from "svelte-i18n";
  import Background from "../components/background/Background.svelte";
  import Header from "../components/header/Header.svelte";
  import Sidebar from "../components/sidebar/Sidebar.svelte";
  import Toast from "../components/toast/Toast.svelte";
  import { route } from "/src/router";

  let { children }: { children: Snippet } = $props();
</script>

<div class="relative flex h-screen max-w-none flex-col overflow-hidden">
  {#if !$isLoading}
    <Header />
    <div class="z-10 flex min-h-0 flex-1">
      <Sidebar />
      {#key route.params.game_name}
        <main id="content" class="min-w-0 flex-1 overflow-y-auto">
          <Background />
          {@render children()}
        </main>
      {/key}
    </div>
    <Toast />
  {/if}
</div>
