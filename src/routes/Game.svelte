<script lang="ts">
  import GameControls from "../components/games/GameControls.svelte";
  import GameSetup from "../components/games/setup/GameSetup.svelte";
  import { Alert } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import {
    doesActiveToolingVersionSupportGame,
    getInstalledVersion,
    isGameInstalled,
  } from "$lib/rpc/config";
  import GameJob from "../components/games/job/GameJob.svelte";
  import GameUpdate from "../components/games/setup/GameUpdate.svelte";
  import {
    ensureActiveVersionStillExists,
    getActiveVersion,
  } from "$lib/rpc/versions";
  import GameToolsNotSet from "../components/games/GameToolsNotSet.svelte";
  import GameNotSupportedByTooling from "../components/games/GameNotSupportedByTooling.svelte";
  import { isMinVCCRuntime } from "$lib/stores/VersionStore";
  import type { Job } from "$lib/utils/jobs";
  import { type } from "@tauri-apps/plugin-os";
  import GameControlsMod from "../components/games/GameControlsMod.svelte";
  import GameInProgress from "../components/games/GameInProgress.svelte";
  import { route } from "../router";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame.ts";
  import { versionState } from "../state/VersionState.svelte";

  const gameParam = $derived(route.params.game_name);
  let activeGame: SupportedGame | undefined = $state(undefined);
  let modName: string | undefined = $derived(route.params.mod_name);
  let modSource: string | undefined = $derived(route.params.source_name);

  const showVccWarning = $derived(type() == "windows" && !$isMinVCCRuntime);

  let modVersionToInstall: string = $state("");
  let modDownloadUrlToInstall: string = $state("");
  let gameInstalled = $state(false);
  let gameJobToRun: Job | undefined = $state(undefined);
  let installedVersion: String | undefined = $state(undefined);
  let versionMismatchDetected = $state(false);
  let gameSupportedByTooling = $state(false);

  $effect(() => {
    const activeGameFromParam = toSupportedGame(gameParam);
    if (activeGameFromParam) {
      activeGame = activeGameFromParam;
    }
  });

  $effect(() => {
    loadGameInfo(activeGame);
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

  async function updateGameState(event: any) {
    if (!activeGame) {
      return;
    }
    gameInstalled = await isGameInstalled(activeGame);
  }

  async function runGameJob(event: any) {
    gameJobToRun = event.detail.type;
    if (gameJobToRun === "installModExternal") {
      modDownloadUrlToInstall = event.detail.modDownloadUrl;
      modVersionToInstall = event.detail.modVersion;
    }
  }

  async function gameJobFinished() {
    gameJobToRun = undefined;
    versionMismatchDetected = false;
  }
</script>

<!-- TODO - stop with the flashes of rendering, don't render until we know what needs to be rendered -->
{#if activeGame}
  <div class="flex flex-col h-full p-5">
    {#if !versionState.activeToolingVersion}
      <GameToolsNotSet />
    {:else if activeGame == "jak3" || activeGame == "jakx"}
      <!-- TODO: remove this else if arm for jak3 support -->
      <GameInProgress />
    {:else if !gameSupportedByTooling}
      <GameNotSupportedByTooling />
    {:else if !gameInstalled}
      <GameSetup {activeGame} on:change={updateGameState} />
    {:else if gameJobToRun !== undefined}
      <GameJob
        {activeGame}
        jobType={gameJobToRun}
        modSourceName={modSource}
        modDownloadUrl={modDownloadUrlToInstall}
        modVersion={modVersionToInstall}
        {modName}
        on:jobFinished={gameJobFinished}
      />
    {:else if versionMismatchDetected}
      <GameUpdate {installedVersion} on:job={runGameJob} />
    {:else}
      {#if showVccWarning}
        <Alert rounded={false} class="border-t-4 text-red-400">
          <span class="font-bold"
            >{$_("gameControls_warning_vccVersion_headerA")}</span
          >
          <em>{$_("gameControls_warning_vccVersion_headerB")}</em>
          <br />
          <ul>
            <li>
              <a
                class="font-bold text-blue-500"
                target="_blank"
                rel="noreferrer"
                href="https://aka.ms/vs/17/release/vc_redist.x64.exe"
                >{$_(
                  "requirements_windows_vccRuntimeExplanation_downloadLink",
                )}</a
              >
            </li>
          </ul>
        </Alert>
      {/if}

      <!-- Jak 2 BETA warning -->
      {#if activeGame === "jak2"}
        <Alert rounded={false} class="border-t-4 text-red-400">
          <span class="font-bold">{$_("gameControls_beta_headerA")}</span>
          <em>{$_("gameControls_beta_headerB")}</em>
          <br />
          <ul>
            <li>
              {$_("gameControls_beta_issueTracker_linkPreText")}
              <a
                class="text-blue-400"
                href="https://github.com/open-goal/jak-project/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3Ajak2"
                target="_blank"
                rel="noopener noreferrer"
                >{$_("gameControls_beta_issueTracker_linkText")}</a
              >
            </li>
            <li>
              {$_("gameControls_beta_bugReport_linkPreText")}
              <a
                class="text-blue-400"
                href="https://github.com/open-goal/jak-project/issues/new?template=jak2-bug-report.yml"
                target="_blank"
                rel="noopener noreferrer"
                >{$_("gameControls_beta_bugReport_linkText")}</a
              >
            </li>
          </ul>
        </Alert>
      {/if}

      <!-- Jak 3 BETA warning -->
      <!-- NOTE - dead branch, disabling until relevant to avoid ts error -->
      <!-- {#if activeGame === "jak3"}
      <Alert rounded={false} class="border-t-4 text-red-400">
        <span class="font-bold">{$_("gameControls_beta_headerA_jak3")}</span>
        <em>{$_("gameControls_beta_headerB")}</em>
        <br />
        <ul>
          <li>
            {$_("gameControls_beta_issueTracker_linkPreText")}
            <a
              class="text-blue-400"
              href="https://github.com/open-goal/jak-project/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3Ajak3"
              target="_blank"
              rel="noopener noreferrer"
              >{$_("gameControls_beta_issueTracker_linkText")}</a
            >
          </li>
          <li>
            {$_("gameControls_beta_bugReport_linkPreText")}
            <a
              class="text-blue-400"
              href="https://github.com/open-goal/jak-project/issues/new?template=jak3-bug-report.yml"
              target="_blank"
              rel="noopener noreferrer"
              >{$_("gameControls_beta_bugReport_linkText")}</a
            >
          </li>
        </ul>
      </Alert>
    {/if} -->
      {#if modName && modSource}
        <GameControlsMod
          {activeGame}
          {modName}
          {modSource}
          on:change={updateGameState}
          on:job={runGameJob}
        />
      {:else}
        <GameControls
          {activeGame}
          on:change={updateGameState}
          on:job={runGameJob}
        />
      {/if}
    {/if}
  </div>
{/if}
