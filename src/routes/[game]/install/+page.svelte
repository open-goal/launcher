<script lang="ts">
  import { Button } from "flowbite-svelte";
  import type { PageProps } from "./$types";
  import { _ } from "svelte-i18n";
  import { goto } from "$app/navigation";
  import { installBaseGame } from "$lib/utils/jobs";
  import Progress from "../../../old_components/Progress.svelte";
  import LogViewer from "../../../components/LogViewer.svelte";
  import { progressTracker } from "$lib/stores/ProgressStore";

  let { data }: PageProps = $props();
  const game = $derived(data.game);
  const config = $derived(data.config);
  const installed = $derived(config.games[game]?.isInstalled);
  const proceedAfterInstall = $derived(data.proceed);
  let installing = $state(false);
  const showLogs = $derived(installing || (installed && !proceedAfterInstall));
</script>

<div class="flex flex-col h-full items-center p-4 gap-3">
  <h1 class="text-2xl font-bold pb-3 text-orange-500 text-outline">
    {$_(`gameName_${game}`)}
  </h1>

  {#if !showLogs}
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      onclick={async () => {
        installing = true;
        await installBaseGame(game);
        installing = false;
      }}>{$_("setup_button_installViaISO")}</Button
    >
  {/if}

  {#if showLogs}
    <div class="flex flex-col w-full justify-content shrink">
      {#if installing}
        <Progress />
      {/if}

      <LogViewer />

      {#if installed && !proceedAfterInstall}
        <Button
          class="mt-8 border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
          onclick={() => {
            progressTracker.clear();
            goto(`/${game}`);
          }}
          >{$_("setup_button_continue")}
        </Button>
      {/if}
    </div>
  {/if}
</div>
