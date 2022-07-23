<script type="ts">
  import { launcherConfig } from "$lib/config";
  import { getInternalName, SupportedGame } from "$lib/constants";
  import { launchGame } from "$lib/launch";
  import { openDir } from "$lib/rpc/commands";
  import { compileGame, decompileGameData } from "$lib/setup/setup";
  import { appDir, configDir, join } from "@tauri-apps/api/path";
import { createEventDispatcher, onMount } from "svelte";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();
  let componentLoaded = false;
  let configPath = undefined;
  let gameVersion = undefined;

  onMount(async () => {
    gameVersion = await launcherConfig.getGameInstallVersion(activeGame);
    configPath = await join(await configDir(), "OpenGOAL", getInternalName(activeGame));
    componentLoaded = true;
  });

  function onClickPlay() {
    launchGame();
  }

  async function onClickUninstall() {
    await launcherConfig.setInstallStatus(SupportedGame.Jak1, false);
    dispatch('change');
  }

  async function onClickDecompile() {
    // TODO - ensure this path is correct!
    const isoPath = await join(await appDir(), "data", "extracted_iso");
    await decompileGameData(isoPath);
  }

  async function onClickCompile() {
    // TODO - ensure this path is correct!
    const isoPath = await join(await appDir(), "data", "extracted_iso");
    await compileGame(isoPath);
  }
</script>

{#if componentLoaded}
<div id="launcherControls">
  <button class="btn lg" on:click={onClickPlay}>Play</button>
  <p class="text-shadow">Game Version: {gameVersion}</p>
  <!-- TODO - when clicking decompile/compile -- show logs -->
  <div class="mt-1">
    <button class="btn md" on:click={() => openDir(configPath)}>Settings and Saves</button>
    <button class="btn md" on:click={onClickDecompile}>Decompile</button>
    <button class="btn md" on:click={onClickCompile}>Compile</button>
    <button class="btn md" on:click={onClickUninstall}>Uninstall</button>
  </div>
</div>
{/if}

<style>
  .text-shadow {
    text-shadow: -1px -1px 0 #000, 1px -1px 0 #000, -1px 1px 0 #000, 1px 1px 0 #000;
  }
</style>
