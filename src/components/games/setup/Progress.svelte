<script lang="ts">
  import {
    progressTracker,
    type ProgressStatus,
  } from "$lib/stores/ProgressStore";
  import ProgressIcon from "./ProgressIcon.svelte";

  $: progress = $progressTracker;

  const iconContainerStyle =
    "w-10 h-10 mx-auto border-solid border-2 border-slate-800 bg-slate-900 rounded-full text-lg text-white flex justify-center items-center";
  const stepLabelStyle = "text-center text-outline font-semibold";
  const progressBarContainerStyle =
    "w-full rounded items-center align-middle align-center flex-1";

  function progressBarStyle(currentStatus: ProgressStatus) {
    let style = "w-full py-1 rounded";
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

<div class="w-full py-6">
  <div class="flex">
    {#each progress.steps as step, i}
      <!-- NOTE - this will break if you add too many steps! -->
      <div class="grow">
        <div class="relative mb-2">
          <!-- BAR (skipped for first element) -->
          {#if i !== 0}
            <div
              class="absolute flex align-center items-center align-middle content-center"
              style="width: calc(100% - 2.5rem - 1rem); top: 50%; transform: translate(-50%, -50%)"
            >
              <div class={progressBarContainerStyle}>
                <div class={progressBarStyle(progress.steps[i - 1].status)} />
              </div>
            </div>
          {/if}
          <!-- ICON -->
          <div class={iconContainerStyle}>
            <ProgressIcon status={step.status} />
          </div>
        </div>
        <!-- LABEL -->
        <div class={stepLabelStyle}>
          {step.label}
        </div>
      </div>
    {/each}
  </div>
</div>
