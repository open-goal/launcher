<script lang="ts">
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { _ } from "svelte-i18n";
  import { Select, Label } from "flowbite-svelte";

  const LOCALES = AVAILABLE_LOCALES.map((locale) => ({
    value: locale.id,
    name: locale.localizedName,
  }));

  let selectedLocale = $state("");
  // $effect(() => ()); // update the locale
</script>

<Label>
  Select a locale
  <Select
    class="emoji-font bg-gray-700"
    data-testId="locale-select"
    items={LOCALES}
    bind:value={selectedLocale}
  ></Select>
</Label>

<select
  data-testId="locale-select"
  name="locales"
  id="locales"
  class="pointer-events-auto p-0 pl-1 text-xs bg-gray-700 mt-1"
>
  <option disabled selected value hidden></option>
  {#each AVAILABLE_LOCALES as locale}
    <option value={locale.id} class="emoji-font"
      >{locale.flag} {locale.localizedName}</option
    >
  {/each}
</select>
