<script lang="ts">
  import { useLocation } from "svelte-navigator";
  import { isGameInstalled } from "$lib/rpc/config";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { getFurthestGameMilestone } from "$lib/rpc/game";
  import jak2Background from "$assets/images/background-jak2.webp";
  import jak3InProgressVid from "$assets/videos/jak3-dev.mp4";
  import jak3InProgressPoster from "$assets/videos/jak3-poster.png";
  import { platform } from "@tauri-apps/plugin-os";
  import { getModSourcesData } from "$lib/rpc/cache";
  import coverArtPlaceholder from "$assets/images/mod-coverart-placeholder.webp";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import { getLocalModThumbnailBase64 } from "$lib/rpc/features";
  import { activeGame } from "$lib/stores/AppStore";
  import { SupportedGame } from "$lib/constants";

  const location = useLocation();
  $: $location.pathname, updateStyle();
  $: $activeGame, updateStyle();

  let style = "absolute object-fill h-screen brightness-75 pt-[60px] w-full";
  let jak1Image = "";
  let onWindows = platform() !== "linux";
  let modBackground = "";
  let grayscale = false;

  onMount(async () => {
    const unlistenInstalled = await listen("gameInstalled", (event) => {
      updateStyle();
    }); // TODO - refactor this out
    const unlistenUninstalled = await listen("gameUninstalled", (event) => {
      updateStyle();
    }); // TODO - refactor this out
    // TODO - call this when the game is closed
    const milestoneImage = await getFurthestGameMilestone($activeGame);
    jak1Image = `/images/${$activeGame}/${milestoneImage}.jpg`;
    // TODO - do jak 2 milestones
  });

  async function updateStyle(): Promise<void> {
    if (!$activeGame) return;
    grayscale = !(await isGameInstalled($activeGame));
    modBackground = "";

    // Handle mod backgrounds
    const pathComponents = $location.pathname
      .split("/")
      .filter((s) => s !== "");
    if (pathComponents.length === 5) {
      const modSource = decodeURI(pathComponents[3]); // TODO: i dislike this pattern, but im keeping it for now
      const modName = decodeURI(pathComponents[4]);

      if (modSource === "_local") {
        const coverResult = await getLocalModThumbnailBase64(
          $activeGame,
          modName,
        );
        modBackground = coverResult || coverArtPlaceholder;
        return;
      }

      // TODO now - centralize this in a store so we don't unnecessarily fetch the info
      const modSourceData = await getModSourcesData();
      const foundMod: ModInfo | undefined = Object.values(modSourceData).find(
        (source) =>
          source.sourceName === modSource &&
          source.mods.hasOwnProperty(modName),
      )?.mods[modName];

      modBackground =
        foundMod?.perGameConfig?.[$activeGame]?.coverArtUrl ||
        foundMod?.coverArtUrl ||
        coverArtPlaceholder;
    }
  }
</script>

<!-- TODO: the three else if statements can go away once 1. the milestone code is finished and 2. jak3 is released -->
<div class:grayscale>
  {#if modBackground}
    <!-- svelte-ignore a11y_missing_attribute -->
    <img class={style} src={modBackground} />
  {:else if $activeGame == SupportedGame.Jak1}
    <!-- svelte-ignore a11y_missing_attribute -->
    <img class={style} src={jak1Image} />
  {:else if $activeGame == SupportedGame.Jak2}
    <!-- svelte-ignore a11y_missing_attribute -->
    <img class={style} src={jak2Background} />
  {:else if $activeGame == SupportedGame.Jak3}
    {#if onWindows}
      <video
        class={style}
        poster={jak3InProgressPoster}
        src={jak3InProgressVid}
        autoplay
        muted
        loop
      ></video>
    {:else}
      <!-- svelte-ignore a11y_missing_attribute -->
      <img class={style} src={jak3InProgressPoster} />
    {/if}
  {/if}
</div>
