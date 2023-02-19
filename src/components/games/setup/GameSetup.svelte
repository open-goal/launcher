<script lang="ts">
  // components
  import Progress from "./Progress.svelte";
  // constants
  import { getGameTitle, type SupportedGame } from "$lib/constants";
  import LogViewer from "./LogViewer.svelte";
  import Requirements from "./Requirements.svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { Button } from "flowbite-svelte";
  import {
    extractAndValidateISO,
    runCompiler,
    runDecompiler,
  } from "$lib/rpc/extractor";
  import { folderPrompt, isoPrompt } from "$lib/utils/file";
  import { finalizeInstallation, isOpenGLRequirementMet } from "$lib/rpc/config";
  import { progressTracker } from "$lib/stores/ProgressStore";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();

  let requirementsMet = false;
  let installing = false;

  onMount(async () => {
    // TODO - properly check requirements
    requirementsMet = true;
  });

  async function install(viaFolder: boolean) {
    let sourcePath = "";
    if (viaFolder) {
      sourcePath = await folderPrompt("Select a folder with your ISO's data extracted");
    } else {
      sourcePath = await isoPrompt();
    }
    if (sourcePath !== undefined) {
      installing = true;
      // Initialize the installation steps for this particular config
      progressTracker.init([
        {
          status: 'queued',
          label: 'Extract and Verify'
        },
        {
          status: 'queued',
          label: 'Decompile'
        },
        {
          status: 'queued',
          label: 'Compile'
        },
        {
          status: 'queued',
          label: 'Done'
        }
      ]);
      // TODO - handle errors
      progressTracker.start();
      await extractAndValidateISO(sourcePath, "jak1");
      progressTracker.proceed();
      await runDecompiler(sourcePath, "jak1");
      progressTracker.proceed();
      await runCompiler(sourcePath, "jak1");
      progressTracker.proceed();
      await finalizeInstallation("jak1");
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
    {#if $progressTracker.logs}
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
