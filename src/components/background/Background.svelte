<script lang="ts">
  import { isGameInstalled } from "$lib/rpc/config";
  import { onDestroy, onMount } from "svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { getFurthestGameMilestone } from "$lib/rpc/game";
  import jak2Background from "$assets/images/background-jak2.webp";
  import jak3InProgressVid from "$assets/videos/jak3-dev.mp4";
  import jak3InProgressPoster from "$assets/videos/jak3-poster.png";
  import { platform } from "@tauri-apps/plugin-os";
  import coverArtPlaceholder from "$assets/images/mod-coverart-placeholder.webp";
  import { modInfoStore } from "$lib/stores/AppStore";
  import { getLocalModThumbnailBase64 } from "$lib/rpc/features";
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { exists } from "@tauri-apps/plugin-fs";
  import { route } from "../../router";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo.ts";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";

  const gameParam = $derived(route.params.game_name);
  let activeGame: SupportedGame | undefined = $state(undefined);

  $effect(() => {
    const activeGameFromParam = toSupportedGame(gameParam);
    if (activeGameFromParam) {
      activeGame = activeGameFromParam;
      if (route.params.mod_name) {
        updateModBackground(activeGame, $modInfoStore);
      } else {
        updateBackground(activeGame);
      }
    }
  });

  $effect(() => {
    if (activeGame && route.params.mod_name) {
      updateModBackground(activeGame, $modInfoStore);
    }
  });

  const onWindows = platform() !== "linux";
  let grayscale = $state(false);
  let bgVideo: string | null = $state(null);
  let jak1Background: string | undefined = $state(undefined);
  let modBackground: string | undefined = $state(undefined);
  let style = "absolute object-fill h-screen brightness-75 pt-[60px] w-full";

  let installedListener: UnlistenFn | undefined = undefined;
  let uninstalledListener: UnlistenFn | undefined = undefined;

  onMount(async () => {
    installedListener = await listen("gameInstalled", (event) => {
      if (activeGame) {
        updateBackground(activeGame);
      }
    }); // TODO - refactor this out
    uninstalledListener = await listen("gameUninstalled", (event) => {
      if (activeGame) {
        updateBackground(activeGame);
      }
    }); // TODO - refactor this out
  });

  onDestroy(() => {
    if (installedListener) {
      installedListener();
    }
    if (uninstalledListener) {
      uninstalledListener();
    }
  });

  async function updateBackground(activeGame: SupportedGame): Promise<void> {
    bgVideo = null;
    grayscale = !(await isGameInstalled(activeGame));

    const appDataDirPath = await appDataDir();
    const filePath = await join(
      appDataDirPath,
      "backgrounds",
      `${activeGame}.mp4`,
    );
    if (await exists(filePath)) {
      bgVideo = convertFileSrc(filePath);
    }

    // TODO - call this when the game is closed
    const milestoneImage = await getFurthestGameMilestone(activeGame);
    jak1Background = `/images/${activeGame}/${milestoneImage}.jpg`;
    // TODO - do jak 2 milestones
  }

  async function updateModBackground(
    activeGame: SupportedGame,
    modInfo: ModInfo | undefined,
  ): Promise<void> {
    modBackground = "";
    if (!modInfo) {
      // Handle local mod backgrounds
      const pathComponents = route.pathname.split("/").filter((s) => s !== "");
      if (pathComponents.length === 5) {
        const modSource = decodeURI(pathComponents[3]); // TODO: i dislike this pattern, but im keeping it for now
        const modName = decodeURI(pathComponents[4]);
        if (modSource === "_local") {
          const coverResult = await getLocalModThumbnailBase64(
            activeGame,
            modName,
          );
          modBackground = coverResult || coverArtPlaceholder;
          return;
        }
      }
      return;
    }
    modBackground = coverArtPlaceholder;
    if (modInfo?.coverArtUrl) {
      modBackground = modInfo.coverArtUrl;
    } else if (
      modInfo?.perGameConfig &&
      modInfo.perGameConfig[activeGame]?.coverArtUrl
    ) {
      modBackground = modInfo.perGameConfig[activeGame].coverArtUrl;
    }
  }
</script>

<!-- TODO: the three else if statements can go away once 1. the milestone code is finished and 2. jak3 is released -->
<div class:grayscale>
  {#if modBackground && route.pathname.includes("mods")}
    <!-- svelte-ignore a11y_missing_attribute -->
    <img class={style} src={modBackground} />
  {:else if activeGame === "jak1"}
    {#if bgVideo}
      <video
        class={style}
        poster={jak1Background}
        src={bgVideo}
        autoplay
        muted
        loop
      ></video>
    {:else}
      <!-- svelte-ignore a11y_missing_attribute -->
      <img class={style} src={jak1Background} />
    {/if}
  {:else if activeGame === "jak2"}
    {#if bgVideo}
      <video
        class={style}
        poster={jak2Background}
        src={bgVideo}
        autoplay
        muted
        loop
      ></video>
    {:else}
      <!-- svelte-ignore a11y_missing_attribute -->
      <img class={style} src={jak2Background} />
    {/if}
  {:else if activeGame === "jak3"}
    {#if bgVideo}
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
