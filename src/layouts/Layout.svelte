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

<div class="fixed inset-0 bg-[#141414] -z-11"></div>
<div class="relative flex h-screen max-w-none flex-col overflow-hidden">
  {#if !$isLoading}
    <Header />
    <div class="z-10 flex min-h-0 flex-1">
      <Sidebar />
      <main id="content" class="min-w-0 flex-1 overflow-hidden">
        {#key route.params.game_name}
          <Background />
          <div class="relative h-full overflow-y-auto">
            {@render children()}
          </div>
        {/key}
      </main>
    </div>
    <Toast />
  {/if}
</div>
