<script lang="ts">
  import { setContext } from "svelte";
  import type { LayoutProps } from "./$types";
  import { writable } from "svelte/store";
  import { Button, Input, Spinner } from "flowbite-svelte";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import { _ } from "svelte-i18n";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { page } from "$app/state";
  import { filePrompt } from "$lib/utils/file-dialogs";
  import { extractNewMod } from "$lib/rpc/features";

  let { data, children }: LayoutProps = $props();
  const activeGame: SupportedGame = $derived(page.params.game);

  const modFilter = writable("");
  setContext("modFilter", modFilter);
  let addingMod = $state(false);

  async function addModFromFile(evt: Event) {
    addingMod = true;
    const modArchivePath = await filePrompt(
      ["zip, gz"],
      ".zip, .gz",
      "Select a mod",
    );
    if (modArchivePath === null) {
      addingMod = false;
      return;
    }
    // extract the file into install_dir/features/<game>/_local/zip-name
    await extractNewMod(activeGame, modArchivePath, "_local");
    // install it immediately
    // - prompt user for iso if it doesn't exist
    // - decompile
    // - compile
    // dispatch("job", {
    //   type: "installMod",
    //   modSourceName: "_local",
    //   modName: await basename(modArchivePath, ".zip"),
    //   modVersion: "local",
    // });
    addingMod = false;
  }
</script>

<div class="flex flex-col h-screen bg-zinc-900 p-4">
  <div class="flex flex-row gap-2 items-center pb-2">
    <Button
      outline
      class="flex-shrink basis-8 border-solid rounded text-white hover:dark:text-slate-900 hover:bg-white font-semibold px-2 py-2"
      href={`/${activeGame}`}
      aria-label={$_("features_backToGamePage_buttonAlt")}
    >
      <IconArrowLeft />
    </Button>

    <Button
      class="flex-shrink basis-32 border-solid rounded text-nowrap bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
      onclick={addModFromFile}
      aria-label={$_("features_mods_addFromFile_buttonAlt")}
      disabled={addingMod}
    >
      {#if addingMod}
        <Spinner class="mr-3" size="4" color="yellow" />
      {/if}
      {$_("features_mods_addFromFile")}</Button
    >
    <Input
      placeholder={$_("features_mods_filter_placeholder")}
      bind:value={$modFilter}
    />
  </div>
  <div class="flex flex-col overflow-y-scroll pb-16">
    {@render children()}
  </div>
</div>
