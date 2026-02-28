<script lang="ts">
  import GameControls from "../components/games/GameControls.svelte";
  import { _ } from "svelte-i18n";
  import GameControlsMod from "../components/games/GameControlsMod.svelte";
  import GameInProgress from "../components/games/GameInProgress.svelte";
  import { isGameInstalled } from "$lib/rpc/config";
  import { route, navigate } from "../router";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame.ts";
  import GameBetaAlert from "../components/games/GameBetaAlert.svelte";

  const gameParam = $derived(route.params.game_name);
  let activeGame: SupportedGame | undefined = $state(undefined);
  let modName: string | undefined = $derived(route.params.mod_name);
  let modSource: string | undefined = $derived(route.params.source_name);

  $effect(() => {
    const activeGameFromParam = toSupportedGame(gameParam);
    if (activeGameFromParam) {
      activeGame = activeGameFromParam;
    }
  });
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
