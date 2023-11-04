<script>
  import { fromRoute, SupportedGame } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import { onMount } from "svelte";
  import { Spinner } from "flowbite-svelte";
  import GameJob from "../components/games/job/GameJob.svelte";
  import TexturePacks from "../components/games/features/texture-packs/TexturePacks.svelte";

  const params = useParams();
  let activeGame = SupportedGame.Jak1;
  let selectedFeature = "texture_packs";
  let componentLoaded = false;

  let gameJobToRun = undefined;

  let texturePacksToEnable = [];
  let texturePacksToDelete = [];

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

  async function runGameJob(event) {
    gameJobToRun = event.detail.type;
    texturePacksToEnable = event.detail.enabledPacks;
    texturePacksToDelete = event.detail.packsToDelete;
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
    <TexturePacks {activeGame} on:job={runGameJob} />
  {/if}
</div>
