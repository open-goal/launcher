<script lang="ts">
  import {
    cleanupEnabledTexturePacks,
    getEnabledTexturePacks,
  } from "$lib/rpc/config";
  import {
    extractNewTexturePack,
    listExtractedTexturePackInfo,
  } from "$lib/rpc/features";
  import { filePrompt } from "$lib/utils/file-dialogs";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import { Alert, Button, Spinner } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { navigate, route } from "/src/router";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import { asJobType } from "$lib/job/jobs";
  import type { PackInfo } from "$lib/features/texture-packs";
  import TexturePackCard from "./TexturePackCard.svelte";

  const gameParam = $derived(route.params.game_name);
  let activeGame: SupportedGame | undefined = $state(undefined);

  $effect(() => {
    const activeGameFromParam = toSupportedGame(gameParam);
    if (activeGameFromParam) {
      activeGame = activeGameFromParam;
    }
  });

  let loaded = $state(false);
  let extractedPackInfo: Record<string, any> | undefined = $state(undefined);
  let availablePacks: PackInfo[] = $state([]);
  let availablePacksOriginal: PackInfo[] = $state([]);
  let addingPack = $state(false);
  let packAddingError = $state("");

  onMount(async () => {
    await updatePackList();
    loaded = true;
  });

  async function updatePackList() {
    if (!activeGame) {
      return;
    }
    availablePacks = [];
    availablePacksOriginal = [];
    let currentlyEnabledPacks = await getEnabledTexturePacks(activeGame);
    extractedPackInfo = await listExtractedTexturePackInfo(activeGame);
    if (extractedPackInfo === undefined) {
      return;
    }
    // Finalize `availablePacks` list
    // - First, cleanup any packs that were enabled but can no longer be found
    let cleanupPackList = [];
    let filteredCurrentlyEnabledPacks = [];
    for (const [packName, packInfo] of Object.entries(extractedPackInfo)) {
      if (!currentlyEnabledPacks.includes(packName)) {
        cleanupPackList.push(packName);
      } else {
        filteredCurrentlyEnabledPacks.push(packName);
      }
    }
    await cleanupEnabledTexturePacks(activeGame, cleanupPackList);
    // - secondly, add the ones that are enabled so they are at the top of the list
    for (const pack of currentlyEnabledPacks) {
      availablePacks.push({
        name: pack,
        enabled: true,
        toBeDeleted: false,
      });
    }
    // - lastly, add the rest that are available but not enabled
    for (const [packName, packInfo] of Object.entries(extractedPackInfo)) {
      if (!filteredCurrentlyEnabledPacks.includes(packName)) {
        availablePacks.push({
          name: packName,
          enabled: false,
          toBeDeleted: false,
        });
      }
    }
    availablePacks = availablePacks; // assignment for reactivity
    availablePacksOriginal = JSON.parse(JSON.stringify(availablePacks));
  }

  function areChangesPending(
    current: PackInfo[],
    original: PackInfo[],
  ): boolean {
    return JSON.stringify(current) !== JSON.stringify(original);
  }

  async function addNewTexturePack() {
    if (!activeGame) {
      return;
    }
    addingPack = true;
    packAddingError = "";
    const texturePackPath = await filePrompt(
      ["zip"],
      "ZIP",
      "Select a texture pack",
    );
    if (texturePackPath !== null) {
      const success = await extractNewTexturePack(activeGame, texturePackPath);
      if (success) {
        // if the user made any changes, attempt to restore them after
        let preexistingChanges = undefined;
        if (areChangesPending(availablePacks, availablePacksOriginal)) {
          preexistingChanges = JSON.parse(JSON.stringify(availablePacks));
        }
        await updatePackList();
        if (preexistingChanges !== undefined) {
          for (const preexisingPack of preexistingChanges) {
            for (const pack of availablePacks) {
              if (pack.name === preexisingPack.name) {
                pack.enabled = preexisingPack.enabled;
                pack.toBeDeleted = preexisingPack.toBeDeleted;
              }
            }
          }
          availablePacks = availablePacks;
        }
      } else {
        packAddingError = $_("features_textures_invalidPack");
      }
    }
    addingPack = false;
  }

  async function applyTexturePacks() {
    if (!activeGame) {
      return;
    }
    let enabledPacks: string[] = [];
    let packsToDelete: string[] = [];
    for (const pack of availablePacks) {
      if (pack.enabled) {
        enabledPacks.push(pack.name);
      } else if (pack.toBeDeleted) {
        packsToDelete.push(pack.name);
      }
    }
    navigate("/job/:job_type", {
      params: {
        job_type: asJobType("applyTexturePacks"),
      },
      search: {
        activeGame: activeGame,
        enabledPacks: JSON.stringify(enabledPacks),
        packsToDelete: JSON.stringify(packsToDelete),
      },
    });
  }

  function moveTexturePack(dst: number, src: number) {
    const temp = availablePacks[dst];
    availablePacks[dst] = availablePacks[src];
    availablePacks[src] = temp;
    availablePacks = availablePacks;
  }
