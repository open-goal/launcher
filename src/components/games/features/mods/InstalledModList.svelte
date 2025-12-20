<script lang="ts">
  import { _ } from "svelte-i18n";
  import { getLocalModThumbnailBase64 } from "$lib/rpc/features";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";
  import thumbnailPlaceholder from "$assets/images/mod-thumbnail-placeholder.webp";
  import {
    getModDisplayName,
    getModTags,
    getModThumbnailImage,
    isModNotFiltered,
  } from "$lib/features/mods";
  import ModCard from "./ModCard.svelte";

  let {
    activeGame,
    modList,
    modFilter,
    modSourceData,
  }: {
    activeGame: SupportedGame;
    modList: Record<string, Record<string, string>>;
    modFilter: string;
    modSourceData: Record<string, ModSourceData>;
  } = $props();

  async function getThumbnailImageFromSources(
    modName: string,
    modSourceName: string,
  ): Promise<string> {
    if (modSourceName === "_local") {
      return await getLocalModThumbnailBase64(activeGame, modName);
    }
    const source = Object.values(modSourceData).find(
      (s) => s.sourceName === modSourceName && modName in s.mods,
    );
    return source
      ? getModThumbnailImage(activeGame, source?.mods[modName])
      : thumbnailPlaceholder;
  }
</script>

{#if Object.entries(modList).length === 0}
  <p class="mt-2 mb-2 text-slate-400 italic">
    {$_("features_mods_nothing_installed")}
  </p>
{:else}
  {#each Object.keys(modList).sort() as modSourceName}
    {@const sourceInstalledMods = modList[modSourceName]}
    {#if Object.entries(sourceInstalledMods).length !== 0}
      <h2 class="mt-2 text-orange-400">{modSourceName}</h2>
      <div class="grid grid-cols-4 gap-4 mt-2">
        {#each Object.keys(sourceInstalledMods).sort() as modName}
          {@const modDisplayName = getModDisplayName(
            modName,
            modSourceName,
            modSourceData,
          )}
          {@const modTags = getModTags(modName, modSourceName, modSourceData)}
          {#if isModNotFiltered(modDisplayName, modTags, modFilter)}
            {#await getThumbnailImageFromSources(modName, modSourceName) then thumbnailSrc}
              <ModCard
                {activeGame}
                modInternalName={modName}
                {modDisplayName}
                modInfo={undefined}
                {modSourceName}
                showNewIndicator={false}
                isInstalled={true}
                thumbnailUrl={thumbnailSrc}
                href={null}
              ></ModCard>
            {/await}
          {/if}
        {/each}
      </div>
    {/if}
  {/each}
{/if}
