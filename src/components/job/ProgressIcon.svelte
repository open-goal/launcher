<script lang="ts">
  import IconCheck from "~icons/mdi/check";
  import IconDots from "~icons/mdi/dots-horizontal";
  import IconAlert from "~icons/mdi/stop-alert";
  import IconHourglass from "~icons/mdi/hourglass";
  import type { StepStatus } from "$lib/stores/JobStore";

  let { stepStatus, stepLabel }: { stepStatus: StepStatus; stepLabel: string } =
    $props();
</script>

<div>
  <div
    class="size-10 mx-auto bg-slate-900 rounded-full text-lg text-white flex justify-center items-center"
  >
    {#if stepStatus === "success"}
      <IconCheck color="#22c55e" />
    {:else if stepStatus === "pending"}
      <IconDots class="animate-pulse" color="#facc15" />
    {:else if stepStatus === "failed"}
      <IconAlert color="#ef4444" />
    {:else}
      <IconHourglass color="#737373" />
    {/if}
  </div>
  <div class="text-center text-outline font-semibold mt-2">
    {stepLabel}
  </div>
</div>
