<script lang="ts">
  import { page } from "$app/state";
  import { filePrompt } from "$lib/utils/file-dialogs.js";
  import { Button, Indicator, Input, Spinner, Tooltip } from "flowbite-svelte";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import IconGlobe from "~icons/mdi/globe";
  import { extractNewMod, getInstalledMods } from "$lib/rpc/features";
  import { _ } from "svelte-i18n";

  let { data } = $props();
  const activeGame = $derived(page.params.game);
  let modFilter = $state("");
  let loaded = true;
  let addingMod = $state(false);
  let addingFromFile = $state(false);
  let installedMods = $state([]);
  let sourceData = [];

  async function addModFromFile(evt: Event) {
    addingMod = true;
    addingFromFile = true;
    const modArchivePath = await filePrompt(["zip"], "ZIP", "Select a mod");
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
          href={`/game/${activeGame}`}
          aria-label={$_("features_backToGamePage_buttonAlt")}
        >
          <IconArrowLeft />
        </Button>
        <Button
          class="flex-shrink border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
          onclick={addModFromFile}
          aria-label={$_("features_mods_addFromFile_buttonAlt")}
          disabled={addingMod}
        >
          {#if addingFromFile}
            <Spinner class="mr-3" size="4" color="yellow" />
          {/if}
          {$_("features_mods_addFromFile")}</Button
        >
      </div>
      <div class="mt-4">
        <Input
          placeholder={$_("features_mods_filter_placeholder")}
          bind:value={modFilter}
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
                    <a
                      class="h-[200px] bg-cover p-1 flex justify-center items-end relative"
                      style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{thumbnailSrc}'); background-size: cover;"
                      href={`/${activeGame}/mods/${sourceName}/${modName}`}
                    >
                      <h3 class="text-outline">
                        {getModDisplayName(sourceName, modName)}
                      </h3>
                      <div class="absolute top-0 right-0 m-2 flex gap-1">
                        <IconGlobe />
                        <Tooltip placement="bottom">{sourceName}</Tooltip>
                      </div>
                    </a>
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
          <a
            class="flex-shrink border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2 mt-2"
            href={`/settings/mods`}>{$_("features_mods_go_to_settings")}</a
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
                      <h3 class="text-outline">
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
                      hidden={!isLatestVersionOfModSupportedOnCurrentPlatform(
                        userPlatform,
                        modInfo,
                      )}
                      disabled={!isLatestVersionOfModSupportedOnCurrentPlatform(
                        userPlatform,
                        modInfo,
                      )}
                      class="h-[200px] max-w-[160px] bg-cover p-1 flex justify-center items-end relative"
                      style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{getThumbnailImage(
                        modInfo,
                      )}'); background-size: cover;"
                      onclick={async () => {
                        navigate(
                          `/${activeGame}/mods/${encodeURI(sourceInfo.sourceName)}/${encodeURI(modName)}`,
                        );
                      }}
                    >
                      <h3 class="text-outline">
                        {modInfo.displayName}
                      </h3>
                      <div class="absolute top-0 right-0 m-2 flex gap-1">
                        <IconGlobe />
                        <Tooltip placement="bottom"
                          >{sourceInfo.sourceName}</Tooltip
                        >
                      </div>
                      {#if modAge && modAge < 30}
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

                  {#if !isLatestVersionOfModSupportedOnCurrentPlatform(userPlatform, modInfo) && modInfo.externalLink === null}
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
