<script lang="ts">
  import GameControls from "../components/games/GameControls.svelte";
  import GameSetup from "../components/job/GameSetup.svelte";
  import { Alert } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import {
    doesActiveToolingVersionSupportGame,
    getInstalledVersion,
    isGameInstalled,
  } from "$lib/rpc/config";
  import GameUpdate from "../components/job/GameUpdate.svelte";
  import { ensureActiveVersionStillExists } from "$lib/rpc/versions";
  import GameToolsNotSet from "../components/games/GameToolsNotSet.svelte";
  import GameNotSupportedByTooling from "../components/games/GameNotSupportedByTooling.svelte";
  import { type } from "@tauri-apps/plugin-os";
  import GameControlsMod from "../components/games/GameControlsMod.svelte";
  import GameInProgress from "../components/games/GameInProgress.svelte";
  import { route } from "../router";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame.ts";
  import { versionState } from "../state/VersionState.svelte";
  import { systemInfoState } from "../state/SystemInfoState.svelte";
  import GameBetaAlert from "../components/games/GameBetaAlert.svelte";
  import VCCWarning from "../components/games/VCCWarning.svelte";
  import { onMount } from "svelte";

  const gameParam = $derived(route.params.game_name);
  let activeGame: SupportedGame | undefined = $state(undefined);
  let modName: string | undefined = $derived(route.params.mod_name);
  let modSource: string | undefined = $derived(route.params.source_name);

  const showVccWarning = $derived(
    type() == "windows" && !systemInfoState.isMinVCCRuntimeInstalled,
  );

  let gameInstalled: boolean | undefined = $state(undefined);
  let installedVersion: String | undefined = $state(undefined);
  let versionMismatchDetected = $state(false);
  let gameSupportedByTooling = $state(false);

  $effect(() => {
    const activeGameFromParam = toSupportedGame(gameParam);
    if (activeGameFromParam) {
      activeGame = activeGameFromParam;
      loadGameInfo(activeGame);
    }
  });

  async function loadGameInfo(activeGame?: SupportedGame) {
    if (!activeGame) {
      return;
    }
    // First off, check that they've downloaded and have a jak-project release set
    const activeVersionExists = await ensureActiveVersionStillExists();

    if (activeVersionExists) {
      gameSupportedByTooling =
        await doesActiveToolingVersionSupportGame(activeGame);
      // verify the game is installed
      gameInstalled = await isGameInstalled(activeGame);

      // verify selected version is installed tooling version
      // - prompt them to either reinstall OR go and select their previous version
      if (gameInstalled) {
        installedVersion = await getInstalledVersion(activeGame);
        versionMismatchDetected =
          installedVersion !== versionState.activeToolingVersion;
      }
    }
  }

  async function gameInstallStateChanged(event: any) {
    if (!activeGame) {
      return;
    }
    gameInstalled = await isGameInstalled(activeGame);
  }
</script>

{#if activeGame && gameInstalled !== undefined}
  <div class="flex flex-col h-full p-5">
    {#if !versionState.activeToolingVersion}
      <GameToolsNotSet />
    {:else if activeGame == "jak3" || activeGame == "jakx"}
      <!-- TODO: remove this else if arm for jak3 support -->
      <GameInProgress />
    {:else if !gameSupportedByTooling}
      <GameNotSupportedByTooling />
    {:else if !gameInstalled}
      <GameSetup {activeGame} />
    {:else if versionMismatchDetected}
      <GameUpdate {installedVersion} />
    {:else}
      {#if showVccWarning}
        <VCCWarning></VCCWarning>
      {/if}
      <!-- Jak 2 BETA warning -->
      {#if activeGame === "jak2"}
        <GameBetaAlert></GameBetaAlert>
      {/if}
      {#if modName && modSource}
        <GameControlsMod {activeGame} {modName} {modSource} />
      {:else}
        <GameControls
          {activeGame}
          on:gameInstallStateChanged={gameInstallStateChanged}
        />
      {/if}
    {/if}
  </div>
{/if}
