<script lang="ts">
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { Label } from "flowbite-svelte";
  import { _ } from "svelte-i18n";

  let { locale = $bindable() }: { locale: string } = $props();
</script>

<div class="flex flex-row gap-2 items-center">
  <Label for="locales" class="">{$_("splash_selectLocale")}:</Label>
  <select
    data-testId="locale-select"
    name="locales"
    id="locales"
    class="pointer-events-auto p-0 pl-1 text-xs bg-gray-700 mt-1"
    bind:value={locale}
  >
    <option disabled selected value hidden></option>
    {#each AVAILABLE_LOCALES as LOCALE}
      <option value={LOCALE.id} class="emoji-font"
        >{LOCALE.flag}&nbsp;{LOCALE.localizedName}</option
      >
    {/each}
  </select>
</div>

<style>
  .emoji-font {
    font-family: "Twemoji Country Flags", "Roboto Mono";
  }
</style>
