<script lang="ts">
  import type { Snippet } from "svelte";
  import { isLoading } from "svelte-i18n";
  import Background from "../../components/background/Background.svelte";
  import Header from "../../components/header/Header.svelte";
  import Sidebar from "../../components/sidebar/Sidebar.svelte";
  import Toast from "../../components/toast/Toast.svelte";

  let { children }: { children: Snippet } = $props();
</script>

<div class={`container h-screen max-w-none flex flex-col bg-black`}>
  {#if !$isLoading}
    <Background />
    <Header />
    <div class="flex flex-row grow shrink h-[90%] z-10">
      <Sidebar />
      <main id="content" class="overflow-y-auto grow shrink">
        {@render children()}
      </main>
    </div>
    <Toast />
  {/if}
</div>
