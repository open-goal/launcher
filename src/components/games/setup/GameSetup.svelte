<script type="ts">
  import { launcherConfig } from "$lib/config";
  import {
    gameNeedsReinstall,
    isInstalling,
    ProcessLogs,
  } from "$lib/stores/AppStore";
  import {
    checkRequirements,
    fullInstallation,
    recompileGame,
  } from "$lib/setup/setup";
  // components
  import Progress from "./Progress.svelte";
  // constants
  import { getGameTitle, type SupportedGame } from "$lib/constants";
  import LogViewer from "./LogViewer.svelte";
  import Requirements from "./Requirements.svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { Button } from "flowbite-svelte";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();

  let requirementsMet = false;

  onMount(async () => {
    // NOTE - potentially has problems if the user changes hardware
    if (!(await launcherConfig.areRequirementsMet())) {
      await checkRequirements();
    }
    requirementsMet = await launcherConfig.areRequirementsMet();
  });

  async function fullInstall() {
    const success = await fullInstallation(activeGame);
    if (success) {
      dispatch("change");
    }
  }

  async function updateGame() {
    const success = await recompileGame(activeGame);
    if (success) {
      dispatch("change");
    }
  }
</script>

{#if !requirementsMet}
  <Requirements />
{:else if !$isInstalling}
  <div class="flex flex-col justify-end items-end h-5/6 pr-7">
    <h1 class="text-3xl pb-2">
      {getGameTitle(activeGame)}
    </h1>
    {#if $gameNeedsReinstall}
      <Button
        class="!rounded-none !w-56 !bg-[#222222] text-xl"
        on:click={updateGame}>Update Install</Button
      >
    {:else}
      <Button
        class="!rounded-none !w-56 !bg-[#222222] text-xl"
        on:click={fullInstall}>Install</Button
      >
    {/if}
  </div>
{:else}
  <div class="flex flex-col justify-center items-center ml-20 p-8">
    <Progress />
    {#if $ProcessLogs}
      <LogViewer />
    {/if}
  </div>
{/if}
