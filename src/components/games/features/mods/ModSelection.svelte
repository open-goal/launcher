<!-- TODO
 BUGS
 - 'extract_new_mod' complains due to something already being in use
 - when reinstalling the same mod, the settings map ends up as 'null'

 FEATURES
 - launch string (shortcuts)
 - uninstalling
 - pick the version / updating to latest version
 - remove unnecessary version lists (developer,unofficial)
 - cleanup rust code and frontend code
 - translations
 - order by last played

 WRAP UP
 - do a pass of and close https://github.com/open-goal/launcher/discussions/452 features are captured here
   - anything extra, make an actual issue for it (ie. metrics)
 - enable it by default
-->

<script lang="ts">
  import { platform } from "@tauri-apps/api/os";
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
  import { downloadAndExtractNewMod, extractNewMod, getInstalledMods, getLocalModThumbnailBase64 } from "$lib/rpc/features";
  import { basename } from "@tauri-apps/api/path";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import thumbnailPlaceholder from "$assets/images/mod-thumbnail-placeholder.webp";

  const dispatch = createEventDispatcher();
  export let activeGame: SupportedGame;

  let loaded = false;
  let modFilter = "";
  let installedMods: Record<string, Record<string, string>> = {};
  let sourceData: Record<string, ModSourceData> = {};
    // TODO - sort mods by name
  let sortedAvailableModNames: Record<string, string[]> = {};
  let addingMod = false;
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
    addingMod = true;
    addingFromFile = true;
    const modArchivePath = await filePrompt(["zip"], "ZIP", "Select a mod");
    if (modArchivePath === null) {
      addingMod = false;
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
    addingMod = false;
    addingFromFile = false;
  }

  async function addModFromUrl(url: string, modName: string, sourceName: string, modVersion: string) {
    addingMod = true;
    // extract the file into install_dir/features/<game>/<sourceName>/<modName>
    await downloadAndExtractNewMod(getInternalName(activeGame), url, modName, sourceName);
    // install it immediately
    // - prompt user for iso if it doesn't exist
    // - decompile
    // - compile
    dispatch("job", {
      type: "installMod",
      modSourceName: sourceName,
      modName: modName,
      modVersion: modVersion,
    });
    addingMod = false;
  }

  function getThumbnailImage(modInfo: ModInfo): string {
    // Prefer pre-game-config if available
    if (modInfo.perGameConfig !== null && modInfo.perGameConfig.hasOwnProperty(getInternalName(activeGame)) && modInfo.perGameConfig[getInternalName(activeGame)].thumbnailArtUrl !== null) {
      return modInfo.perGameConfig[getInternalName(activeGame)].thumbnailArtUrl;
    } else if (modInfo.thumbnailArtUrl !== null) {
      return modInfo.coverArtUrl;
    }
    return thumbnailPlaceholder;
  }

  async function getThumbnailImageFromSources(sourceName: string, modName: string): Promise<string> {
    if (sourceName === "_local") {
      return await getLocalModThumbnailBase64(getInternalName(activeGame), modName);
    }
    // Find the mod by looking at the sources, if we can't find it then return the placeholder
    for (const [sourceUrl, sourceInfo] of Object.entries(sourceData)) {
      if (sourceInfo.sourceName === sourceName && Object.keys(sourceInfo.mods).includes(modName)) {
        // TODO NOW - pass in active game incase of per-game?
        return getThumbnailImage(sourceInfo.mods[modName]);
      }
    }
    return thumbnailPlaceholder;
  }

  function getModDisplayName(sourceName: string, modName: string): string {
    // Find the mod by looking at the sources, if we can't find it then return the placeholder
    for (const [sourceUrl, sourceInfo] of Object.entries(sourceData)) {
      if (sourceInfo.sourceName === sourceName && Object.keys(sourceInfo.mods).includes(modName)) {
        return sourceInfo.mods[modName].displayName;
      }
    }
    return modName;
  }

  function getModTags(sourceName: string, modName: string): Array<string> {
    // Find the mod by looking at the sources, if we can't find it then return the placeholder
    for (const [sourceUrl, sourceInfo] of Object.entries(sourceData)) {
      if (sourceInfo.sourceName === sourceName && Object.keys(sourceInfo.mods).includes(modName)) {
        return sourceInfo.mods[modName].tags;
      }
    }
    return [];
  }

  async function getModAssetUrl(modInfo: ModInfo): Promise<string> {
    // TODO - make safer
    const plat = await platform();
    if (plat === "linux") {
      return modInfo.versions[0].assets["linux"];
    } else if (plat == "darwin") {
      return modInfo.versions[0].assets["macos"];
    }
    return modInfo.versions[0].assets["windows"];
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
          disabled={addingMod}
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
      <h2 class="font-bold mt-2">Installed Mods</h2>
      {#if Object.entries(installedMods).length === 0}
        <p class="mt-2 mb-2 text-slate-400 italic">No mods installed!</p>
      {:else}
        {#each Object.entries(installedMods) as [sourceName, sourceInstalledMods]}
          <div class="grid grid-cols-4 gap-4 mt-2">
            <!-- TODO - for real mods, go fetch the metadata using the version and name from the source -->
            {#each Object.entries(sourceInstalledMods) as [modName, modVersion]}
              {#if modFilter === "" 
                  || getModDisplayName(sourceName, modName).toLocaleLowerCase().includes(modFilter.toLocaleLowerCase())
                  || getModTags(sourceName, modName).join(',').toLocaleLowerCase().includes(modFilter.toLocaleLowerCase())
                  }
                <!-- TODO - background image -->
                {#await getThumbnailImageFromSources(sourceName, modName) then thumbnailSrc}
                  <button
                  class="h-[200px] bg-cover p-1 flex justify-center items-end relative"
                  style="background-image: url('{thumbnailSrc}')"
                  on:click={async () => {
                    navigate(
                      `/${getInternalName(activeGame)}/features/mods/${encodeURI(sourceName)}/${encodeURI(modName)}`,
                    );
                  }}
                >
                  <h3 class="pointer-events-none select-none text-outline">
                    {getModDisplayName(sourceName, modName)}
                  </h3>
                  <div class="absolute top-0 right-0 m-2 flex gap-1">
                    <IconGlobe />
                    <Tooltip placement="bottom">{sourceName}</Tooltip>
                  </div>
                </button>
                {/await}
              {/if}
            {/each}
          </div>
        {/each}
      {/if}
      <h2 class="font-bold">Available Mods</h2>
      {#if Object.keys(sourceData).length <= 0}
        <div class="mt-2 mb-2">
          <p class="text-slate-400 italic">You have no mod sources configured or they are unavailable!</p>
          <Button
            class="flex-shrink border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2 mt-2"
            on:click={async () => {
              navigate(`/settings/mods`, {
                replace: true,
              });
            }}>Go to Mod Settings</Button
          >
        </div>
      {:else}
        {#each Object.entries(sourceData) as [sourceUrl, sourceInfo]}
          <div class="grid grid-cols-4 gap-4 mt-2">
            {#each Object.entries(sourceInfo.mods) as [modName, modInfo]}
              {#if modInfo.supportedGames.includes(getInternalName(activeGame))}
                {#if modFilter === ""
                    || modInfo.displayName.toLocaleLowerCase().includes(modFilter.toLocaleLowerCase())
                    || modInfo.tags.join(",").toLocaleLowerCase().includes(modFilter.toLocaleLowerCase())
                    }
                  <!-- TODO - disable with tooltip if not on their platform -->
                  <button
                    class="h-[200px] max-w-[160px] bg-cover p-1 flex justify-center items-end relative grayscale"
                    style="background-image: url('{getThumbnailImage(modInfo)}')"
                    on:click={async () => {
                      // Install the mod
                      const assetUrl = await getModAssetUrl(modInfo);
                      await addModFromUrl(assetUrl, modName, sourceInfo.sourceName, modInfo.versions[0].version);
                    }}
                  >
                    <h3 class="pointer-events-none select-none text-outline">
                      {modInfo.displayName}
                    </h3>
                    <div class="absolute top-0 right-0 m-2 flex gap-1">
                      <IconGlobe />
                      <Tooltip placement="bottom">{sourceInfo.sourceName}</Tooltip>
                    </div>
                  </button>
                {/if}
              {/if}
            {/each}
          </div>
        {/each}
      {/if}
    </div>
  {/if}
</div>
