<script lang="ts">
  import { Button } from "flowbite-svelte";
  import type { PageProps } from "./$types";
  import { _ } from "svelte-i18n";
  import { goto } from "$app/navigation";

  let { data }: PageProps = $props();
  const game = $derived(data.game);
  let installing = $state(false);
  const proceedAfterSuccessfulOperation = false;
  async function install() {
    installing = true;
  }
</script>

<div class="flex flex-col h-full items-center justify-center p-4 gap-3">
  <h1
    class="tracking-tighter text-2xl font-bold pb-3 text-orange-500 text-outline"
  >
    {$_(`gameName_${game}`)}
  </h1>

  {#if installing}
    <div class="flex flex-col justify-content shrink">
      <!-- <Progress /> -->
      <!-- <LogViewer /> -->
    </div>
  {:else}
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      onclick={async () => await install()}
      >{$_("setup_button_installViaISO")}</Button
    >
  {/if}

  {#if !proceedAfterSuccessfulOperation}
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      onclick={async () => goto(`/${game}`)}
      >{$_("setup_button_continue")}
    </Button>
  {/if}
</div>
