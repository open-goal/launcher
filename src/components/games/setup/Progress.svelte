<script lang="ts">
  import {
    progressTracker,
    type ProgressStatus,
  } from "$lib/stores/ProgressStore";
  import ProgressIcon from "./ProgressIcon.svelte";

  function progressBarStyle(currentStatus: ProgressStatus) {
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

<div class="w-full pt-2 pb-6 flex justify-evenly gap-2">
  {#each $progressTracker.steps as step, i}
    <!-- NOTE - this will break if you add too many steps! -->
    {#if i > 0}
      <div class={progressBarStyle($progressTracker.steps[i - 1].status)}></div>
    {/if}
    <ProgressIcon {step} />
  {/each}
</div>
