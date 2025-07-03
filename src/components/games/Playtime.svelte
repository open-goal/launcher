<script lang="ts">
  import { getPlaytime } from "$lib/rpc/config";
  import { activeGame } from "$lib/stores/AppStore";
  import { listen } from "@tauri-apps/api/event";
  import { _ } from "svelte-i18n";

  let playtime = "";
  function formatPlaytime(playtimeSec: string) {
    // TODO: test this snippet that I found on stackoverflow
    return new Date(playtimeSec * 1000).toISOString().slice(11, 19);
    // calculate the number of hours and minutes
    const hours = Math.floor(playtimeRaw / 3600);
    const minutes = Math.floor((playtimeRaw % 3600) / 60);

    // initialize the formatted playtime string
    let formattedPlaytime = "";

    // add the hours to the formatted playtime string
    if (hours > 0) {
      if (hours > 1) {
        formattedPlaytime += `${hours} ${$_(`gameControls_timePlayed_hours`)}`;
      } else {
        formattedPlaytime += `${hours} ${$_(`gameControls_timePlayed_hour`)}`;
      }
    }

    // add the minutes to the formatted playtime string
    if (minutes > 0) {
      // add a comma if there are already hours in the formatted playtime string
      if (formattedPlaytime.length > 0) {
        formattedPlaytime += ", ";
      }
      if (minutes > 1) {
        formattedPlaytime += `${minutes} ${$_(
          `gameControls_timePlayed_minutes`,
        )}`;
      } else {
        formattedPlaytime += `${minutes} ${$_(
          `gameControls_timePlayed_minute`,
        )}`;
      }
    }
    return formattedPlaytime;
  }

  // listen for the custom playtimeUpdated event from the backend and then refresh the playtime on screen
  listen<string>("playtimeUpdated", async (e) => {
    playtime = await getPlaytime($activeGame);
    playtime = formatPlaytime(playtime);
  });
</script>

{#if playtime}
  <h1 class="pb-4 text-xl text-outline tracking-tighter font-extrabold">
    {`${$_(`gameControls_timePlayed_label`)} ${playtime}`}
  </h1>
{/if}
