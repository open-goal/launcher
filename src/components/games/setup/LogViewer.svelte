<script lang="ts">
  import { progressTracker } from "$lib/stores/ProgressStore";
  import { listen } from "@tauri-apps/api/event";
  import { ansiSpan } from "ansi-to-span";
  import escapeHtml from "escape-html";
  import { onDestroy, onMount } from "svelte";
  import { _ } from "svelte-i18n";

  let logListener: any = undefined;
  let logElement;

  const scrollToBottom = async (node) => {
    node.scroll({ top: node.scrollHeight, behavior: 'smooth' });
  };

  onMount(async () => {
    logListener = await listen("log_update", (event) => {
      progressTracker.updateLogs(event.payload.logs);
      if (logElement) {
        scrollToBottom(logElement);
      }
    });
  });

  onDestroy(() => {
    if (logListener !== undefined) {
      logListener();
    }
  })

  function convertLogColors(text) {
    return ansiSpan(escapeHtml(text)).replaceAll("\n", "<br/>");
  }
</script>

<pre class="rounded p-2 bg-[#141414] text-[11px] max-h-[300px] overflow-auto text-pretty font-mono" bind:this={logElement}>{@html convertLogColors($progressTracker.logs)}</pre>
