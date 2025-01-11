<script lang="ts">
  import { AVAILABLE_LOCALES, type Locale } from "$lib/i18n/i18n";
  import { Button } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ change: { locale: Locale } }>();

  function selectLocale(locale: Locale) {
    dispatch("change", {
      locale
    });
  }

</script>



<div class="grid grid-cols-2 gap-3 flex-grow overflow-auto p-5">
  {#each AVAILABLE_LOCALES as locale}
    <Button
      class="bg-secondary text-secondary hover:bg-secondary-hover hover:text-secondary-hover rounded flex flex-row gap-3 justify-start"
      on:click={() => selectLocale(locale)}>
      <span class="grow-0">{locale.flag}</span>
      <span>{locale.localizedName}</span>
    </Button>
  {/each}
</div>
<div>
  <Button class="bg-primary text-primary hover:bg-primary-hover hover:text-primary-hover rounded">
    {$_("setup_button_continue")}
  </Button>
</div>