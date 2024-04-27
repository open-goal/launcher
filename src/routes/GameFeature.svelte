<script lang="ts">
  import { fromRoute, SupportedGame } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import { onMount } from "svelte";
  import { Spinner } from "flowbite-svelte";
  import GameJob from "../components/games/job/GameJob.svelte";
  import TexturePacks from "../components/games/features/texture-packs/TexturePacks.svelte";
  import ModSelection from "../components/games/features/mods/ModSelection.svelte";
  import { refreshModSources } from "$lib/rpc/cache";

  const params = useParams();
  let activeGame = SupportedGame.Jak1;
  let selectedFeature = "texture_packs";
  let componentLoaded = false;

  let gameJobToRun: undefined = undefined;

  let texturePacksToEnable: any[] | undefined = undefined;
  let texturePacksToDelete: any[] | undefined = undefined;

  let modSourceName: string | undefined = undefined;
  let modName: string | undefined = undefined;
  let modVersion: string | undefined = undefined;

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
    if (
      $params["feature"] !== undefined &&
      $params["feature"] !== null &&
      $params["feature"] !== ""
    ) {
      selectedFeature = $params["feature"];
    } else {
      selectedFeature = "texture_packs";
    }

    componentLoaded = true;
  });

  async function runTexturePackGameJob(event: {
    detail: { type: any; enabledPacks: any[]; packsToDelete: any[] };
  }) {
    gameJobToRun = event.detail.type;
    texturePacksToEnable = event.detail.enabledPacks;
    texturePacksToDelete = event.detail.packsToDelete;
    modSourceName = undefined;
    modName = undefined;
    modVersion = undefined;
  }

  async function runModInstallGameJob(event: {
    detail: { type: any; modSourceName: string; modName: string };
  }) {
    gameJobToRun = event.detail.type;
    texturePacksToEnable = undefined;
    texturePacksToDelete = undefined;
    modSourceName = event.detail.modSourceName;
    modName = event.detail.modName;
    modVersion = event.detail.modVersion;
  }

  async function gameJobFinished() {
    gameJobToRun = undefined;
  }
</script>

<div class="flex flex-col h-full bg-[#1e1e1e]">
  {#if !componentLoaded}
    <div class="flex flex-col h-full justify-center items-center">
      <Spinner color="yellow" size={"12"} />
    </div>
  {:else if gameJobToRun !== undefined}
    <div class="flex flex-col p-5 h-full">
      <GameJob
        {activeGame}
        jobType={gameJobToRun}
        {texturePacksToEnable}
        {texturePacksToDelete}
        {modSourceName}
        {modName}
        {modVersion}
        on:jobFinished={gameJobFinished}
      />
    </div>
  {:else if selectedFeature === "texture_packs"}
    <TexturePacks {activeGame} on:job={runTexturePackGameJob} />
  {:else if selectedFeature === "mods"}
    <ModSelection {activeGame} on:job={runModInstallGameJob}></ModSelection>
  {/if}
</div>
