<script lang="ts">
  import { SupportedGame } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import GameJob from "../components/games/job/GameJob.svelte";
  import TexturePacks from "../components/games/features/texture-packs/TexturePacks.svelte";
  import ModSelection from "../components/games/features/mods/ModSelection.svelte";

  const params = useParams();
  $: activeGame = $params["game_name"] || SupportedGame.Jak1;
  $: selectedFeature = $params["feature"] || "texture_packs";

  let gameJobToRun: undefined = undefined;

  let texturePacksToEnable: any[] | undefined = undefined;
  let texturePacksToDelete: any[] | undefined = undefined;

  let modDownloadUrl: string | undefined = undefined;
  let modSourceName: string | undefined = undefined;
  let modName: string | undefined = undefined;
  let modVersion: string | undefined = undefined;

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
    detail: {
      type: any;
      modDownloadUrl: string;
      modSourceName: string;
      modName: string;
    };
  }) {
    gameJobToRun = event.detail.type;
    texturePacksToEnable = undefined;
    texturePacksToDelete = undefined;
    modDownloadUrl = event.detail.modDownloadUrl;
    modSourceName = event.detail.modSourceName;
    modName = event.detail.modName;
    modVersion = event.detail.modVersion;
  }

  function gameJobFinished() {
    gameJobToRun = undefined;
  }
</script>

<div class="flex flex-col h-full bg-[#1e1e1e]">
  {#if gameJobToRun}
    <div class="flex flex-col p-5 h-full">
      <GameJob
        {activeGame}
        jobType={gameJobToRun}
        {texturePacksToEnable}
        {texturePacksToDelete}
        {modDownloadUrl}
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
