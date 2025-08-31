<script lang="ts">
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import IconWindowMinimize from "~icons/mdi/window-minimize";
  import IconWindowClose from "~icons/mdi/window-close";
  import { isInDebugMode } from "$lib/utils/common";
  import {
    downloadOfficialVersion,
    listDownloadedVersions,
    removeOldVersions,
  } from "$lib/rpc/versions";
  import { getLatestOfficialRelease } from "$lib/utils/github";
  import { _ } from "svelte-i18n";
  import { toastStore } from "$lib/stores/ToastStore";
  import { saveActiveVersionChange } from "$lib/rpc/config";
  import { check } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { ask } from "@tauri-apps/plugin-dialog";
  import { invalidateAll } from "$app/navigation";

  let { config } = $props();
  const appWindow = getCurrentWebviewWindow();
  let launcherVerison = $derived("");
  let updateAvailable = $state(false);

  onMount(async () => {
    launcherVerison = `v${await getVersion()}`;
    if (!isInDebugMode()) {
      // Check for a launcher update
      const update = await check();
      if (update) {
        const doUpdate = await ask(
          `${$_("update_versionLabel")}: ${update.version}`,
          {
            title: $_("header_updateAvailable"),
            kind: "info",
          },
        );
        if (doUpdate) {
          await update.downloadAndInstall();
          await relaunch();
        }
      }
    }
    if (!(await isLatestVersionInstalled())) {
      updateAvailable = true;
      await installLatestVersion();
    }
  });

  async function isLatestVersionInstalled() {
    const latest = await getLatestOfficialRelease();
    const downloadedVersions = await listDownloadedVersions();
    return downloadedVersions.includes(latest.version);
  }

  async function installLatestVersion() {
    const latest = await getLatestOfficialRelease();
    if (config.autoUpdateGames) {
      await downloadOfficialVersion(latest);
      await removeOldVersions();
      await saveActiveVersionChange(latest);
      await invalidateAll();
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
      src="/images/icon.webp"
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
      {launcherVerison}
    </p>
    <p class="font-mono text-sm">
      {config.activeVersion === null ? "not set!" : config.activeVersion}
    </p>
  </div>
  {#if updateAvailable}
    <a
      class="font-mono text-sm mt-5 text-orange-500 hover:text-orange-300"
      href="/settings/versions"
      >{$_("header_updateAvailable")}
    </a>
  {/if}
  <div class="flex shrink-0 space-x-4 text-xl ml-auto">
    <button class="hover:text-amber-600" onclick={() => appWindow.minimize()}>
      <IconWindowMinimize />
    </button>
    <button class="hover:text-red-600" onclick={() => appWindow.close()}>
      <IconWindowClose />
    </button>
  </div>
</header>
