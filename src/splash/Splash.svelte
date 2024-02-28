<script lang="ts">
  import { openMainWindow } from "$lib/rpc/window";
  import { onMount } from "svelte";
  import logo from "$assets/images/icon.webp";
  import {
    getInstallationDirectory,
    getLocale,
    oldDataDirectoryExists,
    setLocale,
  } from "$lib/rpc/config";
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { locale as svelteLocale, _ } from "svelte-i18n";
  import { Progressbar } from "flowbite-svelte";
  import LocaleSelector from "./LocaleSelector.svelte";
  import DataDirCleanup from "./DataDirCleanup.svelte";
  import SetInstallDir from "./SetInstallDir.svelte";
  import { P } from "flowbite-svelte";

  let currentProgress = 10;
  let currentStatusText = "Loading Locales...";

  let selectLocale = false;
  let isLocaleSet = false;
  let installationDirSet: boolean | undefined = undefined;
  let stepError: string | undefined = undefined;
  let oldDataDirToClean: boolean | undefined = undefined;

  $: if (isLocaleSet && installationDirSet && !oldDataDirToClean) {
    currentProgress = 100;
    currentStatusText = "Finishing up"; // TODO: i18n
    openMainWindow();
  }

  // Events
  onMount(async () => {
    svelteLocale.set("en-US");
    currentStatusText = $_("splash_step_readingSettings");
    await checkLocale();
    await checkInstallDirectory();
    await checkOldDirectories();
  });

  async function checkLocale() {
    const locale = await getLocale();
    if (locale) {
      setLocale(locale);
      svelteLocale.set(locale);
      isLocaleSet = true;
      return;
    }

    const userDefaultLocale = navigator.language;
    const isDefaultLocaleSupported = AVAILABLE_LOCALES.some(
      (locale) => locale.id === userDefaultLocale,
    );

    if (isDefaultLocaleSupported) {
      svelteLocale.set(userDefaultLocale);
      setLocale(userDefaultLocale);
      isLocaleSet = true;
      return;
    }

    selectLocale = true;
  }

  async function checkInstallDirectory() {
    currentStatusText = $_("splash_step_checkingDirectories");
    currentProgress = 15;
    // Check to see if the install dir has been setup or not
    const install_dir = await getInstallationDirectory();
    if (install_dir) {
      installationDirSet = true;
      currentProgress = 50;
    }
  }

  async function checkOldDirectories() {
    currentStatusText = "Checking for old install directories...";
    oldDataDirToClean = await oldDataDirectoryExists();
  }
</script>

<div class="container max-width text-white py-2" data-tauri-drag-region>
  <div class="flex justify-center h-1/2">
    <img
      src={logo}
      data-testId="splash-logo"
      alt="OpenGOAL logo"
      aria-label="OpenGOAL logo"
      draggable="false"
      data-tauri-drag-region
      class="w-3/5"
    />
  </div>
  <div class="flex flex-col justify-center py-2 px-4" data-tauri-drag-region>
    <P size="sm" weight="medium" align="center" class="mb-1">
      {currentStatusText}
    </P>
    <Progressbar
      animate
      color="yellow"
      progress={currentProgress}
      data-tauri-drag-region
    ></Progressbar>
  </div>
  <div class="flex justify-center flex-col" data-tauri-drag-region>
    {#if selectLocale}
      <LocaleSelector bind:selectLocale bind:isLocaleSet></LocaleSelector>
    {:else if oldDataDirToClean}
      <DataDirCleanup bind:oldDataDirToClean bind:currentStatusText
      ></DataDirCleanup>
    {:else if !installationDirSet}
      <SetInstallDir
        bind:currentProgress
        bind:currentStatusText
        bind:stepError
        bind:installationDirSet
      ></SetInstallDir>
      {#if stepError !== undefined}
        {stepError}
      {/if}
    {/if}
  </div>
</div>
