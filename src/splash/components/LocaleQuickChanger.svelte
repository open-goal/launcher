<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { _ } from "svelte-i18n";
  import { getLocale } from "$lib/rpc/config";

  const dispatch = createEventDispatcher();

  let currentLocale = "en-US";

  // Events
  onMount(async () => {
    const locale = await getLocale();
    if (locale !== null) {
      currentLocale = locale;
    }
  });
</script>

<select
  data-testId="locale-select"
  name="locales"
  id="locales"
  title={$_("splash_selectLocale")}
  class="emoji-font pointer-events-auto !p-0 !pl-1 !pr-1 !pt-1 text-sm bg-gray-700 mb-1 absolute top-0 border-transparent focus:border-transparent focus:ring-0"
  on:change={(evt) => {
    let newLocale = evt.target.value;
    dispatch("change", {
      newLocale: newLocale,
    });
  }}
>
  {#each AVAILABLE_LOCALES as locale}
    {#if locale.id === currentLocale}
      <option value={locale.id} selected>{locale.flag}</option>
    {:else}
      <option value={locale.id}>{locale.flag}</option>
    {/if}
  {/each}
</select>

<style>
  .emoji-font {
    font-family: "Twemoji Country Flags", "Roboto Mono";
  }
</style>
