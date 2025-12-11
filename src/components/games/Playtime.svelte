<script lang="ts">
  import { getPlaytime } from "$lib/rpc/config";
  import { listen } from "@tauri-apps/api/event";
  import { _ } from "svelte-i18n";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";

  let { activeGame }: { activeGame: SupportedGame } = $props();
  let playtime = $state("");

  listen<string>("playtimeUpdated", async (e) => {
    await refreshPlaytime();
  });

  async function refreshPlaytime() {
    let playtimeSec = await getPlaytime(activeGame);
    playtime = new Date(playtimeSec * 1000).toISOString().slice(11, 19);
  }
</script>

<!-- add an option to disable this -->
{#if playtime}
  <h1 class="pb-4 text-xl text-outline tracking-tighter font-extrabold">
    {`${$_(`gameControls_timePlayed_label`)} ${playtime}`}
  </h1>
{/if}
