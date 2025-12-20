<script lang="ts">
  import {
    getModDisplayName,
    getModTags,
    getModThumbnailImage,
    isModNotFiltered,
  } from "$lib/features/mods";
  import { Button } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import { navigate } from "/src/router";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import ModCard from "./ModCard.svelte";

  let {
    activeGame,
    installedModList,
    modFilter,
    modSourceData,
  }: {
    activeGame: SupportedGame;
    installedModList: Record<string, Record<string, string>>;
    modFilter: string;
    modSourceData: Record<string, ModSourceData>;
  } = $props();

  function ageOfModInDays(modInfo: ModInfo | undefined): number | undefined {
    if (modInfo === undefined) {
      return undefined;
    }
    const config = modInfo.perGameConfig?.[activeGame];
    if (!config?.releaseDate) {
      return undefined;
    }
    const releaseTime = Date.parse(config.releaseDate);
    return Math.round((Date.now() - releaseTime) / (1000 * 3600 * 24));
  }

  function isModAlreadyInstalled(sourceName: string, modName: string): boolean {
    return installedModList[sourceName]?.hasOwnProperty(modName) ?? false;
  }

  function isModSupportedByCurrentGame(modInfo: ModInfo): boolean {
    if (modInfo.supportedGames.includes(activeGame)) {
      return true;
    }
    return false;
  }
</script>

{#if Object.keys(modSourceData).length <= 0}
  <div class="mt-2 mb-2">
    <p class="text-slate-400 italic">
      {$_("features_mods_no_sources")}
    </p>
    <Button
      class="flex-shrink border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2 mt-2"
      onclick={async () => {
        navigate(`/settings/:tab`, {
          params: { tab: "mod" },
        });
      }}>{$_("features_mods_go_to_settings")}</Button
    >
  </div>
{:else}
  {#each Object.keys(modSourceData).sort() as sourceUrl}
    {@const sourceInfo = modSourceData[sourceUrl]}
    <h2 class="mt-2 text-orange-400">{sourceInfo.sourceName}</h2>
    <div class="grid grid-cols-4 gap-4 mt-2">
      <!-- TODO - sort new mods to the top (please let the user control the sorting (by name or newest)) -->
      {#each Object.keys(sourceInfo.mods).sort() as modName}
        {@const modInfo = sourceInfo.mods[modName]}
        {#if modInfo}
          {@const modAge = ageOfModInDays(modInfo)}
          {@const modDisplayName = getModDisplayName(
            modName,
            sourceInfo.sourceName,
            modSourceData,
          )}
          {@const modTags = getModTags(
            modName,
            sourceInfo.sourceName,
            modSourceData,
          )}
          {#if isModSupportedByCurrentGame(modInfo) && !isModAlreadyInstalled(sourceInfo.sourceName, modName)}
            {#if isModNotFiltered(modDisplayName, modTags, modFilter)}
              <ModCard
                {activeGame}
                modInternalName={modName}
                {modDisplayName}
                modSourceName={sourceInfo.sourceName}
                {modInfo}
                showNewIndicator={modAge !== undefined && modAge < 30}
                isInstalled={false}
                thumbnailUrl={getModThumbnailImage(activeGame, modInfo)}
                href={modInfo.externalLink}
              ></ModCard>
            {/if}
          {/if}
        {/if}
      {/each}
    </div>
  {/each}
{/if}
