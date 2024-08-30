<script>
  import { Toast } from "flowbite-svelte";
  import { toastStore } from "$lib/stores/ToastStore";
  import { fly } from "svelte/transition";
  import IconCheck from "~icons/mdi/check";
  import IconAlert from "~icons/mdi/stop-alert";
  import IconAlertCircle from "~icons/mdi/alert-circle";

  let open = false;
  let counter;
  let currentToast = null;

  $: if ($toastStore.length > 0 && !currentToast) {
    currentToast = $toastStore[0];
    open = true;
    counter = 6;
    timeout();
  }

  function timeout() {
    if (--counter > 0) {
      return setTimeout(timeout, 1000);
    }
    open = false;
    toastStore.removeToast();
    currentToast = null;
  }
</script>

{#if currentToast}
  <Toast
    {open}
    dismissable={false}
    position="top-right"
    class="z-50 top-20 text-wrap"
    contentClass="w-full text-sm font-normal overflow-hidden"
    transition={fly}
    params={{ y: 200 }}
  >
    <svelte:fragment slot="icon">
      {#if currentToast.level == "info"}
        <IconCheck class="text-green-500 text-5xl" />
      {:else if currentToast.level == "warn"}
        <IconAlertCircle class="text-orange-500 text-5xl" />
      {:else if currentToast.level == "error"}
        <IconAlert class="text-red-500 text-5xl" />
      {/if}
    </svelte:fragment>
    <p class="text-sm font-semibold">
      {currentToast.msg}
    </p>
  </Toast>
{/if}
