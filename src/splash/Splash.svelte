<script lang="ts">
  import { openMainWindow } from "$lib/rpc/window";
  import { onMount } from "svelte";
  import logo from "$assets/images/icon.webp";
  import { folderPrompt } from "$lib/utils/file";
  import {
    deleteOldDataDirectory,
    getInstallationDirectory,
    oldDataDirectoryExists,
    setInstallationDirectory,
  } from "$lib/rpc/config";

  let currentProgress = 10;
  let currentStatusText = "Reading Settings";

  let installationDirSet = true;
  let stepError = undefined;
  let oldDataDirToClean = false;

  // Events
  onMount(async () => {
    currentStatusText = "Checking Directories";
    currentProgress = 15;
    // Check to see if the install dir has been setup or not
    const install_dir = await getInstallationDirectory();
    if (install_dir === null) {
      // Check to see if they have the old data directory, ask them if they'd like us to
      // remove it
      const hasOldDataDir = await oldDataDirectoryExists();
      if (hasOldDataDir) {
        oldDataDirToClean = true;
      }
      installationDirSet = false;
    } else {
      currentProgress = 25;
      finishSplash();
    }
  });

  async function selectInstallationFolder() {
    // If not -- let's ask the user to set one up
    // This is part of what allows for the user to install the games and such wherever they want
    currentStatusText = "Pick an Installation Folder";
    currentProgress = 25;
    const newInstallDir = await folderPrompt("Pick an Installation Folder");
    if (newInstallDir !== undefined) {
      const result = await setInstallationDirectory(newInstallDir);
      if (result !== null) {
        stepError = result;
      } else {
        installationDirSet = true;
        finishSplash();
      }
    }
  }

  async function finishSplash() {
    currentProgress = 50;
    currentStatusText = "Finishing Up";
    await new Promise((res) => setTimeout(res, 1000));
    currentProgress = 100;
    await new Promise((res) => setTimeout(res, 500));
    const errorClosing = await openMainWindow();
    if (!errorClosing) {
      currentStatusText = "Problem opening Launcher";
    }
  }
</script>

<div class="content" data-tauri-drag-region>
  <div class="splash-logo no-pointer-events">
    <img src={logo} alt="" draggable="false" />
  </div>
  <div class="splash-contents no-pointer-events">
    {#if oldDataDirToClean}
      The old installation folder is no longer needed, delete it?
      <br />
      <span>
        <button
          class="splash-button pointer-events"
          on:click={() => {
            oldDataDirToClean = false;
            deleteOldDataDirectory();
          }}>Yes</button
        >
        <button
          class="splash-button pointer-events"
          on:click={() => {
            oldDataDirToClean = false;
          }}>No</button
        >
      </span>
    {:else if !installationDirSet}
      {#if stepError !== undefined}
        {stepError}
      {:else}
        No installation directory set!
      {/if}
      <br />
      <button
        class="splash-button pointer-events"
        on:click={selectInstallationFolder}>Set Install Folder</button
      >
    {:else}
      <div class="splash-status-text">{currentStatusText}</div>
    {/if}
  </div>
  <div class="splash-bar">
    <div class="splash-status-bar fg" style="width: {currentProgress}%" />
    <div class="splash-status-bar bg" />
  </div>
</div>

<style>
  @font-face {
    font-family: "Roboto Mono";
    src: url("/src/assets/fonts/Roboto_Mono/static/RobotoMono-Regular.ttf");
  }
  @font-face {
    font-family: "Roboto Mono";
    src: url("/src/assets/fonts/Roboto_Mono/static/RobotoMono-Bold.ttf");
    font-weight: 700;
  }

  .content {
    color: white;
    height: 100%;
    padding-top: 10px;
    padding-bottom: 10px;
  }

  .splash-contents {
    height: 35%;
    align-items: center;
    justify-content: center;
    font-family: "Roboto Mono", monospace;
    font-size: 10pt;
    text-align: center;
    padding-left: 10px;
    padding-right: 10px;
    display: flex;
    flex-direction: column;
  }

  .splash-bar {
    height: 10%;
  }

  .splash-logo {
    height: 50%;
  }

  .splash-logo img {
    object-fit: contain;
    height: 100%;
    width: 100%;
  }

  .splash-status-bar {
    width: 100%;
    height: 15px;
    margin-top: auto;
  }

  .splash-status-bar.bg {
    background-color: #775500;
    position: absolute;
  }
  .splash-status-bar.fg {
    background-color: #ffb807;
    position: absolute;
    z-index: 999;
  }

  .splash-button {
    margin-top: 5px;
    appearance: none;
    background-color: #ffb807;
    border-radius: 6px;
    box-sizing: border-box;
    color: #000;
    cursor: pointer;
    display: inline-block;
    font-family: "Roboto Mono", monospace;
    font-size: 8pt;
    font-weight: 700;
    position: relative;
    text-align: center;
    text-decoration: none;
    user-select: none;
    -webkit-user-select: none;
    touch-action: manipulation;
    vertical-align: middle;
    white-space: nowrap;
  }

  .splash-button:focus:not(:focus-visible):not(.focus-visible) {
    box-shadow: none;
    outline: none;
  }

  .splash-button:hover {
    background-color: #775500;
  }

  .splash-button:focus {
    outline: none;
  }

  .splash-button:active {
    background-color: #775500;
  }

  .no-pointer-events {
    pointer-events: none;
  }

  .pointer-events {
    pointer-events: auto;
  }
</style>
