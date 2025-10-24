<script lang="ts">
  import { useParams } from "svelte-navigator";
  import GameControls from "../components/games/GameControls.svelte";
  import GameSetup from "../components/games/setup/GameSetup.svelte";
  import { onMount } from "svelte";
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
  import { isMinVCCRuntime, VersionStore } from "$lib/stores/VersionStore";
  import type { Job } from "$lib/utils/jobs";
  import { type } from "@tauri-apps/plugin-os";
  import { getModSourcesData } from "$lib/rpc/cache";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import GameControlsMod from "../components/games/GameControlsMod.svelte";
  import GameInProgress from "../components/games/GameInProgress.svelte";
  import { activeGame, modInfoStore } from "$lib/stores/AppStore";

  const params = useParams();
  $: ($params, loadGameInfo());

  let modName: string | undefined = undefined;
  let modSource: string | undefined = undefined;
  let modVersionToInstall: string = "";
  let modDownloadUrlToInstall: string = "";

  modName = $params["mod_name"];
  modSource = $params["source_url"];

  let gameInstalled = false;
  let gameJobToRun: Job | undefined = undefined;

  let installedVersion: String | undefined;

  let versionMismatchDetected = false;
  export let game_name;
  $: activeGame.set(game_name);

  let gameSupportedByTooling = false;
  let showVccWarning;
  $: showVccWarning = type() == "windows" && !$isMinVCCRuntime;

  onMount(async () => {
    loadGameInfo();
    loadModInfo();
  });

  async function loadGameInfo() {
    // First off, check that they've downloaded and have a jak-project release set
    const activeVersionExists = await ensureActiveVersionStillExists();
    $VersionStore.activeVersionName = await getActiveVersion();

    if (activeVersionExists) {
      gameSupportedByTooling =
        await doesActiveToolingVersionSupportGame($activeGame);
      // verify the game is installed
      gameInstalled = await isGameInstalled($activeGame);

      // verify selected version is installed tooling version
      // - prompt them to either reinstall OR go and select their previous version
      if (gameInstalled) {
        installedVersion = await getInstalledVersion($activeGame);
        versionMismatchDetected =
          installedVersion !== $VersionStore.activeVersionName;
      }
    }
  }

  async function loadModInfo() {
    if (!modName || !modSource) {
      modInfoStore.set(undefined);
      return;
    }

    const modSourceData = await getModSourcesData();

    const foundMod: ModInfo | undefined = Object.values(modSourceData).find(
      (source) =>
        source.sourceName === modSource && source.mods.hasOwnProperty(modName),
    )?.mods[modName];

    if (!foundMod) {
      // couldn't find mod in modlists (user might be offline, or mod/list abandoned)
      // just create a dummy entry so they can at least launch game
      modInfoStore.set({
        name: modName,
        source: modSource,
        displayName: modName,
        description: "",
        authors: [],
        tags: [],
        supportedGames: [],
        versions: [],
        websiteUrl: null,
        perGameConfig: null,
        coverArtUrl: null,
        thumbnailArtUrl: null,
        externalLink: null,
      });
      return;
    }
    modInfoStore.set({ ...foundMod, name: modName, source: modSource });
  }

  async function updateGameState(event: any) {
    gameInstalled = await isGameInstalled($activeGame);
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

<div class="flex flex-col h-full p-5">
  {#if $VersionStore.activeVersionName === null}
    <GameToolsNotSet />
  {:else if $activeGame == "jak3"}
    <!-- TODO: remove this else if arm for jak3 support -->
    <!-- Delete GameInProgress.svelte component -->
    <GameInProgress />
  {:else if !gameSupportedByTooling}
    <GameNotSupportedByTooling />
  {:else if !gameInstalled}
    <GameSetup on:change={updateGameState} />
  {:else if gameJobToRun !== undefined}
    <GameJob
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
    {#if $activeGame === "jak2"}
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
    {#if $activeGame === "jak3"}
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
    {/if}
    {#if modName !== undefined}
      <GameControlsMod on:change={updateGameState} on:job={runGameJob} />
    {:else}
      <GameControls on:change={updateGameState} on:job={runGameJob} />
    {/if}
  {/if}
</div>
