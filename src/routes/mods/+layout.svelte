<script lang="ts">
  import { Input } from "flowbite-svelte";
  import type { LayoutProps } from "./$types";
  import { _ } from "svelte-i18n";
  import { setContext } from "svelte";
  import { writable } from "svelte/store";

  let { data, children }: LayoutProps = $props();
  const modFilter = writable("");
  setContext("modFilter", modFilter);
</script>

<div class="flex flex-col h-screen bg-zinc-900 p-4">
  <div class="flex flex-row gap-2 items-center pb-2">
    <!-- <Button
      outline
      class="flex-shrink basis-8 border-solid rounded text-white hover:dark:text-slate-900 hover:bg-white font-semibold px-2 py-2"
      href={`/${activeGame}`}
      aria-label={$_("features_backToGamePage_buttonAlt")}
    >
      <IconArrowLeft />
    </Button> -->
    <Input
      placeholder={$_("features_mods_filter_placeholder")}
      bind:value={$modFilter}
    />
  </div>
  <div class="flex flex-col overflow-y-scroll pb-16">
    {@render children()}
  </div>
</div>
