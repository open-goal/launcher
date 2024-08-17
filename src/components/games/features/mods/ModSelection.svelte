<script lang="ts">
  import { platform } from "@tauri-apps/api/os";
  import { getInternalName, SupportedGame } from "$lib/constants";
  import { createEventDispatcher, onMount } from "svelte";
  import { navigate } from "svelte-navigator";
  import { _ } from "svelte-i18n";
  import { Button, Indicator, Input, Spinner, Tooltip } from "flowbite-svelte";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import IconGlobe from "~icons/mdi/globe";
  import { getModSourcesData, refreshModSources } from "$lib/rpc/cache";
  import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";
  import { filePrompt } from "$lib/utils/file-dialogs";
  import {
    extractNewMod,
    getInstalledMods,
    getLocalModThumbnailBase64,
  } from "$lib/rpc/features";
  import { basename } from "@tauri-apps/api/path";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import thumbnailPlaceholder from "$assets/images/mod-thumbnail-placeholder.webp";

  const dispatch = createEventDispatcher();
  export let activeGame: SupportedGame;

  let userPlatform = "";
  let loaded = false;
  let modFilter = "";
  let installedMods: Record<string, Record<string, string>> = {};
  let sourceData: Record<string, ModSourceData> = {};
  let addingMod = false;
  let addingFromFile = false;

  onMount(async () => {
    installedMods = await getInstalledMods(getInternalName(activeGame));
    // TODO - move this to a central store!
    await refreshModSources();
    sourceData = await getModSourcesData();
    userPlatform = await platform();
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

  async function addModFromUrl(
    url: string,
    modName: string,
    sourceName: string,
    modVersion: string,
  ) {
    addingMod = true;
    // install it immediately
    // - prompt user for iso if it doesn't exist
    // - decompile
    // - compile
    dispatch("job", {
      type: "installModExternal",
      modDownloadUrl: url,
      modSourceName: sourceName,
      modName: modName,
      modVersion: modVersion,
    });
    addingMod = false;
  }

  function getThumbnailImage(modInfo: ModInfo): string {
    // Prefer pre-game-config if available
    if (
      modInfo.perGameConfig !== null &&
      modInfo.perGameConfig.hasOwnProperty(getInternalName(activeGame)) &&
      modInfo.perGameConfig[getInternalName(activeGame)].thumbnailArtUrl !==
        null
    ) {
      return modInfo.perGameConfig[getInternalName(activeGame)].thumbnailArtUrl;
    } else if (modInfo.thumbnailArtUrl !== null) {
      return modInfo.thumbnailArtUrl;
    }
    return thumbnailPlaceholder;
  }

  async function getThumbnailImageFromSources(
    sourceName: string,
    modName: string,
  ): Promise<string> {
    // TODO - make this not a promise, do it in the initial component loading
    if (sourceName === "_local") {
      return await getLocalModThumbnailBase64(
        getInternalName(activeGame),
        modName,
      );
    }
    // Find the mod by looking at the sources, if we can't find it then return the placeholder
    for (const [sourceUrl, sourceInfo] of Object.entries(sourceData)) {
      if (
        sourceInfo.sourceName === sourceName &&
        Object.keys(sourceInfo.mods).includes(modName)
      ) {
        // TODO NOW - pass in active game incase of per-game?
        return getThumbnailImage(sourceInfo.mods[modName]);
      }
    }
    return thumbnailPlaceholder;
  }

  function getModDisplayName(sourceName: string, modName: string): string {
    // Find the mod by looking at the sources, if we can't find it then return the placeholder
    for (const [sourceUrl, sourceInfo] of Object.entries(sourceData)) {
      if (
        sourceInfo.sourceName === sourceName &&
        Object.keys(sourceInfo.mods).includes(modName)
      ) {
        return sourceInfo.mods[modName].displayName;
      }
    }
    return modName;
  }

  function getModTags(sourceName: string, modName: string): Array<string> {
    // Find the mod by looking at the sources, if we can't find it then return the placeholder
    for (const [sourceUrl, sourceInfo] of Object.entries(sourceData)) {
      if (
        sourceInfo.sourceName === sourceName &&
        Object.keys(sourceInfo.mods).includes(modName)
      ) {
        return sourceInfo.mods[modName].tags;
      }
    }
    return [];
  }

  function getModAssetUrl(modInfo: ModInfo): string {
    // TODO - make safer
    if (userPlatform === "linux") {
      return modInfo.versions[0].assets["linux"];
    } else if (userPlatform == "darwin") {
      return modInfo.versions[0].assets["macos"];
    }
    return modInfo.versions[0].assets["windows"];
  }

  function isModAlreadyInstalled(sourceName: string, modName: string): boolean {
    if (
      Object.keys(installedMods).includes(sourceName) &&
      Object.keys(installedMods[sourceName]).includes(modName)
    ) {
      return true;
    }
    return false;
  }

  function isModSupportedOnCurrentPlatform(modInfo: ModInfo): boolean {
    // TODO - make safer
    if (modInfo.versions.length > 0) {
      if (userPlatform === "linux") {
        return Object.keys(modInfo.versions[0].assets).includes("linux");
      } else if (userPlatform == "darwin") {
        return Object.keys(modInfo.versions[0].assets).includes("macos");
      }
      return Object.keys(modInfo.versions[0].assets).includes("windows");
    }
    return false;
  }

  function isModSupportedByCurrentGame(modInfo: ModInfo): boolean {
    if (
      modInfo.versions.length > 0 &&
      modInfo.versions[0].supportedGames !== null
    ) {
      return modInfo.versions[0].supportedGames.includes(
        getInternalName(activeGame),
      );
    }
    return modInfo.supportedGames.includes(getInternalName(activeGame));
  }

  function ageOfModInDays(modInfo: ModInfo): number | undefined {
    if (
      modInfo.perGameConfig !== null &&
      Object.keys(modInfo.perGameConfig).includes(
        getInternalName(activeGame),
      ) &&
      modInfo.perGameConfig[getInternalName(activeGame)].releaseDate !== null
    ) {
      const difference =
        new Date().getTime() -
        Date.parse(
          modInfo.perGameConfig[getInternalName(activeGame)].releaseDate,
        );
      return Math.round(difference / (1000 * 3600 * 24));
    }
    return undefined;
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
        <Input
          placeholder={$_("features_mods_filter_placeholder")}
          on:input={handleFilterChange}
        />
      </div>
      <h2 class="font-bold mt-2">{$_("features_mods_installed_header")}</h2>
      {#if Object.entries(installedMods).length === 0}
        <p class="mt-2 mb-2 text-slate-400 italic">
          {$_("features_mods_nothing_installed")}
        </p>
      {:else}
        {#each Object.keys(installedMods).sort() as sourceName}
          {@const sourceInstalledMods = installedMods[sourceName]}
          {#if Object.entries(sourceInstalledMods).length !== 0}
            <h2 class="mt-2 text-orange-400">{sourceName}</h2>
            <div class="grid grid-cols-4 gap-4 mt-2">
              {#each Object.keys(sourceInstalledMods).sort() as modName}
                {#if modFilter === "" || getModDisplayName(sourceName, modName)
                    .toLocaleLowerCase()
                    .includes(modFilter.toLocaleLowerCase()) || getModTags(sourceName, modName)
                    .join(",")
                    .toLocaleLowerCase()
                    .includes(modFilter.toLocaleLowerCase())}
                  {#await getThumbnailImageFromSources(sourceName, modName) then thumbnailSrc}
                    <button
                      class="h-[200px] bg-cover p-1 flex justify-center items-end relative"
                      style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{thumbnailSrc}'); background-size: cover;"
                      on:click={async () => {
                        navigate(
                          `/${getInternalName(activeGame)}/features/mods/${encodeURI(sourceName)}/${encodeURI(modName)}`,
                        );
                      }}
                    >
                      <h3 class="pointer-events-none select-none text-shadow">
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
          {/if}
        {/each}
      {/if}
      <h2 class="font-bold mt-5">{$_("features_mods_available_header")}</h2>
      {#if Object.keys(sourceData).length <= 0}
        <div class="mt-2 mb-2">
          <p class="text-slate-400 italic">
            {$_("features_mods_no_sources")}
          </p>
          <Button
            class="flex-shrink border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2 mt-2"
            on:click={async () => {
              navigate(`/settings/mods`, {
                replace: true,
              });
            }}>{$_("features_mods_go_to_settings")}</Button
          >
        </div>
      {:else}
        {#each Object.keys(sourceData).sort() as sourceUrl}
          {@const sourceInfo = sourceData[sourceUrl]}
          <h2 class="mt-2 text-orange-400">{sourceInfo.sourceName}</h2>
          <div class="grid grid-cols-4 gap-4 mt-2">
            <!-- TODO - sort new mods to the top -->
            {#each Object.keys(sourceInfo.mods).sort() as modName}
              {@const modInfo = sourceInfo.mods[modName]}
              {@const modAge = ageOfModInDays(modInfo)}
              {#if isModSupportedByCurrentGame(modInfo) && !isModAlreadyInstalled(sourceInfo.sourceName, modName)}
                {#if modFilter === "" || modInfo.displayName
                    .toLocaleLowerCase()
                    .includes(modFilter.toLocaleLowerCase()) || modInfo.tags
                    .join(",")
                    .toLocaleLowerCase()
                    .includes(modFilter.toLocaleLowerCase())}
                  {#if modInfo.externalLink !== null}
                    <a
                      href={modInfo.externalLink}
                      target="_blank"
                      rel="noreferrer noopener"
                      class="h-[200px] max-w-[160px] bg-cover p-1 flex justify-center items-end relative grayscale"
                      style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{getThumbnailImage(
                        modInfo,
                      )}'); background-size: cover;"
                    >
                      <h3 class="pointer-events-none select-none text-outline">
                        {modInfo.displayName}
                      </h3>
                      <div class="absolute top-0 right-0 m-2 flex gap-1">
                        <IconGlobe />
                        <Tooltip placement="bottom"
                          >{sourceInfo.sourceName}</Tooltip
                        >
                      </div>
                    </a>
                  {:else}
                    <button
                      disabled={!isModSupportedOnCurrentPlatform(modInfo)}
                      class="h-[200px] max-w-[160px] bg-cover p-1 flex justify-center items-end relative"
                      style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{getThumbnailImage(
                        modInfo,
                      )}'); background-size: cover;"
                      on:click={async () => {
                        // Install the mod
                        const assetUrl = getModAssetUrl(modInfo);
                        await addModFromUrl(
                          assetUrl,
                          modName,
                          sourceInfo.sourceName,
                          modInfo.versions[0].version,
                        );
                      }}
                    >
                      <h3 class="pointer-events-none select-none text-outline">
                        {modInfo.displayName}
                      </h3>
                      <div class="absolute top-0 right-0 m-2 flex gap-1">
                        <IconGlobe />
                        <Tooltip placement="bottom"
                          >{sourceInfo.sourceName}</Tooltip
                        >
                      </div>
                      {#if modAge !== undefined && modAge < 30}
                        <Indicator
                          color="green"
                          border
                          size="xl"
                          placement="top-right"
                        >
                          <span class="text-white text-xs font-bold">!</span>
                        </Indicator>
                      {/if}
                    </button>
                  {/if}

                  {#if !isModSupportedOnCurrentPlatform(modInfo) && modInfo.externalLink === null}
                    <Tooltip placement="top"
                      >{$_("features_mods_not_supported_platform_1")} ({userPlatform})<br
                      />{$_("features_mods_not_supported_platform_2")}</Tooltip
                    >
                  {/if}
                {/if}
              {/if}
            {/each}
          </div>
        {/each}
      {/if}
    </div>
  {/if}
</div>
