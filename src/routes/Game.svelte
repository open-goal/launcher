<script>
  import { launcherConfig } from "$lib/config";
  import { fromRoute, getGameTitle, SupportedGame } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import GameContent from "../components/games/GameControls.svelte";
  import GameSetup from "../components/games/setup/GameSetup.svelte";
  import { onMount } from "svelte";
  import { gameNeedsReinstall } from "$lib/stores/AppStore";
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
      <div class="flex flex-col justify-end items-end h-5/6 pr-7">
        <h1 class="text-4xl pb-2 drop-shadow-text">
          {getGameTitle(activeGame)}
        </h1>
        <GameContent {activeGame} on:change={updateGameState} />
      </div>
    {/if}
  {:else}
    <!-- TODO: THIS BLOCK HERE KINDA SUCKS AND IDK HOW TO FIX IT -->
    {#if $gameNeedsReinstall}
      <Reinstall />
    {/if}
    <GameSetup {activeGame} on:change={updateGameState} />
  {/if}
{:else}
  <div class="ml-20">
    <Spinner />
  </div>
{/if}
