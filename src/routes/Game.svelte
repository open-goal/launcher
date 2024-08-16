<script lang="ts">
  import { fromRoute, getInternalName, SupportedGame } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import GameControls from "../components/games/GameControls.svelte";
  import GameSetup from "../components/games/setup/GameSetup.svelte";
  import { onMount } from "svelte";
  import { Alert, Spinner } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import {
    doesActiveToolingVersionSupportGame,
    getInstalledVersion,
    getInstalledVersionFolder,
    isGameInstalled,
    isMinimumVCCRuntimeInstalled,
  } from "$lib/rpc/config";
  import GameJob from "../components/games/job/GameJob.svelte";
  import GameUpdate from "../components/games/setup/GameUpdate.svelte";
  import {
    ensureActiveVersionStillExists,
    getActiveVersion,
    getActiveVersionFolder,
  } from "$lib/rpc/versions";
  import GameToolsNotSet from "../components/games/GameToolsNotSet.svelte";
  import GameNotSupportedByTooling from "../components/games/GameNotSupportedByTooling.svelte";
  import { VersionStore } from "$lib/stores/VersionStore";
  import type { Job } from "$lib/utils/jobs";
  import { type } from "@tauri-apps/api/os";
  import { getModSourcesData, refreshModSources } from "$lib/rpc/cache";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import GameControlsMod from "../components/games/GameControlsMod.svelte";

  const params = useParams();
  $: $params, loadGameInfo();

  export let modName: string | undefined = undefined;
  export let modSource: string | undefined = undefined;

  let activeGame = SupportedGame.Jak1;
  let modDisplayName: string | undefined = undefined;
  let componentLoaded = false;

  let gameInstalled = false;
  let gameJobToRun: Job | undefined = undefined;

  let installedVersion: String | undefined;
  let installedVersionFolder: String | undefined;

  let versionMismatchDetected = false;

  let gameInBeta = false;
  let gameSupportedByTooling = false;
  let showVccWarning = false;

  onMount(async () => {
    loadGameInfo();
    const osType = await type();
    if (osType == "Windows_NT") {
      showVccWarning = !(await isMinimumVCCRuntimeInstalled());
    }
  });

  async function loadGameInfo() {
    componentLoaded = false;
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
    if (
      $params["mod_name"] !== undefined &&
      $params["mod_name"] !== null &&
      $params["mod_name"] !== ""
    ) {
      modName = $params["mod_name"];
      // Go get the mod's display name
      // TODO now - centralize this in a store so we don't unnecessarily fetch the info
      await refreshModSources();
      const modSourceData = await getModSourcesData();
      // Find the source
      let foundMod: ModInfo | undefined = undefined;
      for (const [sourceUrl, sourceInfo] of Object.entries(modSourceData)) {
        if (
          sourceInfo.sourceName === modSource &&
          sourceInfo.mods.hasOwnProperty(modName)
        ) {
          foundMod = sourceInfo.mods[modName];
          break;
        }
      }
      // Prefer pre-game-config if available
      if (foundMod !== undefined) {
        modDisplayName = foundMod.displayName;
      } else {
        modDisplayName = modName;
      }
    }
    if (
      $params["source_url"] !== undefined &&
      $params["source_url"] !== null &&
      $params["source_url"] !== ""
    ) {
      modSource = $params["source_url"];
    }

    gameInBeta = activeGame === SupportedGame.Jak2;

    // First off, check that they've downloaded and have a jak-project release set
    const activeVersionExists = await ensureActiveVersionStillExists();
    $VersionStore.activeVersionType = await getActiveVersionFolder();
    $VersionStore.activeVersionName = await getActiveVersion();

    if (activeVersionExists) {
      gameSupportedByTooling = await doesActiveToolingVersionSupportGame(
        getInternalName(activeGame),
      );
      // First obvious thing to check -- is the game installed at all
      gameInstalled = await isGameInstalled(getInternalName(activeGame));

      // Next step, check if there is a version mismatch
      // - they installed the game before with a different version than what they currently have selected
      // - prompt them to either reinstall OR go and select their previous version
      if (gameInstalled) {
        installedVersion = await getInstalledVersion(
          getInternalName(activeGame),
        );
        installedVersionFolder = await getInstalledVersionFolder(
          getInternalName(activeGame),
        );
        versionMismatchDetected =
          installedVersion !== $VersionStore.activeVersionName ||
          installedVersionFolder !== $VersionStore.activeVersionType;
      }
    }

    componentLoaded = true;
  }

  async function updateGameState(event: any) {
    gameInstalled = await isGameInstalled(getInternalName(activeGame));
  }

  async function runGameJob(event: any) {
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
  {:else if !gameSupportedByTooling}
    <GameNotSupportedByTooling />
  {:else if !gameInstalled}
    <GameSetup {activeGame} on:change={updateGameState} />
  {:else if gameJobToRun !== undefined}
    <GameJob
      {activeGame}
      jobType={gameJobToRun}
      modSourceName={modSource}
      {modName}
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
    {#if showVccWarning}
      <Alert color="red" rounded={false} class="border-t-4">
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
    {#if gameInBeta}
      <Alert color="red" rounded={false} class="border-t-4">
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
              href="https://github.com/open-goal/jak-project/issues/new?assignees=&labels=bug%2Cjak2&projects=&template=jak2-bug-report.yml"
              target="_blank"
              rel="noopener noreferrer"
              >{$_("gameControls_beta_bugReport_linkText")}</a
            >
          </li>
        </ul>
      </Alert>
    {/if}
    {#if modName !== undefined}
      <GameControlsMod
        {activeGame}
        {modName}
        {modDisplayName}
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
