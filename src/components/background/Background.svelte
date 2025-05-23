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
  import coverArtPlaceholder from "$assets/images/mod-coverart-placeholder.webp";
  import { activeGame, modInfoStore } from "$lib/stores/AppStore";
  import { SupportedGame } from "$lib/constants";
  import { getLocalModThumbnailBase64 } from "$lib/rpc/features";
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { exists } from "@tauri-apps/plugin-fs";

  const location = useLocation();
  $: $location.pathname, updateBackground();
  $: $activeGame, updateBackground();
  $: $modInfoStore, updateModBackground();

  let style = "absolute object-fill h-screen brightness-75 pt-[60px] w-full";
  let jak1Background = "";
  let onWindows = platform() !== "linux";
  let modBackground = "";
  let grayscale = false;
  let bgVideo = null;

  onMount(async () => {
    const unlistenInstalled = await listen("gameInstalled", (event) => {
      updateBackground();
    }); // TODO - refactor this out
    const unlistenUninstalled = await listen("gameUninstalled", (event) => {
      updateBackground();
    }); // TODO - refactor this out
    // TODO - call this when the game is closed
    const milestoneImage = await getFurthestGameMilestone($activeGame);
    jak1Background = `/images/${$activeGame}/${milestoneImage}.jpg`;
    // TODO - do jak 2 milestones
  });

  async function updateBackground(): Promise<void> {
    bgVideo = null;
    if (!$activeGame) return;
    grayscale = !(await isGameInstalled($activeGame));

    const appDataDirPath = await appDataDir();
    const filePath = await join(
      appDataDirPath,
      "backgrounds",
      `${$activeGame}.mp4`,
    );
    const assetUrl = convertFileSrc(filePath);
    if (await exists(assetUrl)) {
      bgVideo = assetUrl;
    }
  }

  async function updateModBackground(): Promise<void> {
    modBackground = "";
    if (!$activeGame) return;
    if (!$modInfoStore) {
      // Handle local mod backgrounds
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
      }
    }

    modBackground =
      $modInfoStore.coverArtUrl ||
      $modInfoStore.perGameConfig[$activeGame].coverArtUrl ||
      coverArtPlaceholder;
  }
</script>

<!-- TODO: the three else if statements can go away once 1. the milestone code is finished and 2. jak3 is released -->
<div class:grayscale>
  {#if modBackground}
    <!-- svelte-ignore a11y_missing_attribute -->
    <img class={style} src={modBackground} />
  {:else if $activeGame == SupportedGame.Jak1}
    <video
      class={style}
      poster={jak1Background}
      src={bgVideo}
      autoplay
      muted
      loop
    ></video>
  {:else if $activeGame == SupportedGame.Jak2}
    <video
      class={style}
      poster={jak2Background}
      src={bgVideo}
      autoplay
      muted
      loop
    ></video>
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
