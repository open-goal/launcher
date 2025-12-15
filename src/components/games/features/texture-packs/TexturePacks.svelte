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
  import IconArrowUp from "~icons/mdi/arrow-up";
  import IconArrowDown from "~icons/mdi/arrow-down";
  import IconDelete from "~icons/mdi/delete";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import {
    Accordion,
    AccordionItem,
    Alert,
    Badge,
    Button,
    Card,
    Spinner,
  } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";

  import { navigate, route } from "/src/router";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import { asJobType } from "$lib/job/jobs";
  import { act } from "@testing-library/svelte";

  const gameParam = $derived(route.params.game_name);
  let activeGame: SupportedGame | undefined = $state(undefined);

  $effect(() => {
    const activeGameFromParam = toSupportedGame(gameParam);
    if (activeGameFromParam) {
      activeGame = activeGameFromParam;
    }
  });

  let loaded = $state(false);
  let extractedPackInfo: any = $state(undefined);
  let availablePacks = $state([]);
  let availablePacksOriginal = $state([]);
  let addingPack = $state(false);
  let packAddingError = $state("");

  onMount(async () => {
    await update_pack_list();
    loaded = true;
  });

  async function update_pack_list() {
    if (!activeGame) {
      return;
    }
    availablePacks = [];
    availablePacksOriginal = [];
    let currentlyEnabledPacks = await getEnabledTexturePacks(activeGame);
    extractedPackInfo = await listExtractedTexturePackInfo(activeGame);
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

  function pending_changes(current, original): boolean {
    return JSON.stringify(current) !== JSON.stringify(original);
  }

  function num_textures_in_pack(packName: string): number {
    return extractedPackInfo[packName]["fileList"].length;
  }

  function tag_name_to_color(
    tagName: string,
  ):
    | "none"
    | "red"
    | "yellow"
    | "green"
    | "indigo"
    | "purple"
    | "pink"
    | "blue"
    | "dark"
    | "primary" {
    if (
      tagName === "enhancement" ||
      tagName === "overhaul" ||
      tagName === "highres"
    ) {
      return "indigo";
    } else if (tagName == "parody" || tagName === "themed") {
      return "pink";
    } else if (tagName === "mods") {
      return "purple";
    } else {
      return "dark";
    }
  }

  // Iterate through all enabled packs, flag and files that are in the relevant pack
  function find_pack_conflicts(relevantPackName: string): Set<String> {
    let conflicts: Set<String> = new Set();
    for (const filePath of extractedPackInfo[relevantPackName]["fileList"]) {
      for (const [packName, packInfo] of Object.entries(extractedPackInfo)) {
        if (packName === relevantPackName) {
          continue;
        }
        if (packInfo["fileList"].includes(filePath)) {
          conflicts.add(filePath);
        }
      }
    }
    return conflicts;
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
        if (pending_changes(availablePacks, availablePacksOriginal)) {
          preexistingChanges = JSON.parse(JSON.stringify(availablePacks));
        }
        await update_pack_list();
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
          onclick={async () =>
            navigate(`/:game_name`, { params: { game_name: activeGame } })}
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
        {#if pending_changes(availablePacks, availablePacksOriginal)}
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
        {#if !pack.toBeDeleted}
          <div class="flex flex-row gap-2 mt-3">
            <Card
              horizontal={true}
              img={convertFileSrc(
                extractedPackInfo[pack.name]["coverImagePath"],
              )}
              size="xl"
            >
              <div class="w-full pl-4 py-2">
                <h2 class="text-xl font-bold tracking-tight text-white">
                  {extractedPackInfo[pack.name]["name"]}
                  <span class="text-xs text-gray-500"></span>
                </h2>
                <p class="font-bold text-xs text-gray-500">
                  {extractedPackInfo[pack.name]["version"]} by {extractedPackInfo[
                    pack.name
                  ]["author"]}
                </p>
                <p class="font-bold text-gray-500 text-xs">
                  {extractedPackInfo[pack.name]["releaseDate"]}
                </p>
                <p class="font-bold text-gray-500 text-xs">
                  {$_("features_textures_replacedCount")} - {num_textures_in_pack(
                    pack.name,
                  )}
                </p>
                <p class="mt-2 mb-4 font-normal text-gray-400 leading-tight">
                  {extractedPackInfo[pack.name]["description"]}
                </p>
                {#if extractedPackInfo[pack.name]["tags"].length > 0}
                  <div class="flex flex-row gap-2">
                    {#each extractedPackInfo[pack.name]["tags"] as tag}
                      <Badge border color={tag_name_to_color(tag)}>{tag}</Badge>
                    {/each}
                  </div>
                {/if}
                <!-- Buttons -->
                <div class="mt-2 flex flex-row gap-2">
                  <!-- TODO: this button should be a toggle tbh -->
                  <Button
                    size={"xs"}
                    color={pack.enabled ? "green" : "red"}
                    onclick={() => {
                      pack.enabled = !pack.enabled;
                    }}
                  >
                    {pack.enabled
                      ? $_("features_textures_enabled")
                      : $_("features_textures_disabled")}
                  </Button>
                  {#if pack.enabled}
                    {#if packIndex !== 0}
                      <Button
                        outline
                        class="!p-1.5 rounded-md border-blue-500 text-blue-500 hover:bg-blue-600"
                        aria-label={$_("features_textures_moveUp_buttonAlt")}
                        onclick={() => {
                          moveTexturePack(packIndex - 1, packIndex);
                        }}
                      >
                        <IconArrowUp />
                      </Button>
                    {/if}
                    {#if packIndex !== availablePacks.length - 1}
                      <Button
                        outline
                        class="!p-1.5 rounded-md border-blue-500 text-blue-500 hover:bg-blue-600"
                        aria-label={$_("features_textures_moveDown_buttonAlt")}
                        onclick={() => {
                          moveTexturePack(packIndex + 1, packIndex);
                        }}
                      >
                        <IconArrowDown />
                      </Button>
                    {/if}
                  {/if}
                  <Button
                    outline
                    class="!p-1.5 rounded-md border-red-500 text-red-500 hover:bg-red-600"
                    aria-label={$_("features_textures_deletePack_buttonAlt")}
                    onclick={() => {
                      pack.toBeDeleted = true;
                      pack.enabled = false;
                    }}
                  >
                    <IconDelete />
                  </Button>
                </div>
                <!-- double computation, TODO - separate component -->
                <!-- TODO: this could be more elegant -->
                {#if find_pack_conflicts(pack.name).size > 0}
                  <Accordion flush class="mt-2">
                    <AccordionItem class="p-2">
                      {#snippet header()}{$_(
                          "features_textures_conflictsDetected",
                        )}{/snippet}
                      <p class="mb-2 text-gray-500 dark:text-gray-400 text-xs">
                        {[...find_pack_conflicts(pack.name)].join("\n").trim()}
                      </p>
                    </AccordionItem>
                  </Accordion>
                {/if}
              </div>
            </Card>
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
