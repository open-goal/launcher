<script lang="ts">
  import Progress from "../setup/Progress.svelte";
  import LogViewer from "../setup/LogViewer.svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { Alert, Button } from "flowbite-svelte";
  import { progressTracker } from "$lib/stores/ProgressStore";
  import type { Job } from "$lib/utils/jobs";
  import {
    runCompiler,
    runDecompiler,
    updateDataDirectory,
  } from "$lib/rpc/binaries";
  import {
    finalizeInstallation,
    getProceedAfterSuccessfulOperation,
    setEnabledTexturePacks,
  } from "$lib/rpc/config";
  import { generateSupportPackage } from "$lib/rpc/support";
  import { _ } from "svelte-i18n";
  import {
    baseGameIsoExists,
    compileForModInstall,
    decompileForModInstall,
    deleteTexturePacks,
    downloadAndExtractNewMod,
    extractIsoForModInstall,
    saveModInstallInfo,
    updateTexturePackData,
  } from "$lib/rpc/features";
  import { isoPrompt } from "$lib/utils/file-dialogs";
  import { emit } from "@tauri-apps/api/event";
  import { activeGame } from "$lib/stores/AppStore";

  export let jobType: Job;

  // texture packs
  export let texturePacksToDelete: string[] | undefined = undefined;
  export let texturePacksToEnable: string[] | undefined = undefined;

  // mods
  export let modDownloadUrl: string | undefined = undefined;
  export let modSourceName: string | undefined = undefined;
  export let modName: string | undefined = undefined;
  export let modVersion: string | undefined = undefined;

  const dispatch = createEventDispatcher();
  let installationError: string | undefined | null = undefined;
  let proceedAfterSuccessfulOperation = true;

  onMount(async () => {
    proceedAfterSuccessfulOperation =
      await getProceedAfterSuccessfulOperation();
  });

  $: if (
    $progressTracker.overallStatus === "success" &&
    proceedAfterSuccessfulOperation
  ) {
    progressTracker.clear();
    dispatch("jobFinished");
  }

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
    let resp = await runDecompiler("", $activeGame, true, true);
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
    let resp = await runCompiler("", $activeGame, true);
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
    let resp = await updateDataDirectory($activeGame);
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await runDecompiler("", $activeGame, true, false);
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await runCompiler("", $activeGame);
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    await finalizeInstallation($activeGame);
    await emit("gameInstalled");
    progressTracker.proceed();
    location.reload();
  }

  async function setupModInstallation() {
    // Check to see if we need to prompt for the ISO or not
    installationError = undefined;
    let jobs = [];
    const isoAlreadyExtracted = await baseGameIsoExists($activeGame);
    if (!isoAlreadyExtracted) {
      jobs.push({
        status: "queued",
        label: $_("setup_extractAndVerify"),
      });
    }
    jobs.push(
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
    );
    progressTracker.init(jobs);
    progressTracker.start();
    if (!isoAlreadyExtracted) {
      let sourcePath = await isoPrompt(
        $_("setup_prompt_ISOFileLabel"),
        $_("setup_prompt_selectISO"),
      );
      if (sourcePath !== undefined) {
        let resp = await extractIsoForModInstall(
          $activeGame,
          modName,
          modSourceName,
          sourcePath,
        );
        if (!resp.success) {
          progressTracker.halt();
          installationError = resp.msg;
          return;
        }
      } else {
        progressTracker.halt();
        installationError = "Can't continue without an ISO - TODO translate";
        return;
      }
      progressTracker.proceed();
    }
    let resp = await decompileForModInstall(
      $activeGame,
      modName,
      modSourceName,
    );
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await compileForModInstall($activeGame, modName, modSourceName);
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await saveModInstallInfo(
      $activeGame,
      modName,
      modSourceName,
      modVersion,
    );
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
  }

  async function setupModInstallationExternal() {
    // Check to see if we need to prompt for the ISO or not
    installationError = undefined;
    let jobs = [];
    const isoAlreadyExtracted = await baseGameIsoExists($activeGame);
    jobs.push(
      {
        status: "queued",
        label: $_("setup_extractAndVerify"),
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
    );
    progressTracker.init(jobs);
    progressTracker.start();
    if (!isoAlreadyExtracted) {
      let sourcePath = await isoPrompt(
        $_("setup_prompt_ISOFileLabel"),
        $_("setup_prompt_selectISO"),
      );
      if (sourcePath !== undefined) {
        let resp = await extractIsoForModInstall(
          $activeGame,
          modName,
          modSourceName,
          sourcePath,
        );
        if (!resp.success) {
          progressTracker.halt();
          installationError = resp.msg;
          return;
        }
      } else {
        progressTracker.halt();
        installationError = "Can't continue without an ISO - TODO translate";
        return;
      }
    }
    // extract the file into install_dir/features/<game>/<sourceName>/<modName>
    let resp = await downloadAndExtractNewMod(
      $activeGame,
      modDownloadUrl,
      modName,
      modSourceName,
    );
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await decompileForModInstall($activeGame, modName, modSourceName);
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await compileForModInstall($activeGame, modName, modSourceName);
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    resp = await saveModInstallInfo(
      $activeGame,
      modName,
      modSourceName,
      modVersion,
    );
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
  }

  async function setupDecompileModJob() {
    // Check to see if we need to prompt for the ISO or not
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
    let resp = await decompileForModInstall(
      $activeGame,
      modName,
      modSourceName,
    );
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    progressTracker.proceed();
  }

  async function setupCompileModJob() {
    // Check to see if we need to prompt for the ISO or not
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
    let resp = await compileForModInstall($activeGame, modName, modSourceName);
    if (!resp.success) {
      progressTracker.halt();
      installationError = resp.msg;
      return;
    }
    progressTracker.proceed();
    progressTracker.proceed();
  }

  // This is basically a stripped down `GameSetup` component that doesn't care about user initiation,
  // requirement checking, etc
  //
  // It's used to provide almost the same interface as the normal installation, with logs, etc
  // but for arbitrary jobs.  Such as updating versions, decompiling, or compiling.
  //
  // TODO - break this up into multiple files, getting cumbersome
  onMount(async () => {
    if (jobType === "decompile") {
      await setupDecompileJob();
    } else if (jobType === "compile") {
      await setupCompileJob();
    } else if (jobType === "updateGame") {
      await setupUpdateGameJob();
    } else if (jobType === "installMod") {
      await setupModInstallation();
    } else if (jobType === "installModExternal") {
      await setupModInstallationExternal();
    } else if (jobType === "decompileMod") {
      await setupDecompileModJob();
    } else if (jobType === "compileMod") {
      await setupCompileModJob();
    }
  });
</script>

<div class="flex flex-col justify-content">
  <Progress />
  <LogViewer />
</div>
{#if $progressTracker.overallStatus === "success" && !proceedAfterSuccessfulOperation}
  <div class="flex flex-col justify-end items-end mt-auto">
    <div class="flex flex-row gap-2">
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => dispatch("jobFinished")}
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
