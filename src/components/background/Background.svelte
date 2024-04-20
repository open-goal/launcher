<script lang="ts">
  import { useLocation, useParams } from "svelte-navigator";
  import { isGameInstalled } from "$lib/rpc/config";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { getFurthestGameMilestone } from "$lib/rpc/game";
  import jak2Background from "$assets/images/background-jak2.webp";
  import jak3InProgressVid from "$assets/videos/jak3-dev.mp4";
  import jak3InProgressPoster from "$assets/videos/jak3-poster.png";
  import { platform } from "@tauri-apps/api/os";
  import { getModSourcesData, refreshModSources } from "$lib/rpc/cache";

  const location = useLocation();
  $: $location.pathname, updateStyle();

  let style = "absolute object-fill h-screen brightness-75";
  let jak1Image = "";
  let onWindows = false;
  let modBackground = "";

  onMount(async () => {
    onWindows = (await platform()) === "win32";
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
    // Handle mod backgrounds
    const pathComponents = $location.pathname
      .split("/")
      .filter((s) => s !== "");
    if (pathComponents.length === 5) {
      const modSourceName = decodeURI(pathComponents[3]);
      const modName = decodeURI(pathComponents[4]);
      // TODO - centralize this in a store so we don't unnecessarily fetch the info
      await refreshModSources();
      const modSourceData = await getModSourcesData();
      // Find the source
      let foundMod = undefined;
      for (const [sourceUrl, sourceInfo] of Object.entries(modSourceData)) {
        if (
          sourceInfo.sourceName === modSourceName &&
          sourceInfo.mods.hasOwnProperty(modName)
        ) {
          foundMod = sourceInfo.mods[modName];
        }
      }
      if (foundMod !== undefined && foundMod.coverArtUrl !== null) {
        modBackground = foundMod.coverArtUrl;
      }
    } else {
      modBackground = "";
    }
    let newStyle = "absolute object-fill h-screen brightness-75";
    let pathname = $location.pathname;
    // TODO - check if mod is installed or not
    if (pathname.startsWith("/jak1") || pathname === "/") {
      if (!(await isGameInstalled("jak1"))) {
        newStyle += " grayscale";
      }
    } else if (pathname.startsWith("/jak2")) {
      if (!(await isGameInstalled("jak2"))) {
        newStyle += " grayscale";
      }
    } else if (pathname.startsWith("/jak3")) {
      if (!(await isGameInstalled("jak3"))) {
        newStyle += " grayscale";
      }
    } else if (pathname.startsWith("/jakx")) {
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
