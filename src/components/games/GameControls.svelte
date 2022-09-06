<script type="ts">
  import { launcherConfig } from "$lib/config";
  import { getInternalName, SupportedGame } from "$lib/constants";
  import { launchGame } from "$lib/launch";
  import { openDir } from "$lib/rpc/commands";
  import { compileGame, decompileGameData } from "$lib/setup/setup";
  import { appDir, configDir, join } from "@tauri-apps/api/path";
  import { createEventDispatcher, onMount } from "svelte";
  import LogViewer from "./setup/LogViewer.svelte";
  import {
    isCompiling,
    isDecompiling,
    ProcessLogs,
  } from "$lib/stores/AppStore";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();
  let componentLoaded = false;
  let configPath = undefined;

  onMount(async () => {
    configPath = await join(
      await configDir(),
      "OpenGOAL",
      getInternalName(activeGame)
    );
    componentLoaded = true;
  });

  function onClickPlay() {
    launchGame();
  }

  async function onClickUninstall() {
    const confirmed = await confirm("Are you sure you want to uninstall?");
    if (confirmed) {
      await launcherConfig.setInstallStatus(activeGame, false);
      dispatch("change");
    }
  }

  async function onClickDecompile() {
    ProcessLogs.update(() => "");
    const isoPath = await join(
      await appDir(),
      "data",
      "iso_data",
      getInternalName(activeGame)
    );
    await decompileGameData(isoPath);
  }

  async function onClickCompile() {
    ProcessLogs.update(() => "");
    const isoPath = await join(
      await appDir(),
      "data",
      "iso_data",
      getInternalName(activeGame)
    );
    await compileGame(isoPath);
  }
</script>

{#if componentLoaded}
  <div id="launcherControls">
    <button
      class="btn lg"
      on:click={onClickPlay}
      disabled={$isDecompiling || $isCompiling}>Play</button
    >
    <div class="mt-1">
      <button class="btn md" on:click={() => openDir(configPath)}
        >Settings and Saves</button
      >
      <button
        class="btn md"
        on:click={onClickDecompile}
        disabled={$isDecompiling || $isCompiling}>Decompile</button
      >
      <button
        class="btn md"
        on:click={onClickCompile}
        disabled={$isCompiling || $isDecompiling}>Compile</button
      >
      <button
        class="btn md"
        on:click={onClickUninstall}
        disabled={$isDecompiling || $isCompiling}>Uninstall</button
      >
    </div>
    {#if $isDecompiling || $isCompiling}
      <!-- TODO - some sort of spinner component instead -->
      <div class="mt-1">Please Wait</div>
    {/if}
    {#if $ProcessLogs}
      <LogViewer />
    {/if}
  </div>
{/if}
