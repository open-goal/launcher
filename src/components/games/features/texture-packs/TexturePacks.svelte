<!--
  - verify mod JSON file with a json schema https://docs.rs/jsonschema/latest/jsonschema/
 -->
<!-- NOTE - this does not attempt to verify that the user has not manually messed with the texture_replacements folder.
  This is no different than how we don't verify the user hasn't messed with goal_src -->

<!-- TODO - collecting rating metrics / number of users might be cool (same for mods) -->
<!-- TODO - instead of currently allowing full access - explicitly allow the install folder in the Rust layer https://docs.rs/tauri/1.4.1/tauri/scope/struct.FsScope.html -->

<script lang="ts">
  import { fromRoute, getInternalName, SupportedGame } from "$lib/constants";
  import {
    cleanupEnabledTexturePacks,
    getEnabledTexturePacks,
    setEnabledTexturePacks,
  } from "$lib/rpc/config";
  import {
    extractNewTexturePack,
    listExtractedTexturePackInfo,
    updateTexturePackData,
  } from "$lib/rpc/features";
  import { filePrompt } from "$lib/utils/file";
  import Icon from "@iconify/svelte";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import {
    Accordion,
    AccordionItem,
    Badge,
    Button,
    Card,
    Spinner,
    Toggle,
  } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { navigate, useParams } from "svelte-navigator";
  import GameJob from "../../job/GameJob.svelte";

  const params = useParams();
  export let activeGame: SupportedGame;

  let loaded = false;
  let extractedPackInfo: any = undefined;
  let availablePacks = [];
  let availablePacksOriginal = [];

  let addingPack = false;
  let gameJobToRun = undefined;

  onMount(async () => {
    // Figure out what game we are displaying
    if (
      $params["game_name"] !== undefined &&
      $params["game_name"] !== null &&
      $params["game_name"] !== ""
    ) {
      activeGame = fromRoute($params["game_name"]);
    } else {
      activeGame = SupportedGame.Jak1;
    }
    await update_pack_list();
    loaded = true;
  });

  async function update_pack_list() {
    availablePacks = [];
    availablePacksOriginal = [];
    let currentlyEnabledPacks = await getEnabledTexturePacks(
      getInternalName(activeGame)
    );
    extractedPackInfo = await listExtractedTexturePackInfo(
      getInternalName(activeGame)
    );
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
    await cleanupEnabledTexturePacks(
      getInternalName(activeGame),
      cleanupPackList
    );
    // - secondly, add the ones that are enabled so they are at the top of the list
    for (const pack of currentlyEnabledPacks) {
      availablePacks.push({
        name: pack,
        enabled: true,
      });
    }
    // - lastly, add the rest that are available but not enabled
    for (const [packName, packInfo] of Object.entries(extractedPackInfo)) {
      if (!filteredCurrentlyEnabledPacks.includes(packName)) {
        availablePacks.push({
          name: packName,
          enabled: false,
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
    tagName: string
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
    addingPack = true;
    const texturePackPath = await filePrompt(
      ["zip"],
      "ZIP",
      "Select a texture pack"
    );
    if (texturePackPath !== null) {
      const success = await extractNewTexturePack(
        getInternalName(activeGame),
        texturePackPath
      );
      // TODO - display error if `false`
      if (success) {
        await update_pack_list();
      }
    }
    addingPack = false;
  }

  // TODO - implement ordering

  async function applyTexturePacks() {
    let enabledPacks = [];
    for (const pack of availablePacks) {
      if (pack.enabled) {
        enabledPacks.push(pack.name);
      }
    }
    await setEnabledTexturePacks(getInternalName(activeGame), enabledPacks);
    // TODO - move this into a job
    await updateTexturePackData(getInternalName(activeGame));
  }

  async function gameJobFinished() {
    gameJobToRun = undefined;
  }
</script>

<div class="flex flex-col h-full bg-slate-900">
  {#if !loaded}
    <div class="flex flex-col h-full justify-center items-center">
      <Spinner color="yellow" size={"12"} />
    </div>
    <!-- TODO - make a generic features page -->
  {:else if gameJobToRun !== undefined}
    <GameJob
      {activeGame}
      jobType={gameJobToRun}
      on:jobFinished={gameJobFinished}
    />
  {:else}
    <div class="pb-20 overflow-y-auto p-4">
      <div class="flex flex-row gap-2">
        <Button
          outline
          class="flex-shrink border-solid rounded text-white hover:dark:text-slate-900 hover:bg-white font-semibold px-2 py-2"
          on:click={async () =>
            navigate(`/${getInternalName(activeGame)}`, { replace: true })}
          aria-label="back to game page"
          ><Icon
            icon="material-symbols:arrow-left-alt"
            width="20"
            height="20"
          /></Button
        >
        <Button
          class="flex-shrink border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
          on:click={addNewTexturePack}
          aria-label="add a new texture pack"
          disabled={addingPack}
        >
          {#if addingPack}
            <Spinner class="mr-3" size="4" color="white" />
          {/if}
          Add New Pack</Button
        >
        {#if pending_changes(availablePacks, availablePacksOriginal)}
          <Button
            class="flex-shrink border-solid rounded bg-green-400 hover:bg-green-500 text-sm text-slate-900 font-semibold px-5 py-2"
            on:click={applyTexturePacks}
            aria-label="apply texture changes">Apply Texture Changes</Button
          >
        {/if}
      </div>
      <div class="flex flex-row font-bold mt-3">
        <h2>Currently Added Packs</h2>
      </div>
      <div class="flex flex-row text-sm">
        <p>
          You can enable as many packs as you want, but if multiple packs
          replace the same file the order matters. For example if two packs
          replace the grass, the first pack in the list takes precedence.
        </p>
      </div>
      {#each availablePacks as pack, packIndex}
        <div class="flex flex-row gap-2 mt-3">
          <!-- Placeholder image -->
          <Card
            img={convertFileSrc(extractedPackInfo[pack.name]["coverImagePath"])}
            horizontal
            class="texture-pack-card max-w-none md:max-w-none basis-full"
            padding="md"
          >
            <div class="flex flex-row mt-auto">
              <h2 class="text-xl font-bold tracking-tight text-white">
                {extractedPackInfo[pack.name]["name"]}
                <span class="text-xs text-gray-500" />
              </h2>
            </div>
            <p class="font-bold text-xs text-gray-500">
              {extractedPackInfo[pack.name]["version"]} by {extractedPackInfo[
                pack.name
              ]["author"]}
            </p>
            <p class="font-bold text-gray-500 text-xs">
              {extractedPackInfo[pack.name]["releaseDate"]}
            </p>
            <p class="font-bold text-gray-500 text-xs">
              Files replaced - {num_textures_in_pack(pack.name)}
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
              <Toggle
                bind:checked={pack.enabled}
                class="m-0"
                color="orange"
                on:change={() => {
                  console.log(availablePacks);
                  console.log(availablePacksOriginal);
                }}
              />
              {#if pack.enabled}
                {#if packIndex !== 0}
                  <Button
                    outline
                    class="!p-1.5 rounded-md border-blue-500 text-blue-500 hover:bg-blue-600"
                    aria-label="move texture pack up in order"
                  >
                    <Icon
                      icon="material-symbols:arrow-upward"
                      width="15"
                      height="15"
                    />
                  </Button>
                {/if}
                {#if packIndex !== availablePacks.length - 1}
                  <Button
                    outline
                    class="!p-1.5 rounded-md border-blue-500 text-blue-500 hover:bg-blue-600"
                    aria-label="move texture pack down in order"
                  >
                    <Icon
                      icon="material-symbols:arrow-downward"
                      width="15"
                      height="15"
                    />
                  </Button>
                {/if}
              {/if}
              <!-- TODO - implement delete -->
              <Button
                outline
                class="!p-1.5 rounded-md border-red-500 text-red-500 hover:bg-red-600"
                aria-label="delete texture pack"
              >
                <Icon icon="material-symbols:delete" width="15" height="15" />
              </Button>
            </div>
            <!-- double computation, TODO - separate component -->
            {#if find_pack_conflicts(pack.name).size > 0}
              <Accordion flush class="mt-2">
                <AccordionItem paddingFlush="p-2">
                  <span
                    slot="header"
                    class="flex gap-2 text-yellow-300 text-sm"
                  >
                    <svg
                      aria-hidden="true"
                      class="w-5 h-5"
                      fill="currentColor"
                      viewBox="0 0 20 20"
                      xmlns="http://www.w3.org/2000/svg"
                      ><path
                        fill-rule="evenodd"
                        d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                        clip-rule="evenodd"
                      /></svg
                    >
                    <span> Conflicts Detected!</span>
                  </span>
                  <div slot="arrowup" />
                  <div slot="arrowdown" />
                  <pre class="mb-2 text-gray-500 dark:text-gray-400 text-xs">{[
                      ...find_pack_conflicts(pack.name),
                    ]
                      .join("\n")
                      .trim()}</pre>
                </AccordionItem>
              </Accordion>
            {/if}
          </Card>
        </div>
      {/each}
    </div>
  {/if}
</div>