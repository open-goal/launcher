<script lang="ts">
  import { jobTracker, type StepStatus } from "$lib/stores/JobStore";
  import ProgressIcon from "./ProgressIcon.svelte";

  function progressBarStyle(currentStatus: StepStatus) {
    let style = "w-xs h-min py-1 my-4 rounded";
    if (currentStatus === "success") {
      style += " bg-green-500";
    } else if (currentStatus === "pending") {
      style += " bg-yellow-400 animate-pulse";
    } else if (currentStatus === "queued") {
      style += " bg-slate-900";
    } else if (currentStatus === "failed") {
      style += " bg-red-500";
    }
    return style;
  }
</script>

<div class="w-full mt-2 mb-5 flex justify-evenly gap-2">
  {#each $jobTracker.steps as step, i}
    <!-- NOTE - this will break if you add too many steps! -->
    <ProgressIcon stepStatus={step.status} stepLabel={step.label} />
    {#if i < $jobTracker.steps.length - 1}
      <div class={progressBarStyle(step.status)}></div>
    {/if}
  {/each}
</div>
