<script lang="ts">
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { _ } from "svelte-i18n";
  import { Select, Label } from "flowbite-svelte";
  import { setLocale } from "$lib/rpc/config";

  const LOCALES = AVAILABLE_LOCALES.map((locale) => ({
    value: locale.id,
    name: locale.localizedName,
  }));

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
    }}
  ></Select>
</Label>
