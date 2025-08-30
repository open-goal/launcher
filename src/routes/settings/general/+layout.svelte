<script lang="ts">
  import type { LayoutProps } from "./$types";
  import {
    localeSpecificFontAvailableForDownload,
    resetLauncherSettingsToDefaults,
    setAutoUpdateGames,
    setBypassRequirements,
    setInstallationDirectory,
    setLocale,
    setAutoUninstallOldVersions,
  } from "$lib/rpc/config";
  import {
    Button,
    Helper,
    Input,
    Label,
    Select,
    Spinner,
    Toggle,
  } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import { platform } from "@tauri-apps/plugin-os";
  import { downloadFile } from "$lib/rpc/download";
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import { writable } from "svelte/store";
  import LocaleSelector from "../../../components/LocaleSelector.svelte";
  import { invalidateAll } from "$app/navigation";

  let { data, children }: LayoutProps = $props();
  const config = $derived(data.config);

  let currentInstallationDirectory = "";
  let currentLocale = writable();
  let currentBypassRequirementsVal = false;
  let keepGamesUpdated = writable(false);
  let uninstallOldVersions = writable(false);
  let localeFontForDownload: Locale | undefined = undefined;
  let localeFontDownloading = false;
  let isLinux = platform() === "linux";
  let initialized = false;
  // config.deletePreviousVersions; // getAutouninstallOldVersions

  // localeFontForDownload = await localeSpecificFontAvailableForDownload($currentLocale); // TODO: FIND A BETTER HOME FOR THIS CODE (root route?)
</script>

<div class="flex flex-col gap-5 mt-2">
  <div>
    <LocaleSelector></LocaleSelector>
    <!-- DOWNLOAD LOCALE FONT. TODO: FIND A BETTER HOME FOR THIS -->
    {#if localeFontForDownload !== undefined}
      <Button
        class="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2 mt-2"
        disabled={localeFontDownloading}
        onclick={async () => {
          if (
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
        placeholder={config.installationDir}
        onclick={async () => {
          const newInstallDir = await folderPrompt(
            $_("settings_folders_installationDir_prompt"),
          );
          if (
            newInstallDir !== undefined &&
            newInstallDir !== config.installationDir
          ) {
            await setInstallationDirectory(newInstallDir);
          }
        }}
      />
      <Helper class="text-xs mt-2 italic"
        >{$_("settings_general_installationDir_helper")}</Helper
      >
    </div>
  {/if}
  <div class="*:text-gray-200">
    <Toggle
      color="orange"
      bind:checked={config.autoUpdateGames}
      onchange={async () => {
        $uninstallOldVersions = false;
      }}
      class="mb-2">{$_("settings_general_keep_updated")}</Toggle
    >
    {#if config.autoUpdateGames}
      <Toggle
        color="orange"
        bind:checked={config.deletePreviousVersions}
        class="ml-14 mb-2">{$_("settings_general_uninstall_old")}</Toggle
      >
    {/if}
    <Toggle
      checked={config.requirements.bypassRequirements}
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
            await setBypassRequirements(true);
            await invalidateAll();
          } else {
            // evt.target.checked = false;
          }
        } else {
          await setBypassRequirements(evt.target.checked);
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
          await resetLauncherSettingsToDefaults();
        }
      }}>{$_("settings_general_button_resetSettings")}</Button
    >
  </div>
</div>

{@render children()}
