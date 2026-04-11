<script lang="ts">
  import { getPlaytime } from "$lib/rpc/config";
  import { listen } from "@tauri-apps/api/event";
  import { _ } from "svelte-i18n";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { onMount } from "svelte";

  let { activeGame }: { activeGame: SupportedGame } = $props();
  let playtime = $state("");

  onMount(() => {
    refreshPlaytime();

    const unlistenPromise = listen("playtimeUpdated", async () => {
      await refreshPlaytime();
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  });

  function formatPlaytime(totalSeconds: number): string {
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    return [hours, minutes, seconds]
      .map((value) => value.toString().padStart(2, "0"))
      .join(":");
  }

  async function refreshPlaytime() {
    const playtimeSec = await getPlaytime(activeGame);
    playtime = formatPlaytime(playtimeSec);
  }
</script>

<!-- add an option to disable this -->
{#if playtime}
  <h1 class="pb-4 text-xl text-outline tracking-tighter font-extrabold">
    {`${$_(`gameControls_timePlayed_label`)} ${playtime}`}
  </h1>
{/if}
