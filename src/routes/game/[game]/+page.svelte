<script lang="ts">
  import type { PageProps } from "./$types";
  import { page } from "$app/state";
  const activeGame = $derived(page.params.game);

  let { data }: PageProps = $props();
</script>

<!-- Hello from {activeGame} page
<div class="container">
  <a href="/game/jak1">jak1</a>
  <a href="/game/jak2">jak2</a>
  <a href="/game/jak3">jak3</a>
</div> -->
