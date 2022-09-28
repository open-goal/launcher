<script>
  import { launcherConfig } from "$lib/config";
  import { fromRoute, getGameTitle, SupportedGame } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import GameContent from "../components/games/GameControls.svelte";
  import GameSetup from "../components/games/setup/GameSetup.svelte";
  import { onMount } from "svelte";
  import { gameNeedsReinstall, isInstalling } from "$lib/stores/AppStore";
  import { isDataDirectoryUpToDate } from "$lib/utils/data-files";
  import Outdated from "../components/games/setup/Outdated.svelte";
  import Reinstall from "../components/games/setup/Reinstall.svelte";
  import { Spinner } from "flowbite-svelte";

  const params = useParams();
  let activeGame = SupportedGame.Jak1;
  let componentLoaded = false;

  let isGameInstalled = false;

  let dataDirUpToDate = false;
  let updatingDataDir = false;

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
        gameNeedsReinstall.update(() => true);
      }
    }
    componentLoaded = true;
  });

  async function updateGameState(evt) {
    isGameInstalled = await launcherConfig.getInstallStatus(activeGame);
    dataDirUpToDate = await isDataDirectoryUpToDate();
  }
</script>

{#if componentLoaded}
  {#if isGameInstalled && !$gameNeedsReinstall}
    {#if !dataDirUpToDate}
      <Outdated {updatingDataDir} {activeGame} />
    {:else}
      <!-- NOTE: 560px height is 600px (total) - 40px (header) -->
      <div class="flex flex-col justify-end items-end h-[560px] pb-7 pr-7">
        <h1 class="text-4xl pb-2 drop-shadow-text">
          {getGameTitle(activeGame)}
        </h1>
        <GameContent {activeGame} on:change={updateGameState} />
      </div>
    {/if}
  {:else}
    <div class="flex flex-col h-[560px] ml-20 p-7">
      {#if $gameNeedsReinstall && !$isInstalling}
        <Reinstall />
      {/if}
      <GameSetup {activeGame} on:change={updateGameState} />
    </div>
  {/if}
{:else}
  <div class="ml-20">
    <Spinner />
  </div>
{/if}
