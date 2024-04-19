<!-- 
    install a mod / pick the version
    uninstalling
    and updating to latest version
    display picture
    when selecting the mod, you go back to the game page but the background is different
 -->

<!-- future improvements:
- order by last played -->

<script lang="ts">
  import { getInternalName, SupportedGame } from "$lib/constants";
  import { createEventDispatcher, onMount } from "svelte";
  import { navigate } from "svelte-navigator";
  import { _ } from "svelte-i18n";
  import { Button, Input, Spinner, Tooltip } from "flowbite-svelte";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import IconUpdate from "~icons/mdi/update";
  import IconGlobe from "~icons/mdi/globe";
  import { getModSourcesData, refreshModSources } from "$lib/rpc/cache";
  import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";

  const dispatch = createEventDispatcher();
  export let activeGame: SupportedGame;

  let loaded = false;
  let modFilter = "";
  let sourceData: Record<string, ModSourceData> = {};

  onMount(async () => {
    await refreshModSources();
    sourceData = await getModSourcesData();
    loaded = true;
  });

  async function handleFilterChange(evt: Event) {
    const inputElement = evt.target as HTMLInputElement;
    modFilter = inputElement.value;
    console.log("ye");
  }
</script>

<div class="flex flex-col h-full bg-slate-900">
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
        <!-- TODO - add mod from file -->
      </div>
      <div class="mt-4">
        <Input placeholder="Filter Mods..." on:input={handleFilterChange} />
      </div>
      <!-- TODO - handle no sources added -->
      <!-- Installed (all sources) -->
      <!-- <div
          class="h-[200px] bg-cover p-1 flex justify-center items-end relative"
          style="background-image: url('https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQuKmzjYulECD9I4ERduqu7FQ9b9l0_N1Ib2ocC1tCsGQ&s')"
        >
          <h3>Test Mod Name</h3>
          <div class="absolute top-0 right-0 m-2 flex gap-1">
            <IconUpdate />
            <IconGlobe />
            <Tooltip placement="bottom">Source URL</Tooltip>
          </div>
        </div> -->
      <!-- Sources (uninstalled) -->
      <h2 class="font-bold mt-2">Installed Mods</h2>
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

<!-- navigate(`/${getInternalName(activeGame)}/features/texture_packs`); -->
