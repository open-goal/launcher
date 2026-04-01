<script lang="ts">
  import GameControls from "../components/games/GameControls.svelte";
  import { _ } from "svelte-i18n";
  import GameControlsMod from "../components/games/GameControlsMod.svelte";
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
    <!-- Jak 2 & Jak 3 BETA warning -->
    <!-- Not shown on mod pages because mod bugs shouldn't be reported to the jak-projects repo -->
    {#if (activeGame === "jak2" || activeGame === "jak3") && !modName}
      <GameBetaAlert {activeGame}></GameBetaAlert>
    {/if}
    {#if modName && modSource}
      <GameControlsMod {activeGame} {modName} {modSource} />
    {:else}
      <GameControls {activeGame} />
    {/if}
  </div>
{/if}
