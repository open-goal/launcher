<script lang="ts">
  import { fromRoute, getInternalName, SupportedGame } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import GameModControls from "./GameModControls.svelte";
  import GameSetup from "../../components/games/setup/GameSetup.svelte";
  import { onMount } from "svelte";
  import { Spinner, Button } from "flowbite-svelte";
  import Icon from "@iconify/svelte";

  import {
    getInstalledVersion,
    getInstalledVersionFolder,
    isGameInstalled,
  } from "$lib/rpc/config";
  import GameJob from "../../components/games/job/GameJob.svelte";
  import GameUpdate from "../../components/games/setup/GameUpdate.svelte";
  import {
    ensureActiveVersionStillExists,
    getActiveVersion,
    getActiveVersionFolder,
  } from "$lib/rpc/versions";
  import GameToolsNotSet from "../../components/games/GameToolsNotSet.svelte";
  import { VersionStore } from "$lib/stores/VersionStore";

  export let game_name: string;
  export let mod_id: string;
  export let mod_version: string;

  let activeGame: SupportedGame;
  let componentLoaded = false;

  let gameInstalled = false;
  let gameJobToRun = undefined;

  let installedVersion;
  let installedVersionFolder;

  let versionMismatchDetected = false;

  function fixedDecodeURIComponent(str) {
    return decodeURIComponent(str.replace(/-DOT-/g, '.'))
  }

  onMount(async () => {
    activeGame = fromRoute(game_name);

    // TODO: do this right
    gameInstalled = true;

    console.log(mod_version, fixedDecodeURIComponent(mod_version))

    // console.log(activeGame, activeMod, activeVersion);

    // // First off, check that they've downloaded and have a jak-project release set
    // const activeVersionExists = await ensureActiveVersionStillExists();
    // $VersionStore.activeVersionType = await getActiveVersionFolder();
    // $VersionStore.activeVersionName = await getActiveVersion();

    // if (activeVersionExists) {
    //   // First obvious thing to check -- is the game installed at all
    //   gameInstalled = await isGameInstalled(getInternalName(activeGame));

    //   // Next step, check if there is a version mismatch
    //   // - they installed the game before with a different version than what they currently have selected
    //   // - prompt them to either reinstall OR go and select their previous version
    //   if (gameInstalled) {
    //     installedVersion = await getInstalledVersion(
    //       getInternalName(activeGame)
    //     );
    //     installedVersionFolder = await getInstalledVersionFolder(
    //       getInternalName(activeGame)
    //     );
    //     if (
    //       installedVersion !== $VersionStore.activeVersionName ||
    //       installedVersionFolder !== $VersionStore.activeVersionType
    //     ) {
    //       versionMismatchDetected = true;
    //     }
    //   }
    // }

    componentLoaded = true;
  });

  async function updateGameState(evt) {
    gameInstalled = await isGameInstalled(game_name);
  }

  async function runGameJob(event) {
    gameJobToRun = event.detail.type;
  }

  async function gameJobFinished() {
    gameJobToRun = undefined;
    versionMismatchDetected = false;
  }
</script>

<div class="flex flex-col h-full p-5">
  {#if !componentLoaded}
    <div class="flex flex-col h-full justify-center items-center">
      <Spinner color="yellow" size={"12"} />
    </div>
  {:else if $VersionStore.activeVersionName === null || $VersionStore.activeVersionType === null}
    <GameToolsNotSet />
  {:else if !gameInstalled}
    <GameSetup {activeGame} on:change={updateGameState} />
  {:else if gameJobToRun !== undefined}
    <GameJob
      {activeGame}
      jobType={gameJobToRun}
      on:jobFinished={gameJobFinished}
    />
  {:else if versionMismatchDetected}
    <GameUpdate
      {activeGame}
      {installedVersion}
      {installedVersionFolder}
      on:job={runGameJob}
    />
  {:else}
    <div class="flex flex-row gap-2">
      <Button
        btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
        on:click={() => history.back()}
      >
        <Icon
          icon="ic:baseline-arrow-back"
          color="#ffffff"
          width="20"
          height="20"
        />
      </Button>
    </div>
    <GameModControls
      game_name={game_name}
      mod_id={mod_id}
      mod_version={fixedDecodeURIComponent(mod_version)}
      on:change={updateGameState}
      on:job={runGameJob}
    />
  {/if}
</div>
