<script lang="ts">
  import { progressTracker } from "$lib/stores/ProgressStore";
  import { listen } from "@tauri-apps/api/event";
  import { ansiSpan } from "ansi-to-span";
  import escapeHtml from "escape-html";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";

  onMount(async () => {
    await listen("log_update", (event) => {
      const newLogs = event.payload.logs
        .split("\n")
        .map((log) => ansiSpan(escapeHtml(log)).replaceAll("\n", "<br/>"))
        .filter((log) => log.length > 0);
      progressTracker.appendLogs(newLogs);
    });
  });
</script>

{#if $progressTracker.logs.length > 0}
  <pre
    class="rounded bg-[#141414] text-[11px] max-h-[300px] overflow-auto font-mono">{#each $progressTracker.logs as log}
      {@html log}
    {/each}
</pre>
{/if}
