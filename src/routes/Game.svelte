<script>
  import { launcherConfig } from "$lib/config";
  import {
    fromRoute,
    getGameTitle,
    getInternalName,
    SupportedGame,
  } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import GameControls from "../components/games/GameControls.svelte";
  import GameSetup from "../components/games/setup/GameSetup.svelte";
  import { onMount } from "svelte";
  import { gameNeedsReinstall, isInstalling } from "$lib/stores/AppStore";
  import { isDataDirectoryUpToDate } from "$lib/utils/data-files";
  import Outdated from "../components/games/setup/Outdated.svelte";
  import Reinstall from "../components/games/setup/Reinstall.svelte";
  import { Spinner } from "flowbite-svelte";
  import { isGameInstalled } from "$lib/rpc/config";

  const params = useParams();
  let activeGame = SupportedGame.Jak1;
  let componentLoaded = false;

  let gameInstalled = false;

  let dataDirUpToDate = false;
  let updatingDataDir = false;

  onMount(async () => {
    // Figure out what game we are displaying
    if (
      $params["game_name"] !== undefined &&
      $params["game_name"] !== null &&
      $params["game_name"] !== ""
    ) {
      activeGame = fromRoute($params["game_name"]);
    } else {
      activeGame = SupportedGame.Jak1;
    }

    // First obvious thing to check -- is the game installed at all
    gameInstalled = await isGameInstalled(getInternalName(activeGame));

    // Next step, check if there is a version mismatch
    // - they installed the game before with a different version than what they currently have selected
    // - prompt them to either reinstall OR go and select their previous version

    componentLoaded = true;
  });

  async function updateGameState(evt) {
    gameInstalled = await launcherConfig.getInstallStatus(activeGame);
    dataDirUpToDate = await isDataDirectoryUpToDate();
  }
</script>

<div class="ml-20">
  <!-- TODO - the static height here is kinda a hack, it's because the
    header and the rest of the layout aren't within a shared container -->
  <div class="flex flex-col h-[544px] p-5">
    {#if !componentLoaded}
      <div class="flex flex-col h-full justify-center items-center">
        <Spinner color="yellow" size={"12"} />
      </div>
    {:else if !gameInstalled}
      <GameSetup {activeGame} on:change={updateGameState} />
    {:else}
      <GameControls {activeGame} on:change={updateGameState} />
    {/if}
  </div>
</div>
