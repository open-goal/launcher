<script lang="ts">
  import { ProcessLogs, InstallationProgress } from "$lib/stores/AppStore";
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
  import { isoPrompt } from "$lib/utils/file";
  import { finalizeInstallation } from "$lib/rpc/config";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();

  let requirementsMet = false;
  let installing = false;

  onMount(async () => {
    // NOTE - potentially has problems if the user changes hardware
    // TODO
    // if (!(await launcherConfig.areRequirementsMet())) {
    //   await checkRequirements();
    // }
    requirementsMet = true; //await launcherConfig.areRequirementsMet();
  });

  async function installViaISO() {
    const isoPath = await isoPrompt();
    if (isoPath !== undefined) {
      installing = true;
      // TODO - reset installation steps
      ProcessLogs.update(() => "");
      // TODO - handle errors and such
      // TODO - get rid of hard-coding
      // TODO - methods!
      $InstallationProgress.currentStep = 0;
      $InstallationProgress.steps[0].status = "pending";
      await extractAndValidateISO(isoPath, "jak1");
      $InstallationProgress.steps[0].status = "success";
      $InstallationProgress.currentStep = 1;
      $InstallationProgress.steps[1].status = "pending";
      await runDecompiler(isoPath, "jak1");
      $InstallationProgress.steps[1].status = "success";
      $InstallationProgress.currentStep = 2;
      $InstallationProgress.steps[2].status = "pending";
      await runCompiler(isoPath, "jak1");
      $InstallationProgress.steps[2].status = "success";
      $InstallationProgress.currentStep = 3;
      $InstallationProgress.steps[3].status = "pending";
      await finalizeInstallation("jak1");
      $InstallationProgress.steps[3].status = "success";
    }
  }

  async function dispatchSetupEvent() {
    dispatch("change");
  }
</script>

<!-- TODO - allow passing a folder path -->

{#if !requirementsMet}
  <Requirements />
{:else if installing}
  <div class="flex flex-col justify-content">
    <Progress />
    {#if $ProcessLogs}
      <LogViewer />
    {/if}
  </div>
  {#if $InstallationProgress.currentStep === 3 && $InstallationProgress.steps[3].status === "success"}
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
        on:click={async () => await installViaISO()}>Install via ISO</Button
      ><Button
        btnClass="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        >Install via Folder</Button
      >
    </div>
  </div>
{/if}
