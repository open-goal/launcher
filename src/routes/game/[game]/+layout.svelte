<script lang="ts">
  import type { LayoutProps } from "./$types";
  import { page } from "$app/state";

  let { data, children }: LayoutProps = $props();
  const activeGame = $derived(page.params.game);
</script>

<!-- keep this its important -->
{@render children()}

<!-- BACKGROUND -->
<img
  class="absolute right-0 top-0 w-screen h-screen -z-100"
  src={`/images/${activeGame}/basin.jpg`}
  alt=""
/>
