<script lang="ts">
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { Label, Select } from "flowbite-svelte";
  import { _ } from "svelte-i18n";

  let { locale = $bindable() }: { locale: string } = $props();
</script>

<div class="flex flex-col w-full space-y-2">
  <Label for="locales" class="text-gray-200 font-semibold"
    >{$_("splash_selectLocale")}</Label
  >
  <Select
    data-testId="locale-select"
    name="locales"
    id="locales"
    class="pointer-events-auto rounded-lg bg-gray-700"
    bind:value={locale}
  >
    <option disabled selected value hidden></option>
    {#each AVAILABLE_LOCALES as LOCALE}
      <option value={LOCALE.id} class="emoji-font"
        >{LOCALE.flag}&nbsp;{LOCALE.localizedName}</option
      >
    {/each}
  </Select>
</div>

<style>
  .emoji-font {
    font-family: "Twemoji Country Flags", "Roboto Mono";
  }
</style>
