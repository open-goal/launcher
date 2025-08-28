<script lang="ts">
  import { progressTracker } from "$lib/stores/ProgressStore";
  import { listen } from "@tauri-apps/api/event";
  import { ansiSpan } from "ansi-to-span";
  import escapeHtml from "escape-html";
  import { onDestroy, onMount } from "svelte";
  import { _ } from "svelte-i18n";

  let logContainer;
  let unlisten: () => void;

  onMount(async () => {
    const unlistenFn = await listen("log_update", (event) => {
      const newLogs = event.payload.logs
        .split("\n")
        .map((log) => ansiSpan(escapeHtml(log)).replaceAll("\n", "<br/>"))
        .filter((log) => log.length > 0);
      progressTracker.appendLogs(newLogs);

      requestAnimationFrame(() => {
        if (logContainer) {
          logContainer.scrollTop = logContainer.scrollHeight;
        }
      });
    });
    unlisten = unlistenFn;
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });
</script>

{#if $progressTracker.logs.length > 0}
  <div
    bind:this={logContainer}
    class="rounded bg-gray-900 text-[11px] max-h-[300px] overflow-auto font-mono px-2 py-2"
  >
    {#each $progressTracker.logs as log}
      <div>{@html log}</div>
    {/each}
  </div>
{/if}
