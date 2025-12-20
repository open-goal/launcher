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
  import { downloadFile } from "$lib/rpc/download";
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import { versionState } from "/src/state/VersionState.svelte";

  let currentLocale: string | null = $state(null);
  let currentInstallationDirectory: string | null = $state(null);
  let keepGamesUpdated: boolean = $state(false);
  let uninstallOldVersions: boolean = $state(false);
  let currentBypassRequirementsVal = $state(false);
  let availableLocales: LocaleOption[] = $state([]);
  let localeFontForDownload: Locale | undefined = $state(undefined);
  let localeFontDownloading = $state(false);
  interface LocaleOption {
    value: string;
    name: string;
  }

  onMount(async () => {
    currentLocale = await getLocale();
    currentInstallationDirectory = await getInstallationDirectory();
    keepGamesUpdated = await getAutoUpdateGames();
    uninstallOldVersions = await getAutoUninstallOldVersions();
    currentBypassRequirementsVal = await getBypassRequirements();

    for (const locale of AVAILABLE_LOCALES) {
      availableLocales = [
        ...availableLocales,
        {
          value: locale.id,
          name: `${locale.flag} ${locale.localizedName}`,
        },
      ];
    }

    if (currentLocale !== null) {
      localeFontForDownload =
        await localeSpecificFontAvailableForDownload(currentLocale);
    }
  });
</script>

<div class="flex flex-col gap-5 mt-2">
  <div>
    <Label class="text-gray-200"
      >{$_("settings_general_localeChange")}
      <Select
        class="mt-2"
        items={availableLocales}
        bind:value={currentLocale}
        onchange={async () => {
          if (currentLocale !== null) {
            await setLocale(currentLocale);
            localeFontForDownload =
              await localeSpecificFontAvailableForDownload(currentLocale);
          }
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
            localeFontForDownload.fontFileName !== undefined &&
            currentLocale !== null
          ) {
            localeFontDownloading = true;
            const fontPath = await join(
              await appDataDir(),
              "fonts",
              localeFontForDownload.fontFileName,
            );
            await downloadFile(localeFontForDownload.fontDownloadUrl, fontPath);
            await setLocale(currentLocale);
            localeFontForDownload =
              await localeSpecificFontAvailableForDownload(currentLocale);
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
              versionState.activeToolingVersion = undefined;
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
  <div class="*:text-gray-200">
    <Toggle
      color="orange"
      bind:checked={keepGamesUpdated}
      onchange={async () => {
        if (!keepGamesUpdated) {
          uninstallOldVersions = false;
        }
        setAutoUpdateGames(keepGamesUpdated);
      }}
      class="mb-2">{$_("settings_general_keep_updated")}</Toggle
    >
    {#if keepGamesUpdated}
      <Toggle
        color="orange"
        bind:checked={uninstallOldVersions}
        onchange={async () => {
          setAutoUninstallOldVersions(uninstallOldVersions);
        }}
        class="ml-14 mb-2">{$_("settings_general_uninstall_old")}</Toggle
      >
    {/if}
    <Toggle
      bind:checked={currentBypassRequirementsVal}
      color="orange"
      onchange={async (evt) => {
        if (evt.currentTarget.checked) {
          const confirmed = await confirm(
            `${$_("requirements_button_bypass_warning_1")}\n\n${$_(
              "requirements_button_bypass_warning_2",
            )}`,
            { title: "OpenGOAL Launcher", kind: "warning" },
          );
          if (confirmed) {
            await setBypassRequirements(evt.currentTarget.checked);
            currentBypassRequirementsVal = await getBypassRequirements();
          } else {
            currentBypassRequirementsVal = false;
          }
        } else {
          await setBypassRequirements(evt.currentTarget.checked);
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
          const result = await resetLauncherSettingsToDefaults();
          if (result) {
            versionState.activeToolingVersion = await getActiveVersion();
          }
        }
      }}>{$_("settings_general_button_resetSettings")}</Button
    >
  </div>
</div>
