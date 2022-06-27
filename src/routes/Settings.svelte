<script>
  import { fade } from "svelte/transition";
  import { openDir } from "$lib/commands";
  import { appDir } from "@tauri-apps/api/path";
  import { getGameInstallVersion } from "$lib/config";
  import { SupportedGame } from "$lib/constants";
  import { getVersion } from "@tauri-apps/api/app";

  // NOTE: added this to shut up the console warning
  export let location;
</script>

<!-- TODO - STYLE THIS PAGE -->
<div class="flex-center" in:fade>
  <h1>Settings</h1>
  {#await appDir() then directory}
    <button class="btn" on:click={() => openDir(directory)}>
      <i class="bi bi-folder" />
      Open App Directory</button
    >
  {/await}
  {#await getGameInstallVersion(SupportedGame.Jak1) then version}
    <p>Game Version: {version}</p>
  {/await}
  {#await getVersion() then version}
    <p>App Version: {version}</p>
  {/await}
</div>
