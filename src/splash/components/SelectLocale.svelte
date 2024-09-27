<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { _ } from "svelte-i18n";

  const dispatch = createEventDispatcher();

  // Events
  onMount(async () => {});
</script>

<select
  data-testId="locale-select"
  name="locales"
  id="locales"
  class="pointer-events-auto p-0 pl-1 text-xs bg-gray-700 mt-1"
  on:change={(evt) => {
    let newLocale = evt.target.value;
    dispatch("change", {
      newLocale: newLocale,
    });
  }}
>
  <option disabled selected value hidden />
  {#each AVAILABLE_LOCALES as locale}
    <option value={locale.id}
      ><span class="emoji-font">{locale.flag}</span
      >&nbsp;{locale.localizedName}</option
    >
  {/each}
</select>

<style>
  .emoji-font {
    font-family: "Twemoji Country Flags", "Roboto Mono";
  }
</style>
