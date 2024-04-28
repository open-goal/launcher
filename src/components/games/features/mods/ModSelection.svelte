<!-- TODO
 BUGS
 - 'extract_new_mod' complains due to something already being in use
 - when reinstalling the same mod, the settings map ends up as 'null'
 
 WRAP UP
 - launch string
 - cleanup rust code and such
 - uninstalling
 - pick the version / updating to latest version
 - remove unnecessary version lists
 - handle sources not being available
 - implement the things discussed here
     - https://github.com/open-goal/launcher/discussions/452
 - order by last played 
 - enable it by default
-->

<script lang="ts">
  import { getInternalName, SupportedGame } from "$lib/constants";
  import { createEventDispatcher, onMount } from "svelte";
  import { navigate } from "svelte-navigator";
  import { _ } from "svelte-i18n";
  import { Button, Input, Spinner, Tooltip } from "flowbite-svelte";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import IconGlobe from "~icons/mdi/globe";
  import { getModSourcesData, refreshModSources } from "$lib/rpc/cache";
  import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";
  import { filePrompt } from "$lib/utils/file-dialogs";
  import { extractNewMod, getInstalledMods } from "$lib/rpc/features";
  import { basename } from "@tauri-apps/api/path";

  const dispatch = createEventDispatcher();
  export let activeGame: SupportedGame;

  let loaded = false;
  let modFilter = "";
  let installedMods: Record<string, Record<string, string>> = {};
  let sourceData: Record<string, ModSourceData> = {};
  let addingFromFile = false;

  onMount(async () => {
    installedMods = await getInstalledMods(getInternalName(activeGame));
    await refreshModSources();
    sourceData = await getModSourcesData();
    loaded = true;
  });

  async function handleFilterChange(evt: Event) {
    const inputElement = evt.target as HTMLInputElement;
    modFilter = inputElement.value;
  }

  async function addModFromFile(evt: Event) {
    addingFromFile = true;
    const modArchivePath = await filePrompt(["zip"], "ZIP", "Select a mod");
    if (modArchivePath === null) {
      addingFromFile = false;
      return;
    }
    // extract the file into install_dir/features/<game>/_local/zip-name
    await extractNewMod(getInternalName(activeGame), modArchivePath, "_local");
    // install it immediately
    // - prompt user for iso if it doesn't exist
    // - decompile
    // - compile
    dispatch("job", {
      type: "installMod",
      modSourceName: "_local",
      modName: await basename(modArchivePath, ".zip"),
      modVersion: "local",
    });
    // remember that it is installed in the configuration
    addingFromFile = false;
  }
</script>

<div class="flex flex-col h-full bg-[#1e1e1e]">
  {#if !loaded}
    <div class="flex flex-col h-full justify-center items-center">
      <Spinner color="yellow" size={"12"} />
    </div>
  {:else}
    <div class="pb-20 overflow-y-auto p-4">
      <div class="flex flex-row gap-2 items-center">
        <Button
          outline
          class="flex-shrink border-solid rounded text-white hover:dark:text-slate-900 hover:bg-white font-semibold px-2 py-2"
          on:click={async () =>
            navigate(`/${getInternalName(activeGame)}`, { replace: true })}
          aria-label={$_("features_backToGamePage_buttonAlt")}
        >
          <IconArrowLeft />
        </Button>
        <Button
          class="flex-shrink border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
          on:click={addModFromFile}
          aria-label={$_("features_mods_addFromFile_buttonAlt")}
          disabled={addingFromFile}
        >
          {#if addingFromFile}
            <Spinner class="mr-3" size="4" color="white" />
          {/if}
          {$_("features_mods_addFromFile")}</Button
        >
      </div>
      <div class="mt-4">
        <Input placeholder="Filter Mods..." on:input={handleFilterChange} />
      </div>
      <!-- TODO - handle no sources added -->
      <h2 class="font-bold mt-2">Installed Mods</h2>
      {#if Object.entries(installedMods).length === 0}
        <p>No mods installed!</p>
      {:else}
        {#each Object.entries(installedMods) as [sourceUrl, sourceInstalledMods]}
          <div class="grid grid-cols-4 gap-4 mt-2">
            <!-- TODO - for real mods, go fetch the metadata using the version and name from the source -->
            {#each Object.entries(sourceInstalledMods) as [modName, modVersion]}
              {#if modFilter === "" || modName
                  .toLocaleLowerCase()
                  .startsWith(modFilter.toLocaleLowerCase())}
                <!-- TODO - background image -->
                <!-- TODO - source name -->
                <button
                  class="h-[200px] bg-cover p-1 flex justify-center items-end relative"
                  style="background-color: magenta"
                  on:click={async () => {
                    navigate(
                      `/${getInternalName(activeGame)}/features/mods/${encodeURI("_local")}/${encodeURI(modName)}`,
                    );
                  }}
                >
                  <h3 class="pointer-events-none select-none">
                    {modName}
                  </h3>
                  <div class="absolute top-0 right-0 m-2 flex gap-1">
                    <IconGlobe />
                    <Tooltip placement="bottom">{sourceUrl}</Tooltip>
                  </div>
                </button>
              {/if}
            {/each}
          </div>
        {/each}
      {/if}
      <h2 class="font-bold">Available Mods</h2>
      {#each Object.entries(sourceData) as [sourceUrl, sourceInfo]}
        <div class="grid grid-cols-4 gap-4 mt-2">
          {#each Object.entries(sourceInfo.mods) as [modName, modInfo]}
            {#if modFilter === "" || modInfo.displayName
                .toLocaleLowerCase()
                .startsWith(modFilter.toLocaleLowerCase())}
              <button
                class="h-[200px] bg-cover p-1 flex justify-center items-end relative grayscale"
                style="background-image: url('{modInfo.thumbnailArtUrl}')"
                on:click={async () => {
                  navigate(
                    `/${getInternalName(activeGame)}/features/mods/${encodeURI(sourceInfo.sourceName)}/${encodeURI(modName)}`,
                  );
                }}
              >
                <h3 class="pointer-events-none select-none">
                  {modInfo.displayName}
                </h3>
                <div class="absolute top-0 right-0 m-2 flex gap-1">
                  <IconGlobe />
                  <Tooltip placement="bottom">{sourceInfo.sourceName}</Tooltip>
                </div>
              </button>
            {/if}
          {/each}
        </div>
      {/each}
    </div>
  {/if}
</div>
