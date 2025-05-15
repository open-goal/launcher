<script lang="ts">
  import { AVAILABLE_LOCALES, type Locale } from "$lib/i18n/i18n";
  import {
    getBypassRequirements,
    getInstallationDirectory,
    getLocale,
    localeSpecificFontAvailableForDownload,
    resetLauncherSettingsToDefaults,
    setAutoUpdateGames,
    getAutoUpdateGames,
    setBypassRequirements,
    setInstallationDirectory,
    setLocale,
    setAutoUninstallOldVersions,
    getAutoUninstallOldVersions,
  } from "$lib/rpc/config";
  import { getActiveVersion } from "$lib/rpc/versions";
  import { VersionStore } from "$lib/stores/VersionStore";
  import {
    Button,
    Helper,
    Input,
    Label,
    Select,
    Spinner,
    Toggle,
  } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import { platform } from "@tauri-apps/plugin-os";
  import { downloadFile } from "$lib/rpc/download";
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import { writable } from "svelte/store";

  let currentInstallationDirectory = "";
  let currentLocale = writable();
  let availableLocales = [];
  let currentBypassRequirementsVal = false;
  let keepGamesUpdated = writable(false);
  let uninstallOldVersions = writable(false);
  let localeFontForDownload: Locale | undefined = undefined;
  let localeFontDownloading = false;
  let isLinux = platform() === "linux";
  let initialized = false;

  onMount(async () => {
    keepGamesUpdated.set(await getAutoUpdateGames());
    uninstallOldVersions.set(await getAutoUninstallOldVersions());
    currentInstallationDirectory = await getInstallationDirectory();
    for (const locale of AVAILABLE_LOCALES) {
      availableLocales = [
        ...availableLocales,
        {
          value: locale.id,
          name: `${locale.flag} ${locale.localizedName}`,
        },
      ];
    }
    $currentLocale = await getLocale();
    currentBypassRequirementsVal = await getBypassRequirements();
    if ($currentLocale !== null) {
      localeFontForDownload =
        await localeSpecificFontAvailableForDownload($currentLocale);
    }
    initialized = true;
  });

  $: if (initialized) {
    setAutoUpdateGames($keepGamesUpdated);
  }

  $: if (initialized) {
    setAutoUninstallOldVersions($uninstallOldVersions);
  }
</script>

<div class="flex flex-col gap-5 mt-2">
  <div>
    <Label
      >{$_("settings_general_localeChange")}
      <Select
        class="mt-2"
        items={availableLocales}
        bind:value={$currentLocale}
        onchange={async () => {
          await setLocale($currentLocale);
          localeFontForDownload =
            await localeSpecificFontAvailableForDownload($currentLocale);
        }}
      />
    </Label>
    <Helper class="text-xs mt-2 italic"
      >{$_("settings_general_localeChange_helper_1")}
      <a
        class=" text-orange-400 hover:text-orange-600"
        href="https://crowdin.com/project/opengoal-launcher"
        target="_blank"
        rel="noreferrer">{$_("settings_general_localeChange_helper_link")}</a
      >
      {$_("settings_general_localeChange_helper_2")}</Helper
    >
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
          <Spinner class="mr-3" size="4" color="white" />
        {/if}
        {$_("settings_general_downloadLocaleSpecificFont")}
      </Button>
    {/if}
  </div>
  {#if !isLinux}
    <div>
      <Label for="default-input" class="block mb-2"
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
      <Helper class="text-xs mt-2 italic"
        >{$_("settings_general_installationDir_helper")}</Helper
      >
    </div>
  {/if}
  <div>
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
    <Toggle
      checked={currentBypassRequirementsVal}
      color="orange"
      onchange={async (evt) => {
        if (evt.target.checked) {
          const confirmed = await confirm(
            `${$_("requirements_button_bypass_warning_1")}\n\n${$_(
              "requirements_button_bypass_warning_2",
            )}`,
            { title: "OpenGOAL Launcher", type: "warning" },
          );
          if (confirmed) {
            await setBypassRequirements(evt.target.checked);
            currentBypassRequirementsVal = await getBypassRequirements();
          } else {
            evt.target.checked = false;
          }
        } else {
          await setBypassRequirements(evt.target.checked);
          currentBypassRequirementsVal = await getBypassRequirements();
        }
      }}>{$_("settings_general_toggle_bypassRequirementsCheck")}</Toggle
    >
  </div>
  <div>
    <Button
      class="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2"
      onclick={async () => {
        const confirmed = await confirm(
          $_("settings_general_button_resetSettings_confirmation"),
        );
        if (confirmed) {
          const result = resetLauncherSettingsToDefaults();
          if (result) {
            // TODO - move these to a store method
            $VersionStore.activeVersionName = await getActiveVersion();
          }
        }
      }}>{$_("settings_general_button_resetSettings")}</Button
    >
  </div>
</div>
