<script>
  import { Toast } from "flowbite-svelte";
  import { toastStore } from "$lib/stores/ToastStore";
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import IconCheck from "~icons/mdi/check";
  import IconAlert from "~icons/mdi/stop-alert";
  import IconAlertCircle from "~icons/mdi/alert-circle";

  let open = false;
  let counter;

  function timeout() {
    if (--counter > 0) return setTimeout(timeout, 1000);
    open = false;
    toastStore.reset();
  }

  function trigger() {
    open = true;
    counter = 6;
    timeout();
  }

  $: if ($toastStore.msg) {
    trigger();
  } else {
    open = false;
  }
</script>

<Toast
  {open}
  dismissable={false}
  position="top-right"
  class="z-50 top-20"
  transition={fly}
  params={{ y: 200 }}
>
  <svelte:fragment slot="icon">
    {#if $toastStore.level == "info"}
      <IconCheck class="text-green-500 text-5xl" />
    {:else if $toastStore.level == "warn"}
      <IconAlertCircle class="text-orange-500 text-5xl" />
    {:else if $toastStore.level == "error"}
      <IconAlert class="text-red-500 text-5xl" />
    {/if}
  </svelte:fragment>
  <div class="ps-4 text-sm font-semibold">
    {$toastStore.msg}
  </div>
</Toast>
