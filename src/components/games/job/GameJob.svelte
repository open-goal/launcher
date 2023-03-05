<script lang="ts">
  // components
  import Progress from "../setup/Progress.svelte";
  // constants
  import LogViewer from "../setup/LogViewer.svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { Alert, Button } from "flowbite-svelte";
  import { progressTracker } from "$lib/stores/ProgressStore";
  import type { Job } from "$lib/jobs/jobs";
  import { getInternalName, type SupportedGame } from "$lib/constants";
  import {
    runCompiler,
    runDecompiler,
    updateDataDirectory,
  } from "$lib/rpc/binaries";
  import { finalizeInstallation } from "$lib/rpc/config";
  import { generateSupportPackage } from "$lib/rpc/support";
  import { listen } from "@tauri-apps/api/event";

  export let activeGame: SupportedGame;
  export let jobType: Job;

  const dispatch = createEventDispatcher();
  let installationError = undefined;

  // This is basically a stripped down `GameSetup` component that doesn't care about user initiation
  // requirement checking, etc
  //
  // It's used to provide almost the same interface as the normal installation, with logs, etc
  // but for arbitrary jobs.  Such as updating versions, decompiling, or compiling.
  onMount(async () => {
    const unlistenLogListener = await listen("updateJobLogs", async (event) => {
      progressTracker.updateLogs(event.payload["stdout"]);
    });

    if (jobType === "decompile") {
      installationError = undefined;
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
      let resp = await runDecompiler("", getInternalName(activeGame));
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      progressTracker.proceed();
    } else if (jobType === "compile") {
      installationError = undefined;
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
      let resp = await runCompiler("", getInternalName(activeGame));
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      progressTracker.proceed();
    } else if (jobType === "updateGame") {
      installationError = undefined;
      progressTracker.init([
        {
          status: "queued",
          label: "Copy Files",
        },
        {
          status: "queued",
          label: "Decompile",
        },
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
      let resp = await updateDataDirectory(getInternalName(activeGame));
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      resp = await runDecompiler("", getInternalName(activeGame));
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      resp = await runCompiler("", getInternalName(activeGame));
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      await finalizeInstallation("jak1");
      progressTracker.proceed();
    }
  });

  function dispatchCompleteJob() {
    dispatch("jobFinished");
  }
</script>

<div class="flex flex-col justify-content">
  <Progress />
  {#if $progressTracker.logs !== undefined}
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
{:else if $progressTracker.overallStatus === "failed"}
  <div class="flex flex-col mt-auto">
    <div class="flex flex-row gap-2">
      <Alert color="red" class="dark:bg-slate-900 flex-grow" accent={true}>
        <span class="font-medium text-red-500"
          >Installation has failed!
        </span><span class="text-white"> {installationError}</span>
      </Alert>
      <Button
        btnClass="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => await generateSupportPackage()}
        >Get Support Package</Button
      >
    </div>
  </div>
{/if}
