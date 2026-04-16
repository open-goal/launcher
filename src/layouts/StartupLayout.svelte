<script lang="ts">
  import type { Snippet } from "svelte";
  import { isLoading } from "svelte-i18n";
  import Header from "../components/header/Header.svelte";
  import Toast from "../components/toast/Toast.svelte";

  let { children }: { children: Snippet } = $props();
</script>

<div class="flex h-screen flex-col overflow-hidden bg-[#141414]">
  {#if !$isLoading}
    <Header />
    <main id="content" class="flex-1 overflow-hidden">
      {@render children()}
    </main>
    <Toast />
  {/if}
</div>
