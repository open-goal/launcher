<script lang="ts">
  // components
  import Progress from "../setup/Progress.svelte";
  // constants
  import LogViewer from "../setup/LogViewer.svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { Button } from "flowbite-svelte";
  import { progressTracker } from "$lib/stores/ProgressStore";
  import type { Job } from "$lib/jobs/jobs";
  import { getInternalName, type SupportedGame } from "$lib/constants";
  import { runCompiler, runDecompiler } from "$lib/rpc/extractor";

  export let activeGame: SupportedGame;
  export let jobType: Job;

  const dispatch = createEventDispatcher();

  let running = false;

  // This is basically a stripped down `GameSetup` component that doesn't care about user initiation
  // requirement checking, etc
  //
  // It's used to provide almost the same interface as the normal installation, with logs, etc
  // but for arbitrary jobs.  Such as decompiling, or compiling.
  onMount(async () => {
    if (jobType === "decompile") {
      progressTracker.init([
        {
          status: "queued",
          label: "Decompile",
        },
        {
          status: "queued",
          label: "Done",
        },
      ]);
      progressTracker.start();
      await runDecompiler("", getInternalName(activeGame));
      progressTracker.proceed();
      progressTracker.proceed();
    } else if (jobType === "compile") {
      progressTracker.init([
        {
          status: "queued",
          label: "Compile",
        },
        {
          status: "queued",
          label: "Done",
        },
      ]);
      progressTracker.start();
      await runCompiler("", getInternalName(activeGame));
      progressTracker.proceed();
      progressTracker.proceed();
    }
  });

  function dispatchCompleteJob() {
    dispatch("jobFinished");
  }
</script>

<div class="flex flex-col justify-content">
  <Progress />
  {#if $progressTracker.logs}
    <LogViewer />
  {/if}
</div>
{#if $progressTracker.overallStatus === "success"}
  <div class="flex flex-col justify-end items-end mt-auto">
    <div class="flex flex-row gap-2">
      <Button
        btnClass="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => dispatchCompleteJob()}>Continue</Button
      >
    </div>
  </div>
{/if}