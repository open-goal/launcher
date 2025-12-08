<script lang="ts">
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import logo from "$assets/images/icon.webp";
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { Link } from "svelte-navigator";
  import IconWindowMinimize from "~icons/mdi/window-minimize";
  import IconWindowClose from "~icons/mdi/window-close";
  import { UpdateStore } from "$lib/stores/AppStore";
  import { isInDebugMode } from "$lib/utils/common";
  import {
    downloadOfficialVersion,
    getActiveVersion,
    listDownloadedVersions,
    removeOldVersions,
  } from "$lib/rpc/versions";
  import { getLatestOfficialRelease } from "$lib/utils/github";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { _ } from "svelte-i18n";
  import { toastStore } from "$lib/stores/ToastStore";
  import { getAutoUpdateGames, saveActiveVersionChange } from "$lib/rpc/config";
  import { check } from "@tauri-apps/plugin-updater";
  import { warnLog } from "$lib/rpc/logging";

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
      $VersionStore.activeVersionName = version;
      toastStore.makeToast($_("toasts_savedToolingVersion"), "info");
    }
  }

  onMount(async () => {
    // Get current versions
    launcherVersion = `v${await getVersion()}`;
    $VersionStore.activeVersionName = await getActiveVersion();

    // Check for a launcher update
    if (!isInDebugMode()) {
      const update = await check();
      if (update) {
        launcherUpdateAvailable = true;
      }
    }
    await checkIfLatestVersionInstalled();
  });

  async function checkIfLatestVersionInstalled() {
    const latestToolingVersion = await getLatestOfficialRelease();
    if (
      latestToolingVersion !== undefined &&
      $VersionStore.activeVersionName !== latestToolingVersion.version
    ) {
      // Check that we havn't already downloaded it
      let alreadyHaveRelease = false;
      const downloadedOfficialVersions = await listDownloadedVersions();
      for (const releaseVersion of downloadedOfficialVersions) {
        if (releaseVersion === latestToolingVersion.version) {
          alreadyHaveRelease = true;
          break;
        }
      }
      if (!alreadyHaveRelease) {
        let shouldAutoUpdate = await getAutoUpdateGames();
        if (shouldAutoUpdate) {
          await downloadLatestVersion(
            latestToolingVersion.version,
            latestToolingVersion.downloadUrl,
          );
          await removeOldVersions();

          location.reload(); // TODO! this is hacky, when i refactor this will be done automatically
        }

        $UpdateStore.selectedTooling = {
          updateAvailable: true,
          versionNumber: latestToolingVersion.version,
        };
      }
    }
  }
</script>

<header
  class="flex flex-row grow-0 shrink-0 bg-[#101010] pl-2 pr-4 pt-1 pb-1 items-center z-10"
  data-tauri-drag-region
>
  <div
    class="flex flex-row shrink-0 items-center space-x-2 pointer-events-none"
  >
    <img
      class="h-8"
      src={logo}
      alt="OpenGOAL logo"
      aria-label="OpenGOAL logo"
    />
    <p class="font-black text-white tracking-tight text-lg">OpenGOAL</p>
  </div>
  <div class="border-l shrink-0 border-[#9f9f9f] h-8 m-2"></div>
  <div class="flex flex-col shrink-0 text-neutral-500 mr-2 pointer-events-none">
    <p class="font-mono text-sm">{$_("header_launcherVersionLabel")}</p>
    <p class="font-mono text-sm">{$_("header_toolingVersionLabel")}</p>
  </div>
  <div
    class="flex flex-col text-neutral-300 mr-2 pointer-events-none max-w-[250px]"
  >
    <p class="font-mono text-sm">
      {launcherVersion}
    </p>
    <p class="font-mono text-sm">
      {$VersionStore.activeVersionName === null
        ? $_("header_toolingNotSet")
        : $VersionStore.activeVersionName}
    </p>
  </div>
  <div
    class="flex flex-col text-neutral-300 mr-2 pointer-events-none max-w-[250px]"
  >
    <Link
      class={`font-mono text-sm text-orange-500 hover:text-orange-300 ${$UpdateStore.selectedTooling.updateAvailable ? "pointer-events-auto" : "invisible pointer-events-none"}`}
      to="/update/launcher"
      >&gt;&nbsp;{$_("header_updateAvailable")}
    </Link>
    <Link
      class={`font-mono text-sm text-orange-500 hover:text-orange-300  ${$UpdateStore.selectedTooling.updateAvailable ? "pointer-events-auto" : "invisible pointer-events-none"}`}
      to="/settings/versions"
      >&gt;&nbsp;{$_("header_updateAvailable")}
    </Link>
  </div>
  <div class="flex shrink-0 space-x-4 text-xl ml-auto">
    <button class="hover:text-amber-600" on:click={() => appWindow.minimize()}>
      <IconWindowMinimize />
    </button>
    <button class="hover:text-red-600" on:click={() => appWindow.close()}>
      <IconWindowClose />
    </button>
  </div>
</header>
