<script lang="ts">
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import logo from "$assets/images/icon.webp";
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import IconWindowMinimize from "~icons/mdi/window-minimize";
  import IconHelpCircle from "~icons/mdi/help-circle";
  import IconWindowClose from "~icons/mdi/window-close";
  import IconUpArrowThick from "~icons/mdi/arrow-up-thick";
  import { UpdateStore } from "$lib/stores/AppStore";
  import { isInDebugMode } from "$lib/utils/common";
  import {
    downloadOfficialVersion,
    listDownloadedVersions,
    removeOldVersions,
  } from "$lib/rpc/versions";
  import { getLatestOfficialRelease } from "$lib/utils/github";
  import { _ } from "svelte-i18n";
  import { toastStore } from "$lib/stores/ToastStore";
  import { getAutoUpdateGames, saveActiveVersionChange } from "$lib/rpc/config";
  import { check } from "@tauri-apps/plugin-updater";
  import { warnLog } from "$lib/rpc/logging";
  import { versionState } from "/src/state/VersionState.svelte";
  import { navigate } from "/src/router";
  import { Tooltip } from "flowbite-svelte";

  let launcherVersion: string | null = null;
  let launcherUpdateAvailable = false;
  const appWindow = getCurrentWebviewWindow();

  async function downloadLatestVersion(
    version: string,
    url: String | undefined,
  ) {
    if (url === undefined) {
      warnLog("can't download latest version with an undefined url");
      return;
    }
    await downloadOfficialVersion(version, url);
    $UpdateStore.selectedTooling.updateAvailable = false;
    await saveOfficialVersionChange(version);
  }

  async function saveOfficialVersionChange(version: string) {
    const success = await saveActiveVersionChange(version);
    if (success) {
      versionState.activeToolingVersion = version;
      toastStore.makeToast($_("toasts_savedToolingVersion"), "info");
    }
  }

  onMount(async () => {
    // Get current versions
    launcherVersion = `v${await getVersion()}`;

    // Check for a launcher update
    if (!isInDebugMode()) {
      const update = await check();
      if (update) {
        launcherUpdateAvailable = true;
      }
    }
    await checkForToolingUpdate();
  });

  async function checkForToolingUpdate() {
    const latestToolingVersion = await getLatestOfficialRelease();
    const latestVersion = latestToolingVersion?.version;

    if (!latestVersion) return;
    if (versionState.activeToolingVersion === latestVersion) return;

    // Check that we havn't already downloaded it
    const downloadedOfficialVersions = await listDownloadedVersions();
    if (downloadedOfficialVersions.includes(latestVersion)) return;

    $UpdateStore.selectedTooling = {
      updateAvailable: true,
      versionNumber: latestVersion,
    };
  }

  // TODO: handle this in the new startup route
  // download the latest `jak-project` binaries
  async function autoUpdateTooling() {
    const shouldAutoUpdate = await getAutoUpdateGames();
    if (!shouldAutoUpdate) return;

    const latestToolingVersion = await getLatestOfficialRelease();
    const latestVersion = latestToolingVersion?.version;

    if (!latestVersion) return;

    await downloadLatestVersion(
      latestToolingVersion.version,
      latestToolingVersion.downloadUrl,
    );
    // delete the existing `jak-project` binaries
    await removeOldVersions();
  }
</script>

<header
  class="flex shrink-0 h-8 items-center bg-[#101010] pl-2 z-10"
  data-tauri-drag-region
>
  <div class="flex shrink-0 items-center gap-2 pointer-events-none">
    <img
      class="h-6"
      src={logo}
      alt="OpenGOAL logo"
      aria-label="OpenGOAL logo"
    />
    <p class="text-base font-black tracking-tight text-white">OpenGOAL</p>
  </div>

  <div class="mx-2 h-6 shrink-0 border-l border-neutral-800"></div>

  <div class="flex min-w-0 items-center gap-2 pointer-events-none">
    <div class="flex items-center gap-1 font-mono text-sm whitespace-nowrap">
      <span class="text-neutral-400">{$_("header_launcherVersionLabel")}</span>
      <span class="text-neutral-500">{launcherVersion}</span>
      {#if launcherUpdateAvailable}
        <a
          class="pointer-events-auto text-orange-500 hover:text-orange-300 animate-pulse relative -top-[1px]"
          href="/update/launcher"
        >
          <IconUpArrowThick />
        </a>
        <Tooltip type="auto" trigger="hover">
          {$_("header_updateAvailable")}
        </Tooltip>
      {/if}
    </div>

    <div class="h-6 shrink-0 border-l border-neutral-800"></div>

    <div class="flex items-center gap-1 font-mono text-sm whitespace-nowrap">
      <span class="text-neutral-400">
        {#if versionState.displayModVersion}
          {$_("header_modVersionLabel")}
        {:else}
          {$_("header_toolingVersionLabel")}
        {/if}
      </span>

      <span class="text-neutral-500">
        {#if versionState.displayModVersion}
          {#if !versionState.activeModVersionInfo.installed}
            {$_("header_modNotInstalled")}
          {:else if versionState.activeModVersionInfo.installedVersion}
            {versionState.activeModVersionInfo.installedVersion}
          {:else}
            <span>&nbsp;</span>
          {/if}
        {:else}
          {!versionState.activeToolingVersion
            ? $_("header_toolingNotSet")
            : versionState.activeToolingVersion}
        {/if}
      </span>

      {#if $UpdateStore.selectedTooling.updateAvailable}
        <a
          class="pointer-events-auto text-orange-500 hover:text-orange-300 animate-pulse relative -top-[1px]"
          href="/settings/versions"
        >
          <IconUpArrowThick />
        </a>
        <Tooltip type="auto" trigger="hover">
          {$_("header_updateAvailable")}
        </Tooltip>
      {/if}
    </div>
  </div>

  <div class="ml-auto flex shrink-0 self-stretch">
    <button
      class="flex h-full w-12 items-center justify-center text-neutral-200 hover:bg-white/10"
      on:click={() => navigate("/help")}
      aria-label="Help"
    >
      <IconHelpCircle />
    </button>
    <Tooltip placement="bottom" type="auto" trigger="hover"
      >{$_("sidebar_help")}</Tooltip
    >

    <div class="h-6 my-auto shrink-0 border-l border-neutral-800"></div>

    <button
      class="flex h-full w-12 items-center justify-center hover:bg-white/10"
      on:click={() => appWindow.minimize()}
      aria-label="Minimize"
    >
      <IconWindowMinimize />
    </button>

    <button
      class="flex h-full w-12 items-center justify-center hover:bg-red-600"
      on:click={() => appWindow.close()}
      aria-label="Close"
    >
      <IconWindowClose />
    </button>
  </div>
</header>
