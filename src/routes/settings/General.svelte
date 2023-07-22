<script lang="ts">
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import {
    getBypassRequirements,
    getInstallationDirectory,
    getLocale,
    resetLauncherSettingsToDefaults,
    setBypassRequirements,
    setLocale,
  } from "$lib/rpc/config";
  import { getActiveVersion, getActiveVersionFolder } from "$lib/rpc/versions";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { Button, Helper, Label, Select, Toggle } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { confirm } from "@tauri-apps/api/dialog";

  let currentInstallationDirectory = "";
  let currentLocale;
  let availableLocales = [];
  let currentBypassRequirementsVal = false;

  onMount(async () => {
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
    currentLocale = await getLocale();
    currentBypassRequirementsVal = await getBypassRequirements();
  });
</script>

<div class="flex flex-col gap-5 mt-2">
  <div>
    <Label
      >{$_("settings_general_localeChange")}
      <Select
        class="mt-2"
        items={availableLocales}
        bind:value={currentLocale}
        on:change={async (evt) => {
          setLocale(evt.target.value);
        }}
      />
    </Label>
    <Helper class="text-sm mt-2"
      >{$_("settings_general_localeChange_helper_1")}
      <a
        class=" text-orange-400 hover:text-orange-600"
        href="https://crowdin.com/project/opengoal-launcher"
        target="_blank"
        rel="noreferrer">{$_("settings_general_localeChange_helper_link")}</a
      >
      {$_("settings_general_localeChange_helper_2")}</Helper
    >
  </div>
  <div>
    <Toggle
      checked={currentBypassRequirementsVal}
      color="orange"
      on:change={async (evt) => {
        if (evt.target.checked) {
          const confirmed = await confirm(
            `${$_("requirements_button_bypass_warning_1")}\n\n${$_(
              "requirements_button_bypass_warning_2"
            )}`,
            { title: "OpenGOAL Launcher", type: "warning" }
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
      on:click={async () => {
        const confirmed = await confirm(
          $_("settings_general_button_resetSettings_confirmation")
        );
        if (confirmed) {
          const result = resetLauncherSettingsToDefaults();
          if (result) {
            // TODO - move these to a store method
            $VersionStore.activeVersionType = await getActiveVersionFolder();
            $VersionStore.activeVersionName = await getActiveVersion();
          }
        }
      }}>{$_("settings_general_button_resetSettings")}</Button
    >
  </div>
</div>
