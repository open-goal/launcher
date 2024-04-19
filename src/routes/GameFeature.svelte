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

  let texturePacksToEnable: any[] = [];
  let texturePacksToDelete: any[] = [];

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
    detail: { type: undefined; enabledPacks: any[]; packsToDelete: any[] };
  }) {
    gameJobToRun = event.detail.type;
    texturePacksToEnable = event.detail.enabledPacks;
    texturePacksToDelete = event.detail.packsToDelete;
  }

  async function runModInstallGameJob(event: {
    detail: { type: undefined; enabledPacks: any[]; packsToDelete: any[] };
  }) {
    console.log("todo!");
  }

  async function gameJobFinished() {
    gameJobToRun = undefined;
  }
</script>

<div class="flex flex-col h-full bg-slate-900">
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
        on:jobFinished={gameJobFinished}
      />
    </div>
  {:else if selectedFeature === "texture_packs"}
    <TexturePacks {activeGame} on:job={runTexturePackGameJob} />
  {:else if selectedFeature === "mods"}
    <ModSelection {activeGame} on:job={runModInstallGameJob}></ModSelection>
  {/if}
</div>
