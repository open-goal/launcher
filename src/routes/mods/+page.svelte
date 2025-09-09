<script lang="ts">
  import { page } from "$app/state";
  import { Indicator, Tooltip } from "flowbite-svelte";
  import IconGlobe from "~icons/mdi/globe";
  import { _ } from "svelte-i18n";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo.js";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame.js";
  import { getContext } from "svelte";

  let { data } = $props();
  const sourceData = $derived(data.sourceData);
  const installedMods = $derived(data.installedMods);
  const modsByGame = $derived(data.modsByGame);
  const { jak1, jak2, jak3 } = $derived(data.modsByGame);

  const activeGame = "jak1";
  let modFilter = getContext("modFilter");

  function ageOfModInDays(modInfo: ModInfo): number | undefined {
    const config = modInfo.perGameConfig?.[activeGame];
    if (!config?.releaseDate) return undefined;
    const releaseTime = Date.parse(config.releaseDate);
    const now = Date.now();
    return Math.round((now - releaseTime) / (1000 * 3600 * 24));
  }

  function isModInstalled(sourceName: string, modName: string): boolean {
    return installedMods[sourceName]?.hasOwnProperty(modName) ?? false;
  }

  function getThumbnailImage(modInfo: ModInfo): string {
    // Prefer pre-game-config if available
    if (
      modInfo.perGameConfig !== null &&
      modInfo.perGameConfig.hasOwnProperty(activeGame) &&
      modInfo.perGameConfig[activeGame].thumbnailArtUrl
    ) {
      return modInfo.perGameConfig[activeGame].thumbnailArtUrl;
    } else if (modInfo.thumbnailArtUrl !== null) {
      return modInfo.thumbnailArtUrl;
    }
    return thumbnailPlaceholder;
  }

  function getModDisplayName(sourceName: string, modName: string): string {
    // Find the mod by looking at the sources, if we can't find it then return the placeholder
    const source = Object.values(sourceData).find(
      (s) => s.sourceName === sourceName && modName in s.mods,
    );
    return source?.mods[modName]?.displayName ?? modName;
  }

  async function getThumbnailImageFromSources(
    sourceName: string,
    modName: string,
  ): Promise<string> {
    // TODO - make this not a promise, do it in the initial component loading
    // if (sourceName === "_local") {
    //   return await getLocalModThumbnailBase64(activeGame, modName);
    // }
    // Find the mod by looking at the sources, if we can't find it then return the placeholder
    const source = Object.values(sourceData).find(
      (s) => s.sourceName === sourceName && modName in s.mods,
    );

    return source
      ? getThumbnailImage(source.mods[modName])
      : thumbnailPlaceholder;
  }
</script>

{#if Object.entries(installedMods).length !== 0}
  <!-- <h2 class="font-bold">{$_("features_mods_installed_header")}</h2> -->
  <!-- {#each Object.keys(installedMods).sort() as sourceName}
    {@const sourceInstalledMods = installedMods[sourceName]}
    {#if Object.entries(sourceInstalledMods).length !== 0}
      <h2 class="mt-2 text-orange-400">{sourceName}</h2>
      <div class="grid grid-cols-4 gap-4 mt-2">
        {#each Object.keys(sourceInstalledMods).sort() as modName}
          {#if $modFilter === "" || getModDisplayName(sourceName, modName)
              .toLocaleLowerCase()
              .includes($modFilter.toLocaleLowerCase()) || getModTags(sourceName, modName)
              .join(",")
              .toLocaleLowerCase()
              .includes($modFilter.toLocaleLowerCase())}
            {#await getThumbnailImageFromSources(sourceName, modName) then thumbnailSrc}
              <a
                class="h-[200px] bg-cover p-1 flex justify-center items-end relative"
                style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{thumbnailSrc}'); background-size: cover;"
                href={`${sourceName}/${modName}`}
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
  {/each} -->
{/if}

{#each Object.entries(modsByGame) as [game, mods]}
  {#if game.length > 0}
    <h2 class="mt-2 text-orange-400">{$_(`gameName_${game}`)}</h2>
    <div class="grid grid-cols-4 gap-4 mt-2">
      {#each Object.values(mods) as mod}
        {#await getThumbnailImageFromSources(mod.source, mod.name) then thumbnailSrc}
          <a
            class="h-[200px] bg-cover p-1 flex justify-center items-end relative"
            style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{thumbnailSrc}'); background-size: cover;"
            href={`/${game}/${mod.source}/${mod.name}`}
          >
            <h3 class="text-outline">
              {getModDisplayName(mod.source, mod.name)}
            </h3>
            <div class="absolute top-0 right-0 m-2 flex gap-1">
              <IconGlobe />
              <Tooltip placement="bottom">{mod.source}</Tooltip>
            </div>
          </a>
        {/await}
      {/each}
    </div>
  {/if}
{/each}

{#if Object.keys(sourceData).length == 0}
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
  <!-- <h2 class="font-bold">{$_("features_mods_available_header")}</h2> -->
  {#each Object.keys(sourceData).sort() as sourceUrl}
    {@const sourceInfo = sourceData[sourceUrl]}
    <div class="grid grid-cols-4 gap-4 mt-2">
      <!-- TODO - sort new mods to the top -->
      <!-- {#each Object.keys(sourceInfo.mods).sort() as modName}
        {@const modInfo = sourceInfo.mods[modName]}
        {@const modAge = ageOfModInDays(modInfo)}
        {#if !isModInstalled(sourceInfo.sourceName, modName)}
          {#if $modFilter === "" || modInfo.displayName
              .toLocaleLowerCase()
              .includes($modFilter.toLocaleLowerCase()) || modInfo.tags
              .join(",")
              .toLocaleLowerCase()
              .includes($modFilter.toLocaleLowerCase())}
            {#if modInfo.externalLink}
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
                  <Tooltip placement="bottom">{sourceInfo.sourceName}</Tooltip>
                </div>
              </a>
            {:else}
              <a
                class="h-[200px] max-w-[160px] bg-cover p-1 flex justify-center items-end relative"
                style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{getThumbnailImage(
                  modInfo,
                )}'); background-size: cover;"
                href={`${sourceInfo.sourceName}/${modName}`}
              >
                <h3 class="text-outline">
                  {modInfo.displayName}
                </h3>
                <div class="absolute top-0 right-0 m-2 flex gap-1">
                  <IconGlobe />
                  <Tooltip placement="bottom">{sourceInfo.sourceName}</Tooltip>
                </div>
              </a>
            {/if}
          {/if}
        {/if}
      {/each} -->
    </div>
  {/each}
{/if}

<!-- {#if modAge && modAge < 30}
                  <Indicator
                    color="green"
                    border
                    size="xl"
                    placement="top-right"
                  >
                    <span class="text-white text-xs font-bold">!</span>
                  </Indicator>
                {/if} -->
