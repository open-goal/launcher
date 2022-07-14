<script>
  import { fade } from "svelte/transition";
  import { openDir } from "$lib/rpc/commands";
  import { appDir } from "@tauri-apps/api/path";
  import { getVersion } from "@tauri-apps/api/app";

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
  {#await getVersion() then version}
    <p>App Version: {version}</p>
  {/await}
</div>
