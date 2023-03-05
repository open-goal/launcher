<script lang="ts">
  // components
  import Progress from "./Progress.svelte";
  // constants
  import {
    getGameTitle,
    getInternalName,
    type SupportedGame,
  } from "$lib/constants";
  import LogViewer from "./LogViewer.svelte";
  import Requirements from "./Requirements.svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { Alert, Button } from "flowbite-svelte";
  import {
    extractAndValidateISO,
    runCompiler,
    runDecompiler,
  } from "$lib/rpc/binaries";
  import { folderPrompt, isoPrompt } from "$lib/utils/file";
  import {
    finalizeInstallation,
    isAVXRequirementMet,
    isOpenGLRequirementMet,
    setOpenGLRequirementMet,
  } from "$lib/rpc/config";
  import { progressTracker } from "$lib/stores/ProgressStore";
  import { generateSupportPackage } from "$lib/rpc/support";
  import { isOpenGLVersionSupported } from "$lib/sidecars/glewinfo";
  import { listen } from "@tauri-apps/api/event";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();

  let requirementsMet = true;
  let installing = false;
  let installationError = undefined;

  onMount(async () => {
    // Check requirements
    const isAvxMet = await isAVXRequirementMet();
    let isOpenGLMet = await isOpenGLRequirementMet();
    if (isOpenGLMet === null) {
      isOpenGLMet = await isOpenGLVersionSupported("4.3");
      await setOpenGLRequirementMet(isOpenGLMet);
    }
    requirementsMet = isAvxMet && isOpenGLMet;

    const unlistenLogListener = await listen("updateJobLogs", async (event) => {
      progressTracker.updateLogs(event.payload["stdout"]);
    });
  });

  async function install(viaFolder: boolean) {
    let sourcePath = "";
    if (viaFolder) {
      sourcePath = await folderPrompt(
        "Select a folder with your ISO's data extracted"
      );
    } else {
      sourcePath = await isoPrompt();
    }
    if (sourcePath !== undefined) {
      installing = true;
      installationError = undefined;
      // Initialize the installation steps for this particular config
      progressTracker.init([
        {
          status: "queued",
          label: "Extract and Verify",
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
      // TODO - make this cleaner
      progressTracker.start();
      let resp = await extractAndValidateISO(
        sourcePath,
        getInternalName(activeGame)
      );
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      resp = await runDecompiler(sourcePath, getInternalName(activeGame));
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      resp = await runCompiler(sourcePath, getInternalName(activeGame));
      if (!resp.success) {
        progressTracker.halt();
        installationError = resp.msg;
        return;
      }
      progressTracker.proceed();
      // TODO - technically should handle the error here too
      await finalizeInstallation(getInternalName(activeGame));
      progressTracker.proceed();
    }
  }

  async function dispatchSetupEvent() {
    dispatch("change");
  }
</script>

{#if !requirementsMet}
  <Requirements />
{:else if installing}
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
          on:click={async () => await dispatchSetupEvent()}>Continue</Button
        >
      </div>
    </div>
  {:else if true || $progressTracker.overallStatus === "failed"}
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
{:else}
  <div class="flex flex-col justify-end items-end mt-auto">
    <h1
      class="tracking-tighter text-2xl font-bold pb-3 text-orange-500 text-outline"
    >
      {getGameTitle(activeGame)}
    </h1>
    <div class="flex flex-row gap-2">
      <Button
        btnClass="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => await install(false)}>Install via ISO</Button
      >
      <!-- TODO - disabled for now, needs fixes in the extractor -->
      <!-- <Button
        btnClass="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => await install(true)}
        >Install via Folder</Button
      > -->
    </div>
  </div>
{/if}
