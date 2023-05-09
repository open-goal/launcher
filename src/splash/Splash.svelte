<script lang="ts">
  import { openMainWindow } from "$lib/rpc/window";
  import { onMount } from "svelte";
  import logo from "$assets/images/icon.webp";
  import { folderPrompt } from "$lib/utils/file";
  import {
    deleteOldDataDirectory,
    getInstallationDirectory,
    getLocale,
    oldDataDirectoryExists,
    setInstallationDirectory,
    setLocale,
  } from "$lib/rpc/config";
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { _ } from "svelte-i18n";

  let currentProgress = 10;
  let currentStatusText = $_("splash_step_readingSettings");

  let selectLocale = false;
  let installationDirSet = true;
  let stepError = undefined;
  let oldDataDirToClean = false;

  // Events
  onMount(async () => {
    // First, see if the user has selected a locale
    await checkLocale();
  });

  // TODO - cleanup this code and make it easier to add steps like with
  // the game setup

  async function checkLocale() {
    const locale = await getLocale();
    if (locale === null) {
      // Prompt the user to select a locale
      selectLocale = true;
    } else {
      // Set locale and continue
      setLocale(locale);
      await checkDirectories();
    }
  }

  async function checkDirectories() {
    currentStatusText = $_("splash_step_checkingDirectories");
    currentProgress = 15;
    // Check to see if the install dir has been setup or not
    const install_dir = await getInstallationDirectory();
    if (install_dir === null) {
      // Check to see if they have the old data directory, ask them if they'd like us to
      // remove it
      currentProgress = 25;
      const hasOldDataDir = await oldDataDirectoryExists();
      if (hasOldDataDir) {
        oldDataDirToClean = true;
      }
      // If not -- let's ask the user to set one up
      installationDirSet = false;
    } else {
      finishSplash();
    }
  }

  async function finishSplash() {
    currentProgress = 50;
    currentStatusText = $_("splash_step_finishingUp");
    await new Promise((res) => setTimeout(res, 1000));
    currentProgress = 100;
    await new Promise((res) => setTimeout(res, 500));
    const errorClosing = await openMainWindow();
    if (!errorClosing) {
      currentStatusText = $_("splash_step_errorOpening");
    }
  }
</script>

<div class="content" data-tauri-drag-region>
  <div class="splash-logo no-pointer-events">
    <img src={logo} alt="" draggable="false" />
  </div>
  <div class="splash-contents no-pointer-events">
    {#if selectLocale}
      <span class="mb-1">{$_("splash_selectLocale")}</span>
      <div class="splash-select">
        <select
          name="cars"
          id="cars"
          class="pointer-events emoji-font"
          on:change={async (evt) => {
            setLocale(evt.target.value);
            selectLocale = false;
            await checkDirectories();
          }}
        >
          <option disabled selected value hidden />
          {#each AVAILABLE_LOCALES as locale}
            <option class="emoji-font" value={locale.id}
              >{locale.flag}&nbsp;{locale.localizedName}</option
            >
          {/each}
        </select>
      </div>
    {:else if oldDataDirToClean}
      {$_("splash_deleteOldInstallDir")}
      <br />
      <span>
        <button
          class="splash-button pointer-events"
          on:click={() => {
            oldDataDirToClean = false;
            deleteOldDataDirectory();
          }}>{$_("splash_button_deleteOldInstallDir_yes")}</button
        >
        <button
          class="splash-button pointer-events"
          on:click={() => {
            oldDataDirToClean = false;
          }}>{$_("splash_button_deleteOldInstallDir_no")}</button
        >
      </span>
    {:else if !installationDirSet}
      {#if stepError !== undefined}
        {stepError}
      {:else}
        {$_("splash_noInstallDirSet")}
      {/if}
      <br />
      <button
        class="splash-button pointer-events"
        on:click={async () => {
          // This is part of what allows for the user to install the games and such wherever they want
          currentStatusText = $_("splash_step_pickInstallFolder");
          currentProgress = 25;
          const newInstallDir = await folderPrompt(
            $_("splash_button_setInstallFolder_prompt")
          );
          if (newInstallDir !== undefined) {
            const result = await setInstallationDirectory(newInstallDir);
            if (result !== null) {
              stepError = result;
            } else {
              installationDirSet = true;
              finishSplash();
            }
          }
        }}>{$_("splash_button_setInstallFolder")}</button
      >
    {:else}
      <div class="splash-status-text">{currentStatusText}</div>
    {/if}
  </div>
  <div class="splash-bar">
    <div
      data-tauri-drag-region
      class="splash-status-bar fg"
      style="width: {currentProgress}%"
    />
    <div data-tauri-drag-region class="splash-status-bar bg" />
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
    font-family: "Twemoji Country Flags", "Roboto Mono", monospace;
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

  .mb-1 {
    margin-bottom: 1rem;
  }

  .emoji-font {
    font-family: "Twemoji Country Flags", "Roboto Mono";
  }
</style>
