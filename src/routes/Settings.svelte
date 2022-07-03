<script>
  import { fade } from "svelte/transition";
  import { openDir } from "$lib/commands";
  import { appDir, join } from "@tauri-apps/api/path";
  import { getGameInstallVersion } from "$lib/config";
  import { SupportedGame } from "$lib/constants";
  import { getVersion } from "@tauri-apps/api/app";
  import { decompileGameData, compileGame } from "$lib/setup/setup";
  import { onMount } from "svelte";
  import { isInstalling } from "../stores/InstallStore";

  // NOTE: added this to shut up the console warning
  export let location;
  let isoPath;
  onMount(async () => {
    isoPath = await join(await appDir(), "/data/extracted_iso/");
  });
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
  <button
    disabled={$isInstalling}
    class="btn"
    on:click={async () => decompileGameData(isoPath)}>Decompile</button
  >
  <button
    disabled={$isInstalling}
    class="btn"
    on:click={async () => compileGame(isoPath)}>Compile</button
  >
  {#await getGameInstallVersion(SupportedGame.Jak1) then version}
    <p>Game Version: {version}</p>
  {/await}
  {#await getVersion() then version}
    <p>App Version: {version}</p>
  {/await}
</div>
