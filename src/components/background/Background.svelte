<script lang="ts">
  import { useLocation } from "svelte-navigator";
  import { isGameInstalled } from "$lib/rpc/config";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { getFurthestGameMilestone } from "$lib/rpc/game";
  import jak2Background from "$assets/images/background-jak2.webp";
  import jak3InProgressVid from "$assets/videos/jak3-dev.mp4";
  import jak3InProgressPoster from "$assets/videos/jak3-poster.png";
  import { platform } from "@tauri-apps/api/os";

  const location = useLocation();
  $: $location.pathname, updateStyle();

  let style = "absolute object-fill h-screen brightness-75";
  let jak1Image = "";
  let onWindows = false;

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
  });

  async function updateStyle(): Promise<void> {
    let newStyle = "absolute object-fill h-screen brightness-75";
    let pathname = $location.pathname;
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
  <img class={style} src={jak1Image} />
{:else if $location.pathname == "/jak2"}
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
    <img class={style} src={jak3InProgressPoster} />
  {/if}
{/if}
