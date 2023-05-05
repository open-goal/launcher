<script lang="ts">
  import bgVideoJak1 from "$assets/videos/background-jak1.webm";
  import bgVideoPosterJak1 from "$assets/images/background-jak1.webp";
  import bgVideoJak2 from "$assets/videos/background-jak2.webm";
  import bgVideoPosterJak2 from "$assets/images/background-jak2.webp";
  import { useLocation } from "svelte-navigator";
  import { getJak1MoviePath, getJak2MoviePath, isGameInstalled } from "$lib/rpc/config";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { convertFileSrc } from "@tauri-apps/api/tauri";

  const location = useLocation();
  $: $location.pathname, updateStyle();

  let style = "absolute -z-50 object-fill h-screen";
  //TODO Add logic to select background movie and fall back to default if needed.
  let customVideoJak1Path;
  let customJak1Video;
  let customVideoJak2Path;
  let customJak2Video;
  onMount(async () => {
    customJak1Video = (await getJak1MoviePath());
    customVideoJak1Path = convertFileSrc(await getJak1MoviePath());
    customJak2Video = (await getJak2MoviePath());
    customVideoJak2Path = convertFileSrc(await getJak2MoviePath());
    const unlistenInstalled = await listen("gameInstalled", (event) => {
      updateStyle();
    });
    const unlistenUninstalled = await listen("gameUninstalled", (event) => {
      updateStyle();
    });
  });

  async function updateStyle(): Promise<void> {
    customJak1Video = (await getJak1MoviePath());
    customVideoJak1Path = convertFileSrc(await getJak1MoviePath());
    customJak2Video = (await getJak2MoviePath());
    customVideoJak2Path = convertFileSrc(await getJak2MoviePath());
    

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

{#if $location.pathname == "/jak1" || $location.pathname == "/"}
  <video
    class={style}
    poster={bgVideoPosterJak1}
    src={customJak1Video? customVideoJak1Path:bgVideoJak1}
    autoplay
    muted
    loop
  />
{:else if $location.pathname == "/jak2"}
  <video
    class={style}
    poster={bgVideoPosterJak2}
    src={customJak2Video? customVideoJak2Path:bgVideoJak2}
    autoplay
    muted
    loop
  />
{/if}
