<script lang="ts">
  import { useParams } from "svelte-navigator";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { isGameInstalled } from "$lib/rpc/config";
  import GameJob from "../old_components/games/job/GameJob.svelte";
  import { ensureActiveVersionStillExists } from "$lib/rpc/versions";
  import GameToolsNotSet from "../old_components/games/GameToolsNotSet.svelte";
  import { isMinVCCRuntime } from "$lib/stores/VersionStore";
  import type { Job } from "$lib/utils/jobs";
  import { type } from "@tauri-apps/plugin-os";
  import { getModSourcesData } from "$lib/rpc/cache";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import GameControlsMod from "../old_components/games/GameControlsMod.svelte";
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

  let versionMismatchDetected = false;
  export let game_name;
  $: activeGame.set(game_name);

  let showVccWarning;
  $: showVccWarning = type() == "windows" && !$isMinVCCRuntime;

  onMount(async () => {
    loadGameInfo();
    loadModInfo();
  });

  async function loadGameInfo() {
    // First off, check that they've downloaded and have a jak-project release set
    const activeVersionExists = await ensureActiveVersionStillExists();
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

    if (!foundMod) return;

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
  {#if activeVersionName === null}
    <GameToolsNotSet />
  {:else if gameJobToRun !== undefined}
    <GameJob
      jobType={gameJobToRun}
      modSourceName={modSource}
      modDownloadUrl={modDownloadUrlToInstall}
      modVersion={modVersionToInstall}
      {modName}
      on:jobFinished={gameJobFinished}
    />
  {:else if modName !== undefined}
    <GameControlsMod on:change={updateGameState} on:job={runGameJob} />
  {/if}
</div>
