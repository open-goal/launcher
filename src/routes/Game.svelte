<script>
  import { fromRoute, getInternalName, SupportedGame } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import GameControls from "../components/games/GameControls.svelte";
  import GameSetup from "../components/games/setup/GameSetup.svelte";
  import { onMount } from "svelte";
  import { Spinner } from "flowbite-svelte";
  import {
    getActiveVersion,
    getActiveVersionFolder,
    getInstalledVersion,
    getInstalledVersionFolder,
    isGameInstalled,
  } from "$lib/rpc/config";
  import GameJob from "../components/games/job/GameJob.svelte";
  import GameUpdate from "../components/games/setup/GameUpdate.svelte";

  const params = useParams();
  let activeGame = SupportedGame.Jak1;
  let componentLoaded = false;

  let gameInstalled = false;
  let gameJobToRun = undefined;

  let installedVersion;
  let installedVersionFolder;
  let activeVersion;
  let activeVersionFolder;

  let versionMismatchDetected = false;

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
    if (gameInstalled) {
      installedVersion = await getInstalledVersion(getInternalName(activeGame));
      installedVersionFolder = await getInstalledVersionFolder(
        getInternalName(activeGame)
      );
      activeVersion = await getActiveVersion(getInternalName(activeGame));
      activeVersionFolder = await getActiveVersionFolder(
        getInternalName(activeGame)
      );
      if (
        installedVersion !== activeVersion ||
        installedVersionFolder !== activeVersionFolder
      ) {
        versionMismatchDetected = true;
      }
    }

    componentLoaded = true;
  });

  async function updateGameState(evt) {
    gameInstalled = await isGameInstalled(getInternalName(activeGame));
    // TODO - check data dir?
  }

  async function runGameJob(event) {
    gameJobToRun = event.detail.type;
  }

  async function gameJobFinished() {
    gameJobToRun = undefined;
    versionMismatchDetected = false;
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
    {:else if gameJobToRun !== undefined}
      <GameJob
        {activeGame}
        jobType={gameJobToRun}
        on:jobFinished={gameJobFinished}
      />
    {:else if versionMismatchDetected}
      <GameUpdate {activeGame} {installedVersion} {installedVersionFolder} {activeVersion} {activeVersionFolder} on:job={runGameJob} />
    {:else}
      <GameControls
        {activeGame}
        on:change={updateGameState}
        on:job={runGameJob}
      />
    {/if}
  </div>
</div>
