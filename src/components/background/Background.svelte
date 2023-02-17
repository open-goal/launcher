<script type="ts">
  import bgVideoJak1 from "$assets/videos/background-jak1.mp4";
  import bgVideoPosterJak1 from "$assets/images/background-jak1-fallback.mp4";
  import bgVideoJak2 from "$assets/videos/background-jak2.webm";
  import bgVideoPosterJak2 from "$assets/images/background-jak2-fallback.png";
  import { useLocation } from "svelte-navigator";

  const location = useLocation();
  $: pathname = $location.pathname;
</script>

<div class="height-screen">
  {#if pathname == "/jak1" || pathname == "/"}
    <video
      class="absolute -z-50 object-fill h-screen"
      poster={bgVideoPosterJak1}
      src={bgVideoJak1}
      autoplay
      muted
      loop
    />
  {:else if pathname == "/jak2"}
    <video
      class="absolute -z-50 object-fill h-screen"
      poster={bgVideoPosterJak2}
      src={bgVideoJak2}
      autoplay
      muted
      loop
    />
  {/if}
</div>
