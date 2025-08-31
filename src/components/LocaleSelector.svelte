<script lang="ts">
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { _ } from "svelte-i18n";
  import { Select, Label, Helper } from "flowbite-svelte";
  import { setLocale } from "$lib/rpc/config";
  import { invalidateAll } from "$app/navigation";
  import { launcherConfig } from "$lib/stores/Config";

  const LOCALES = AVAILABLE_LOCALES.map((locale) => ({
    value: locale.id,
    name: locale.localizedName,
  }));

  // let selectedLocale = $state($launcherConfig.locale);
  let selectedLocale = $state("");
</script>

<Label>
  {$_("settings_general_localeChange")}
  <Select
    class="emoji-font mt-2"
    data-testId="locale-select"
    items={LOCALES}
    bind:value={selectedLocale}
    onchange={async () => {
      await setLocale(selectedLocale);
      await invalidateAll();
    }}
  ></Select>
</Label>

<Helper class="text-xs mt-2 italic"
  >{$_("settings_general_localeChange_helper_1")}
  <a
    class=" text-orange-400 hover:text-orange-600"
    href="https://crowdin.com/project/opengoal-launcher"
    target="_blank"
    rel="noreferrer">{$_("settings_general_localeChange_helper_link")}</a
  >
  {$_("settings_general_localeChange_helper_2")}
</Helper>
