<script lang="ts">
  import { type Locale } from "$lib/i18n/i18n";
  import {
    localeSpecificFontAvailableForDownload,
    setInstallationDirectory,
    setLocale,
  } from "$lib/rpc/config";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { Button, Input, Label, Spinner, Toggle } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import { platform } from "@tauri-apps/plugin-os";
  import { downloadFile } from "$lib/rpc/download";
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import { writable } from "svelte/store";

  let currentInstallationDirectory = "";
  let currentLocale = writable();
  let keepGamesUpdated = writable(false);
  let uninstallOldVersions = writable(false);
  let localeFontForDownload: Locale | undefined = undefined;
  let localeFontDownloading = false;
  let isLinux = platform() === "linux";
</script>

<div class="flex flex-col gap-5 mt-2">
  <div>
    {#if localeFontForDownload !== undefined}
      <Button
        class="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2 mt-2"
        disabled={localeFontDownloading}
        onclick={async () => {
          if (
            localeFontForDownload !== undefined &&
            localeFontForDownload.fontDownloadUrl !== undefined &&
            localeFontForDownload.fontFileName !== undefined
          ) {
            localeFontDownloading = true;
            const fontPath = await join(
              await appDataDir(),
              "fonts",
              localeFontForDownload.fontFileName,
            );
            await downloadFile(localeFontForDownload.fontDownloadUrl, fontPath);
            await setLocale($currentLocale);
            localeFontForDownload =
              await localeSpecificFontAvailableForDownload($currentLocale);
            localeFontDownloading = false;
          }
        }}
      >
        {#if localeFontDownloading}
          <Spinner class="mr-3" size="4" color="yellow" />
        {/if}
        {$_("settings_general_downloadLocaleSpecificFont")}
      </Button>
    {/if}
  </div>
  {#if !isLinux}
    <div>
      <Label for="default-input" class="block mb-2 text-gray-200"
        >{$_("settings_folders_installationDir")}</Label
      >
      <Input
        id="default-input"
        placeholder={currentInstallationDirectory}
        onclick={async () => {
          const newInstallDir = await folderPrompt(
            $_("settings_folders_installationDir_prompt"),
          );
          if (
            newInstallDir !== undefined &&
            newInstallDir !== currentInstallationDirectory
          ) {
            const errMsg = await setInstallationDirectory(newInstallDir);
            if (errMsg === null) {
              if (currentInstallationDirectory !== newInstallDir) {
                $VersionStore.activeVersionType = null;
                $VersionStore.activeVersionName = null;
              }
              currentInstallationDirectory = newInstallDir;
            }
          }
        }}
      />
    </div>
  {/if}
  <div class="*:text-gray-200">
    <Toggle
      color="orange"
      bind:checked={$keepGamesUpdated}
      onchange={async () => {
        $uninstallOldVersions = false;
      }}
      class="mb-2">{$_("settings_general_keep_updated")}</Toggle
    >
    {#if $keepGamesUpdated}
      <Toggle
        color="orange"
        bind:checked={$uninstallOldVersions}
        class="ml-14 mb-2">{$_("settings_general_uninstall_old")}</Toggle
      >
    {/if}
  </div>
</div>
