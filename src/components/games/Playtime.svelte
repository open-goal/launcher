<script lang="ts">
  import { _ } from "svelte-i18n";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { config } from "/src/state/config.svelte";

  let { activeGame }: { activeGame: SupportedGame } = $props();
  const playtime = $derived.by(() => {
    const playtimeSec = config?.games?.[activeGame]?.secondsPlayed!;
    return formatPlaytime(playtimeSec);
  });

  function formatPlaytime(totalSeconds: any): string {
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    return [hours, minutes, seconds]
      .map((value) => value.toString().padStart(2, "0"))
      .join(":");
  }
</script>

<!-- TODO: add an option to disable this -->
{#if playtime}
  <p class="text-sm font-normal tracking-wide text-gray-400">
    {`${$_(`gameControls_timePlayed_label`)} ${playtime}`}
  </p>
{/if}
