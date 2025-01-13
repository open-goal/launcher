<script lang="ts">
  import { useLocation, useParams } from "svelte-navigator";
  import { isGameInstalled } from "$lib/rpc/config";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { getFurthestGameMilestone } from "$lib/rpc/game";
  import jak2Background from "$assets/images/background-jak2.webp";
  import jak3InProgressVid from "$assets/videos/jak3-dev.mp4";
  import jak3InProgressPoster from "$assets/videos/jak3-poster.png";
  import { platform } from "@tauri-apps/plugin-os";
  import { getModSourcesData, refreshModSources } from "$lib/rpc/cache";
  import coverArtPlaceholder from "$assets/images/mod-coverart-placeholder.webp";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import { getLocalModThumbnailBase64 } from "$lib/rpc/features";

  const location = useLocation();
  $: $location.pathname, updateStyle();

  let style = "absolute object-fill h-screen brightness-75 pt-[60px] w-full";
  let jak1Image = "";
  let onWindows = platform() === "windows";
  let modBackground = "";

  onMount(async () => {
    const unlistenInstalled = await listen("gameInstalled", (event) => {
      updateStyle();
    });
    const unlistenUninstalled = await listen("gameUninstalled", (event) => {
      updateStyle();
    });
    // TODO - call this if the game is closed as well
    const jak1_milestone = await getFurthestGameMilestone("jak1");
    jak1Image = `/images/jak1/${jak1_milestone}.jpg`;
    // TODO - do jak 2 milestones once the game is considered out of beta
  });

  async function updateStyle(): Promise<void> {
    // figure out the game
    let pathname = $location.pathname;
    let activeGame = "";
    if (pathname.startsWith("/jak1") || pathname === "/") {
      activeGame = "jak1";
    } else if (pathname.startsWith("/jak2")) {
      activeGame = "jak2";
    } else if (pathname.startsWith("/jak3")) {
      activeGame = "jak3";
    } else if (pathname.startsWith("/jakx")) {
      activeGame = "jakx";
    }

    // Handle mod backgrounds
    const pathComponents = $location.pathname
      .split("/")
      .filter((s) => s !== "");
    if (pathComponents.length === 5) {
      const modSourceName = decodeURI(pathComponents[3]);
      const modName = decodeURI(pathComponents[4]);
      // TODO now - centralize this in a store so we don't unnecessarily fetch the info
      await refreshModSources();
      const modSourceData = await getModSourcesData();
      // Find the source
      let foundMod: ModInfo | undefined = undefined;
      for (const [sourceUrl, sourceInfo] of Object.entries(modSourceData)) {
        if (
          sourceInfo.sourceName === modSourceName &&
          sourceInfo.mods.hasOwnProperty(modName)
        ) {
          foundMod = sourceInfo.mods[modName];
          break;
        }
      }
      if (modSourceName === "_local") {
        const coverResult = await getLocalModThumbnailBase64(
          activeGame,
          modName,
        );
        if (coverResult === "") {
          modBackground = coverArtPlaceholder;
        } else {
          modBackground = coverResult;
        }
      }
      // Prefer pre-game-config if available
      else if (foundMod !== undefined) {
        if (
          foundMod.perGameConfig !== null &&
          foundMod.perGameConfig.hasOwnProperty(activeGame) &&
          foundMod.perGameConfig[activeGame].coverArtUrl !== null
        ) {
          modBackground = foundMod.perGameConfig[activeGame].coverArtUrl;
        } else if (foundMod.coverArtUrl !== null) {
          modBackground = foundMod.coverArtUrl;
        }
      } else {
        modBackground = coverArtPlaceholder;
      }
    } else {
      modBackground = "";
    }
    let newStyle =
      "absolute object-fill h-screen brightness-75 pt-[60px] w-full";
    if (activeGame === "jak1") {
      if (!(await isGameInstalled("jak1"))) {
        newStyle += " grayscale";
      }
    } else if (activeGame === "jak2") {
      if (!(await isGameInstalled("jak2"))) {
        newStyle += " grayscale";
      }
    } else if (activeGame === "jak3") {
      if (!(await isGameInstalled("jak3"))) {
        newStyle += " grayscale";
      }
    } else if (activeGame === "jakx") {
      if (!(await isGameInstalled("jakx"))) {
        newStyle += " grayscale";
      }
    }
    style = newStyle;
  }
</script>

{#if $location.pathname == "/jak1" || $location.pathname == "/"}
  <!-- svelte-ignore a11y-missing-attribute -->
  <img class={style} src={jak1Image} />
{:else if $location.pathname == "/jak2"}
  <!-- svelte-ignore a11y-missing-attribute -->
  <img class={style} src={jak2Background} />
{:else if $location.pathname == "/jak3"}
  {#if onWindows}
    <video
      class={style}
      poster={jak3InProgressPoster}
      src={jak3InProgressVid}
      autoplay
      muted
      loop
    />
  {:else}
    <!-- svelte-ignore a11y-missing-attribute -->
    <img class={style} src={jak3InProgressPoster} />
  {/if}
{:else if modBackground !== ""}
  <!-- svelte-ignore a11y-missing-attribute -->
  <img class={style} src={modBackground} />
{/if}
