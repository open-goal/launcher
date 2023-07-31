<script lang="ts">
  import Progress from "../setup/Progress.svelte";
  import LogViewer from "../setup/LogViewer.svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { Alert, Button } from "flowbite-svelte";
  import { progressTracker } from "$lib/stores/ProgressStore";
  import type { Job } from "$lib/utils/jobs";
  import { getInternalName, type SupportedGame } from "$lib/constants";
  import {
    getEndOfLogs,
    runCompiler,
    runDecompiler,
    updateDataDirectory,
  } from "$lib/rpc/binaries";
  import {
    finalizeInstallation,
    setEnabledTexturePacks,
  } from "$lib/rpc/config";
  import { generateSupportPackage } from "$lib/rpc/support";
  import { _ } from "svelte-i18n";
  import { deleteTexturePacks, updateTexturePackData } from "$lib/rpc/features";

  export let activeGame: SupportedGame;
  export let jobType: Job;

  export let texturePacksToDelete: string[] = [];
  export let texturePacksToEnable: string[] = [];

  const dispatch = createEventDispatcher();
  let installationError = undefined;

  async function setupDecompileJob() {
    installationError = undefined;
    progressTracker.init([
      {
        status: "queued",
        label: $_("setup_decompile"),
      },
      {
        status: "queued",
        label: $_("setup_done"),
      },
    ]);
    progressTracker.start();
    let resp = await runDecompiler("", getInternalName(activeGame), true);
    progressTracker.updateLogs(await getEndOfLogs());
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    progressTracker.proceed();
  }

  async function setupCompileJob() {
    installationError = undefined;
    progressTracker.init([
      {
        status: "queued",
        label: $_("setup_compile"),
      },
      {
        status: "queued",
        label: $_("setup_done"),
      },
    ]);
    progressTracker.start();
    let resp = await runCompiler("", getInternalName(activeGame), true);
    progressTracker.updateLogs(await getEndOfLogs());
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    progressTracker.proceed();
  }

  async function setupUpdateGameJob() {
    installationError = undefined;
    progressTracker.init([
      {
        status: "queued",
        label: $_("setup_copyFiles"),
      },
      {
        status: "queued",
        label: $_("setup_decompile"),
      },
      {
        status: "queued",
        label: $_("setup_compile"),
      },
      {
        status: "queued",
        label: $_("setup_done"),
      },
    ]);
    progressTracker.start();
    let resp = await updateDataDirectory(getInternalName(activeGame));
    progressTracker.updateLogs(await getEndOfLogs());
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await runDecompiler("", getInternalName(activeGame), true);
    progressTracker.updateLogs(await getEndOfLogs());
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await runCompiler("", getInternalName(activeGame));
    progressTracker.updateLogs(await getEndOfLogs());
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    await finalizeInstallation("jak1");
    progressTracker.proceed();
  }

  async function setupTexturePacks() {
    installationError = undefined;
    let jobs = [];
    if (texturePacksToDelete.length > 0) {
      jobs.push({
        status: "queued",
        label: $_("gameJob_deleteTexturePacks"),
      });
    }
    jobs.push(
      {
        status: "queued",
        label: $_("gameJob_enablingTexturePacks"),
      },
      {
        status: "queued",
        label: $_("gameJob_applyTexturePacks"),
      },
      {
        status: "queued",
        label: $_("setup_decompile"),
      },
    );
    progressTracker.init(jobs);
    progressTracker.start();
    if (texturePacksToDelete.length > 0) {
      let resp = await deleteTexturePacks(
        getInternalName(activeGame),
        texturePacksToDelete,
      );
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
    }
    let resp = await setEnabledTexturePacks(
      getInternalName(activeGame),
      texturePacksToEnable,
    );
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await updateTexturePackData(getInternalName(activeGame));
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await runDecompiler("", getInternalName(activeGame), true);
    progressTracker.updateLogs(await getEndOfLogs());
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
  }

  // This is basically a stripped down `GameSetup` component that doesn't care about user initiation,
  // requirement checking, etc
  //
  // It's used to provide almost the same interface as the normal installation, with logs, etc
  // but for arbitrary jobs.  Such as updating versions, decompiling, or compiling.
  onMount(async () => {
    if (jobType === "decompile") {
      await setupDecompileJob();
    } else if (jobType === "compile") {
      await setupCompileJob();
    } else if (jobType === "updateGame") {
      await setupUpdateGameJob();
    } else if (jobType === "updateTexturePacks") {
      await setupTexturePacks();
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
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => dispatchCompleteJob()}
        >{$_("setup_button_continue")}</Button
      >
    </div>
  </div>
{:else if $progressTracker.overallStatus === "failed"}
  <div class="flex flex-col mt-auto">
    <div class="flex flex-row gap-2">
      <Alert color="red" class="dark:bg-slate-900 flex-grow">
        <span class="font-medium text-red-500"
          >{$_("setup_installationFailed")}
        </span><span class="text-white"> {installationError}</span>
      </Alert>
      <!-- TODO - no button to go back! -->
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => await generateSupportPackage()}
        >{$_("setup_button_getSupportPackage")}</Button
      >
    </div>
  </div>
{/if}
