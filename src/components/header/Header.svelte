<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import logo from "$assets/images/icon.png";
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { Link } from "svelte-navigator";
  import Icon from "@iconify/svelte";
  import { UpdateStore } from "$lib/stores/AppStore";
  import { checkUpdate } from "@tauri-apps/api/updater";
  import { isInDebugMode } from "$lib/utils/common";
  import {
    getActiveVersion,
    getActiveVersionFolder,
    listDownloadedVersions,
  } from "$lib/rpc/versions";
  import { getLatestOfficialRelease } from "$lib/utils/github";
  import { listen } from "@tauri-apps/api/event";

  let launcherVerison = undefined;
  let toolingVersion = undefined;

  onMount(async () => {
    // Get current versions
    launcherVerison = `v${await getVersion()}`;
    toolingVersion = await getActiveVersion();

    // Check for a launcher update
    // TODO - skip this while doing local development
    // I think it won't work unless the updater is in the configuration, which of course has other issues
    if (!isInDebugMode()) {
      const updateResult = await checkUpdate();
      if (updateResult.shouldUpdate) {
        $UpdateStore.launcher = {
          updateAvailable: true,
          versionNumber: updateResult.manifest.version,
          date: updateResult.manifest.date,
          changeLog: JSON.parse(updateResult.manifest.body),
        };
        console.log("OG: Launcher Update Available");
      } else {
        $UpdateStore.launcher = {
          updateAvailable: false,
          versionNumber: undefined,
          date: undefined,
          changeLog: [],
        };
        console.log("OG: Launcher is up to date");
      }
    }

    // Check for an update to the tooling (right now, only if it's official)
    const selectedVersionFolder = await getActiveVersionFolder();
    if (selectedVersionFolder === "official") {
      const latestToolingVersion = await getLatestOfficialRelease();
      if (toolingVersion !== latestToolingVersion.version) {
        // Check that we havn't already downloaded it
        let alreadyHaveRelease = false;
        const downloadedOfficialVersions = await listDownloadedVersions(
          "official"
        );
        for (const releaseVersion of downloadedOfficialVersions) {
          if (releaseVersion === toolingVersion) {
            alreadyHaveRelease = true;
            break;
          }
        }
        if (!alreadyHaveRelease) {
          $UpdateStore.selectedTooling = {
            updateAvailable: true,
            versionNumber: latestToolingVersion.version,
          };
        }
      }
    }

    const unlistenInstalled = await listen(
      "toolingVersionChanged",
      async (event) => {
        toolingVersion = await getActiveVersion();
      }
    );
  });
</script>

<header
  class="flex flex-row bg-[#101010] pl-2 pr-4 pt-1 pb-1 items-center"
  data-tauri-drag-region
>
  <div class="flex flex-row items-center space-x-2 pointer-events-none">
    <img class="h-8" src={logo} alt="" />
    <p class="font-black text-white tracking-tight text-lg">OpenGOAL</p>
  </div>

  <div class="border-l border-[#9f9f9f] h-8 m-2" />

  <div class="flex flex-col text-neutral-500 mr-2 pointer-events-none">
    <p class="font-mono text-sm">Launcher</p>
    <p class="font-mono text-sm">Tooling</p>
  </div>

  <div class="flex flex-col text-neutral-300 mr-2 pointer-events-none">
    <p class="font-mono text-sm">
      {launcherVerison === undefined ? "unknown" : launcherVerison}
    </p>
    <p class="font-mono text-sm">
      {toolingVersion === undefined ? "unknown" : toolingVersion}
    </p>
  </div>

  <div class="flex flex-col text-orange-500 pointer-events-none">
    <p
      class="font-mono text-sm hover:text-orange-300 {$UpdateStore.launcher
        .updateAvailable
        ? 'pointer-events-auto'
        : 'invisible pointer-events-none'}"
    >
      <Link class="font-mono" to="/update">> Update Available!</Link>
    </p>
    <p
      class="font-mono text-sm hover:text-orange-300 {$UpdateStore
        .selectedTooling.updateAvailable
        ? 'pointer-events-auto'
        : 'invisible pointer-events-none'}"
    >
      <Link class="font-mono " to="/settings/versions">> Update Available!</Link
      >
    </p>
  </div>

  <div class="flex space-x-4 text-xl ml-auto">
    <button class="hover:text-amber-600" on:click={() => appWindow.minimize()}>
      <Icon icon="material-symbols:chrome-minimize" width="24" height="24" />
    </button>
    <button class="hover:text-red-600" on:click={() => appWindow.close()}>
      <Icon icon="ic:baseline-close" width="24" height="24" />
    </button>
  </div>
</header>