</script>

<div class="flex flex-col h-full bg-[#1e1e1e]">
  {#if !loaded || !activeGame}
    <div class="flex flex-col h-full justify-center items-center">
      <Spinner color="yellow" size={"12"} />
    </div>
  {:else}
    <div class="pb-20 overflow-y-auto p-4">
      <div class="flex flex-row gap-2">
        <Button
          disabled={addingPack}
          outline
          class="flex-shrink border-solid rounded text-white hover:dark:text-slate-900 hover:bg-white font-semibold px-2 py-2"
          onclick={async () => {
            if (activeGame) {
              navigate(`/:game_name`, { params: { game_name: activeGame } });
            }
          }}
          aria-label={$_("features_backToGamePage_buttonAlt")}
        >
          <IconArrowLeft />
        </Button>
        <Button
          class="flex-shrink border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
          onclick={addNewTexturePack}
          aria-label={$_("features_textures_addNewPack_buttonAlt")}
          disabled={addingPack}
        >
          {#if addingPack}
            <Spinner class="mr-3" size="4" color="yellow" />
          {/if}
          {$_("features_textures_addNewPack")}</Button
        >
        {#if areChangesPending(availablePacks, availablePacksOriginal)}
          <Button
            disabled={addingPack}
            class="flex-shrink border-solid rounded bg-green-400 hover:bg-green-500 text-sm text-slate-900 font-semibold px-5 py-2"
            onclick={applyTexturePacks}
            aria-label={$_("features_textures_applyChanges_buttonAlt")}
            >{$_("features_textures_applyChanges")}</Button
          >
        {/if}
      </div>
      {#if packAddingError !== ""}
        <div class="flex flex-row font-bold mt-3">
          <Alert class="flex-grow text-red-400">
            {packAddingError}
          </Alert>
        </div>
      {/if}
      {#if availablePacks.length > 0}
        <div class="flex flex-row font-bold mt-3">
          <h2>{$_("features_textures_listHeading")}</h2>
        </div>
        <div class="flex flex-row text-sm">
          <p>
            {$_("features_textures_description")}
          </p>
        </div>
      {/if}
      {#each availablePacks as pack, packIndex}
        {#if !pack.toBeDeleted && extractedPackInfo && extractedPackInfo[pack.name] !== undefined}
          <div class="flex flex-row gap-2 mt-3">
            <TexturePackCard
              {packIndex}
              packInfo={pack}
              packMetadata={extractedPackInfo[pack.name]}
              allPackMetadata={extractedPackInfo}
              allPackInfo={availablePacks}
              onMovePack={moveTexturePack}
            ></TexturePackCard>
          </div>
        {/if}
      {/each}
      <div class="flex flex-row font-bold mt-3">
        <Alert class="flex-grow text-red-400">
          {$_("features_textures_largePackWarning")}
        </Alert>
      </div>
    </div>
  {/if}
</div>
