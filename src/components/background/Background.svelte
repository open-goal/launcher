<script lang="ts">
  import { isGameInstalled } from "$lib/rpc/config";
  import { onDestroy, onMount } from "svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { getFurthestGameMilestone } from "$lib/rpc/game";
  import jak2Background from "$assets/images/background-jak2.webp";
  import jak3InProgressVid from "$assets/videos/jak3-dev.mp4";
  import jak3InProgressPoster from "$assets/videos/jak3-poster.png";
  import coverArtPlaceholder from "$assets/images/mod-coverart-placeholder.webp";
  import { getLocalModThumbnailBase64 } from "$lib/rpc/features";
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { exists } from "@tauri-apps/plugin-fs";
  import { route } from "../../router";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo.ts";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import { getModInfo } from "$lib/rpc/bindings/utils/ModInfo";
  import { searchParams } from "sv-router";

  const gameParam = $derived(route.params.game_name);
  let activeGame: SupportedGame | undefined = $state(undefined);
  let loading = $state(false);
  let grayscale = $state(false);
  let bgVideo: string | null = $state(null);
  let jak1Background: string | undefined = $state(undefined);
  let modBackground: string | undefined = $state(undefined);
  let style = "absolute object-fill h-screen brightness-75 pt-[60px] w-full";

  let installedListener: UnlistenFn | undefined = undefined;
  let uninstalledListener: UnlistenFn | undefined = undefined;

  onMount(async () => {
    loading = true;
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
    loading = false;
  });

  onDestroy(() => {
    if (installedListener) {
      installedListener();
    }
    if (uninstalledListener) {
      uninstalledListener();
    }
  });

  $effect(() => {
    loading = true;
    let activeGameFromRoute = toSupportedGame(gameParam);
    // if we can't find it from the route, look at the query params
    if (!activeGameFromRoute && searchParams.has("activeGame")) {
      const param = searchParams.get("activeGame");
      if (param) {
        activeGameFromRoute = toSupportedGame(param.toString());
      }
    }
    if (activeGameFromRoute) {
      activeGame = activeGameFromRoute;
      if (route.params.mod_name && route.params.source_name) {
        getModInfo(
          activeGame,
          route.params.mod_name,
          route.params.source_name,
        ).then((modInfo) => {
          if (activeGame) {
            updateModBackground(activeGame, modInfo).then(() => {
              loading = false;
            });
          }
        });
      } else {
        updateBackground(activeGame).then(() => {
          loading = false;
        });
      }
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
    modInfo: ModInfo,
  ): Promise<void> {
    modBackground = "";
    if (modInfo.source === "_local") {
      const coverResult = await getLocalModThumbnailBase64(
        activeGame,
        modInfo.name,
      );
      modBackground = coverResult || coverArtPlaceholder;
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
{#if !loading}
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
{/if}
