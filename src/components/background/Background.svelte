<script lang="ts">
  import bgVideoJak1 from "$assets/videos/background-jak1.mp4";
  import bgVideoPosterJak1 from "$assets/images/background-jak1.webp";
  import bgVideoJak2 from "$assets/videos/background-jak2.webm";
  import bgVideoPosterJak2 from "$assets/images/background-jak2.png";
  import { useLocation } from "svelte-navigator";
  import { isGameInstalled } from "$lib/rpc/config";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";

  const location = useLocation();
  $: $location.pathname, updateStyle();

  let style = "";

  onMount(async () => {
    const unlistenInstalled = await listen("gameInstalled", (event) => {
      updateStyle();
    });
    const unlistenUninstalled = await listen("gameUninstalled", (event) => {
      updateStyle();
    });
  });

  async function updateStyle(): Promise<void> {
    let newStyle = "absolute -z-50 object-fill h-screen";
    let pathname = $location.pathname;
    if (pathname === "/jak1" || pathname === "/") {
      if (!(await isGameInstalled("jak1"))) {
        newStyle += " grayscale";
      }
    } else if (pathname === "/jak2") {
      if (!(await isGameInstalled("jak2"))) {
        newStyle += " grayscale";
      }
    } else if (pathname === "/jak3") {
      if (!(await isGameInstalled("jak3"))) {
        newStyle += " grayscale";
      }
    } else if (pathname === "/jakx") {
      if (!(await isGameInstalled("jakx"))) {
        newStyle += " grayscale";
      }
    }
    style = newStyle;
  }
</script>

<div class="height-screen">
  {#if $location.pathname == "/jak1" || $location.pathname == "/"}
    <video
      class={style}
      poster={bgVideoPosterJak1}
      src={bgVideoJak1}
      autoplay
      muted
      loop
    />
  {:else if $location.pathname == "/jak2"}
    <video
      class={style}
      poster={bgVideoPosterJak2}
      src={bgVideoJak2}
      autoplay
      muted
      loop
    />
  {/if}
</div>
