<script lang="ts">
  import type { PageProps } from "./$types";
  import {
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
    Spinner,
    Toggle,
  } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import { confirm, message } from "@tauri-apps/plugin-dialog";
  import { platform } from "@tauri-apps/plugin-os";
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import LocaleSelector from "../../../components/LocaleSelector.svelte";
  import { invalidateAll } from "$app/navigation";

  let { data }: PageProps = $props();
  const config = $derived(data.config);
  const localeFont = $derived(data.localeFont);

  let localeFontDownloading = false;

  let isLinux = platform() === "linux";
  // config.deletePreviousVersions; // getAutouninstallOldVersions
</script>

<div class="flex flex-col gap-5 mt-2">
  <div>
    <LocaleSelector></LocaleSelector>
    <!-- DOWNLOAD LOCALE FONT. TODO: FIND A BETTER HOME FOR THIS -->
    <!-- {#if localeFont !== undefined}
      <Button
        class="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2 mt-2"
        disabled={localeFontDownloading}
        onclick={async () => {}}
      >
        {#if localeFontDownloading}
          <Spinner class="mr-3" size="4" color="yellow" />
        {/if}
        {$_("settings_general_downloadLocaleSpecificFont")}
      </Button>
    {/if} -->
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
            await invalidateAll();
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
        await setAutoUpdateGames(config.autoUpdateGames);
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
          await message(
            `${$_("requirements_button_bypass_warning_1")}\n\n${$_(
              "requirements_button_bypass_warning_2",
            )}`,
            { title: "OpenGOAL Launcher", kind: "warning" },
          );
          await setBypassRequirements(true);
          await invalidateAll();
        }
        await setBypassRequirements(false);
        await invalidateAll();
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
          await invalidateAll();
        }
      }}>{$_("settings_general_button_resetSettings")}</Button
    >
  </div>
</div>
