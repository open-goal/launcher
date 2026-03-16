<script lang="ts">
  import GameControls from "../components/games/GameControls.svelte";
  import { _ } from "svelte-i18n";
  import GameControlsMod from "../components/games/GameControlsMod.svelte";
  import GameInProgress from "../components/games/GameInProgress.svelte";
  import { route } from "../router";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame.ts";
  import GameBetaAlert from "../components/games/GameBetaAlert.svelte";

  const activeGame: SupportedGame | undefined = $derived(
    toSupportedGame(route.params.game_name),
  );
  let modName: string | undefined = $derived(route.params.mod_name);
  let modSource: string | undefined = $derived(route.params.source_name);
</script>

{#if activeGame}
  <div class="flex flex-col h-full p-5">
    {#if activeGame == "jak3" || activeGame == "jakx"}
      <!-- TODO: remove this else if arm for jak3 support -->
      <GameInProgress />
    {:else}
      <!-- Jak 2 BETA warning -->
      {#if activeGame === "jak2"}
        <GameBetaAlert></GameBetaAlert>
      {/if}
      {#if modName && modSource}
        <GameControlsMod {activeGame} {modName} {modSource} />
      {:else}
        <GameControls {activeGame} />
      {/if}
    {/if}
  </div>
{/if}
