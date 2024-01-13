<script lang="ts">
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { _ } from "svelte-i18n";
  import { setLocale } from "$lib/rpc/config";
  import { Select, Label } from "flowbite-svelte";

  export let selectLocale;
  export let isLocaleSet;

  const LOCALES = AVAILABLE_LOCALES.map((locale) => ({
    value: locale.id,
    name: `${locale.flag} ${locale.localizedName}`,
  }));

  async function handleLocaleChange(evt: Event) {
    const selectElement = evt.target as HTMLSelectElement;
    setLocale(selectElement.value);
    selectLocale = false;
    isLocaleSet = true;
  }
</script>

<Label class="px-4 self-center">
  {$_("splash_selectLocale")}
  <Select
    data-testid="locale-select"
    name="locales"
    id="locales"
    class="mt-1"
    style="font-family: 'Twemoji Country Flags', 'Roboto Mono'"
    size="sm"
    items={LOCALES}
    on:change={handleLocaleChange}
    on:contextmenu={(e) => e.preventDefault}
  ></Select>
</Label>
