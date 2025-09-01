<script lang="ts">
  import type { LayoutProps } from "./$types";
  import { page } from "$app/state";

  let { data, children }: LayoutProps = $props();
  const game = $derived(page.params.game);
  $effect(() => localStorage.setItem("lastGame", game));
</script>

<!-- BACKGROUND -->
<img
  class="absolute right-0 top-0 w-screen h-screen -z-100"
  src={`/images/${game}/background.webp`}
  alt=""
/>

<!-- keep this its important -->
{@render children()}
