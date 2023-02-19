<script lang="ts">
  import bgVideoJak1 from "$assets/videos/background-jak1.mp4";
  // TODO - remake the poster images to be the actual first frame, at the same dimensions
  import bgVideoPosterJak1 from "$assets/images/background-jak1-fallback.webp";
  import bgVideoJak2 from "$assets/videos/background-jak2.webm";
  import bgVideoPosterJak2 from "$assets/images/background-jak2-fallback.png";
  import { useLocation } from "svelte-navigator";
  import { isGameInstalled } from "$lib/rpc/config";

  const location = useLocation();
  $: $location.pathname, updateStyle();
  // TODO - also update once installation completes (store / dispatch?)

  let style = "";

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
