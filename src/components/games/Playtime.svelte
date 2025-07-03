<script lang="ts">
  import { getPlaytime } from "$lib/rpc/config";
  import { activeGame } from "$lib/stores/AppStore";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";

  let playtime = "";
  $: ($activeGame, refreshPlaytime());

  async function refreshPlaytime() {
    let playtimeSec = await getPlaytime($activeGame);
    playtime = new Date(playtimeSec * 1000).toISOString().slice(11, 19);
    return playtime;
  }

  onMount(async () => {
    await refreshPlaytime();
  });

  listen<string>("playtimeUpdated", async (e) => {
    await refreshPlaytime();
  });
</script>

<!-- add an option to disable this -->
{#if playtime}
  <h1 class="pb-4 text-xl text-outline tracking-tighter font-extrabold">
    {`${$_(`gameControls_timePlayed_label`)} ${playtime}`}
  </h1>
{/if}
