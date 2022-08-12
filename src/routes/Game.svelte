<script>
  import { fade } from "svelte/transition";
  import { launcherConfig } from "$lib/config";
  import { fromRoute, getGameTitle, SupportedGame } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import GameContent from "../components/games/GameControls.svelte";
  import GameSetup from "../components/games/setup/GameSetup.svelte";
  import { onMount } from "svelte";
  import { gameNeedsReinstall } from "$lib/stores/AppStore";
  import {
    copyDataDirectory,
    isDataDirectoryUpToDate,
  } from "$lib/utils/data-files";

  const params = useParams();
  let activeGame = SupportedGame.Jak1;
  let componentLoaded = false;

  let isGameInstalled = false;

  let dataDirUpToDate = false;
  let updatingDataDir = false;
  let errorText = "";

  onMount(async () => {
    if (
      $params["game_name"] !== undefined &&
      $params["game_name"] !== null &&
      $params["game_name"] !== ""
    ) {
      activeGame = fromRoute($params["game_name"]);
    }

    isGameInstalled = await launcherConfig.getInstallStatus(activeGame);

    // Do some checks before the user can play the game
    // First, let's see if their data directory needs updating
    // we do it here so we can get the user's consent
    dataDirUpToDate = await isDataDirectoryUpToDate();
    // If it's up to date we'll do the second check now, does their game need to be re-compiled?
    if (dataDirUpToDate) {
      if (await launcherConfig.shouldUpdateGameInstall(activeGame)) {
        // await recompileGame(activeGame);
        gameNeedsReinstall.update(() => true);
      }
    }
    componentLoaded = true;
  });

  async function syncDataDirectory() {
    updatingDataDir = true;
    errorText = "";
    try {
      await copyDataDirectory();
      // Now that the directory is up to date, let's see if they need to reinstall the game
      if (await launcherConfig.shouldUpdateGameInstall(activeGame)) {
        gameNeedsReinstall.update(() => true);
      }
    } catch (err) {
      errorText = `Error encountered when syncing data files - ${err}`;
    }
    updatingDataDir = false;
  }

  async function updateGameState(evt) {
    isGameInstalled = await launcherConfig.getInstallStatus(activeGame);
    dataDirUpToDate = await isDataDirectoryUpToDate();
  }
</script>

{#if componentLoaded}
  <div class="flex-center" in:fade>
    <h1 class="text-shadow">
      {getGameTitle(activeGame)}
    </h1>
    {#if isGameInstalled && !$gameNeedsReinstall}
      {#if !dataDirUpToDate}
        <p>Local data files must be synced up in-order to proceed</p>
        <p>This may overwrite any modifications to the game's source code</p>
        <p>Save files and settings will not be modified</p>
        {#if !updatingDataDir}
          <button class="btn" on:click={syncDataDirectory}>
            Sync Data Files
          </button>
        {/if}
        {#if errorText != ""}
          {errorText}
        {/if}
      {:else}
        <GameContent {activeGame} on:change={updateGameState} />
      {/if}
    {:else}
      {#if $gameNeedsReinstall}
        <p>Game installed with a previous version of OpenGOAL</p>
        <p>The game must be updated before you can proceed</p>
        <p>Save files and settings will not be modified</p>
      {/if}
      <GameSetup {activeGame} on:change={updateGameState} />
    {/if}
  </div>
{:else}
  <!-- TODO - component library - spinner -->
{/if}

<style>
  .text-shadow {
    text-shadow: -1px -1px 0 #000, 1px -1px 0 #000, -1px 1px 0 #000,
      1px 1px 0 #000;
  }
</style>
