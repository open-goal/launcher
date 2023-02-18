<script type="ts">
  import { launcherConfig } from "$lib/config";
  import {
    gameNeedsReinstall,
    isInstalling,
    ProcessLogs,
  } from "$lib/stores/AppStore";
  import { checkRequirements } from "$lib/setup/setup";
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
    if (!(await launcherConfig.areRequirementsMet())) {
      await checkRequirements();
    }
    requirementsMet = await launcherConfig.areRequirementsMet();
  });

  async function installViaISO() {
    const isoPath = await isoPrompt();
    if (isoPath !== undefined) {
      installing = true;
      ProcessLogs.update(() => "");
      // TODO - handle errors and such
      // TODO - get rid of hard-coding
      await extractAndValidateISO(isoPath, "jak1");
      await runDecompiler(isoPath, "jak1");
      await runCompiler(isoPath, "jak1");

      await finalizeInstallation("jak1");
      // if (success) {
      //   dispatch("change");
      // }
    }
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
