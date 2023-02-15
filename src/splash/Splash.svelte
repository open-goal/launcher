<script>
  import { closeSplashScreen } from "$lib/rpc/commands";
  import { onMount } from "svelte";
  import logo from "$assets/images/icon.webp";
  import {
    copyDataDirectory,
    dataDirectoryExists,
  } from "$lib/utils/data-files";
  import { log } from "$lib/utils/log";
  import { invoke } from "@tauri-apps/api/tauri";
  import { folderPrompt } from "$lib/utils/file";

  let currentProgress = 10;
  let currentStatusText = "Reading Settings";

  let installationDirSet = true;

  // Events
  onMount(async () => {
    // Check to see if the install dir has been setup or not
    const install_dir = await invoke("get_install_directory");
    if (install_dir === null) {
      installationDirSet = false;
    } else {
      finishSplash();
    }
  });

  async function selectInstallationFolder(evt) {
      // If not -- let's ask the user to set one up
      // This is part of what allows for the user to install the games and such wherever they want
      currentStatusText = "Pick an Installation Folder";
      // TODO - change to a save dialog instead
      const new_install_dir = await folderPrompt("Pick an Installation Folder");
      // TODO - put invokes into a nice typescript interface
      if (new_install_dir !== undefined) {
        await invoke("set_install_directory", {"newDir": new_install_dir});
        // TODO - we are kinda assuming it succeeded here, improve that
        // - what if the install directory no longer exists
        // - what if what they provide isn't writable?
        installationDirSet = true;
        finishSplash();
      }
  }

  async function finishSplash(evt) {
    currentStatusText = "Checking Data Files";
    currentProgress = 25;
    // TODO - this can likely go away
    // Check if the data directory exists or not
    // - if it doesn't this is the users first launch, so lets copy it
    if (!(await dataDirectoryExists())) {
      try {
        currentStatusText = "Copying Data Files";
        currentProgress = 50;
        await copyDataDirectory();
      } catch (err) {
        log.error("error encountered when copying data files", {
          error: err,
        });
        currentStatusText = `Error - ${err}`;
        return;
      }
    }
    currentStatusText = "Finishing Up";
    currentProgress = 100;
    await new Promise((res) => setTimeout(res, 2000));
    await closeSplashScreen();
  }
</script>

<div class="content" data-tauri-drag-region>
  <div class="splash-logo">
    <img src={logo} alt="" draggable="false" />
  </div>
  <div class="splash-contents">
    {#if installationDirSet}
      <div class="splash-status-text">{currentStatusText}</div>
    {:else}
      <button class="splash-button" on:click={selectInstallationFolder}>Select Install Folder</button>
    {/if}
  </div>
  <div>
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
  }

  .splash-contents {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 1.5em;
  }

  .splash-logo {
    height: 65vh;
    margin-bottom: 1em;
    padding: 10px;
  }

  .splash-logo img {
    object-fit: contain;
    height: 100%;
    width: 100%;
  }

  .splash-status-text {
    text-align: center;
    font-family: "Roboto Mono", monospace;
    font-size: 8pt;
  }

  .splash-status-bar {
    width: 100%;
    height: 15px;
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
</style>
