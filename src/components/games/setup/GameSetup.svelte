<script type="ts">
  import { launcherConfig } from "$lib/config";
  import { gameNeedsReinstall, isInstalling, ProcessLogs } from "$lib/stores/AppStore";
  import { checkRequirements, fullInstallation, recompileGame } from "$lib/setup/setup";
  // components
  import Progress from "./Progress.svelte";
  // constants
  import type { SupportedGame } from "$lib/constants";
  import LogViewer from "./LogViewer.svelte";
  import Requirements from "./Requirements.svelte";
  import { createEventDispatcher, onMount } from "svelte";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();

  let componentLoaded = false;
  let requirementsMet = false;

  onMount(async () => {
    // NOTE - potentially has problems if the user changes hardware
    if (!(await launcherConfig.areRequirementsMet())) {
      await checkRequirements();
    }
    requirementsMet = await launcherConfig.areRequirementsMet();
    componentLoaded = true;
  });

  async function fullInstall() {
    const installationSuccess = await fullInstallation(activeGame);
    if (installationSuccess) {
      dispatch('change');
    }
  }
</script>

{#if componentLoaded}
  <div class="content">
    <div style="text-align:center">
      {#if !requirementsMet}
        <Requirements />
      {:else}
        {#if !$isInstalling}
          {#if $gameNeedsReinstall}
            <button
              class="btn"
              on:click={async () => await recompileGame(activeGame)}
            >
              Update Install
            </button>
          {:else}
            <button
              class="btn"
              on:click={fullInstall}
            >
              Setup
            </button>
          {/if}
        {:else}
          <Progress />
        {/if}
        {#if $ProcessLogs}
        <LogViewer />
        {/if}
      {/if}
    </div>
  </div>
{:else}
  <!-- TODO - component library - spinner -->
{/if}
