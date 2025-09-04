<script lang="ts">
  import { Toast } from "flowbite-svelte";
  import IconCheck from "~icons/mdi/check";
  import IconAlert from "~icons/mdi/stop-alert";
  import IconAlertCircle from "~icons/mdi/alert-circle";
  import { toastStore } from "$lib/stores/ToastStore";

  type Level = "info" | "warn" | "error";
  type ToastItem = { id: string; msg: string; level: Level; duration?: number };

  let queue = $derived($toastStore as ToastItem[]);
  let current = $state<ToastItem | null>(null);
  let toastStatus = $state(false);

  let timeoutId: ReturnType<typeof setTimeout> | null = null;
  const clearTimer = () =>
    timeoutId ? (clearTimeout(timeoutId), (timeoutId = null)) : null;

  function showNext() {
    if (current || queue.length === 0) return;
    current = queue[0];
    toastStatus = true;
    clearTimer();
    const ms = current.duration ?? 6000; // 6000
    timeoutId = setTimeout(() => (toastStatus = false), ms);
  }

  function finalizeAndAdvance() {
    clearTimer();
    toastStore.removeToast();
    current = null;
    showNext();
  }

  $effect(() => {
    if (!current && queue.length > 0) showNext();
  });

  $effect(() => {
    if (current && !toastStatus) queueMicrotask(finalizeAndAdvance);
  });
</script>

{#if current}
  <Toast
    bind:toastStatus
    dismissable={false}
    position="top-right"
    class="z-50 top-20 right-4 text-wrap"
    classes={{ content: "w-full text-sm font-normal overflow-hidden" }}
  >
    {#snippet icon()}
      {#if current?.level === "info"}
        <IconCheck class="text-green-500 text-5xl" />
      {:else if current?.level === "warn"}
        <IconAlertCircle class="text-orange-500 text-5xl" />
      {:else}
        <IconAlert class="text-red-500 text-5xl" />
      {/if}
    {/snippet}

    <p class="text-sm font-semibold">{current.msg}</p>
  </Toast>
{/if}
