<script lang="ts">
  import type { PackInfo, PackMetadata } from "$lib/features/texture-packs";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import {
    Card,
    Badge,
    Button,
    Accordion,
    AccordionItem,
    Toggle,
  } from "flowbite-svelte";
  import IconArrowUp from "~icons/mdi/arrow-up";
  import IconArrowDown from "~icons/mdi/arrow-down";
  import IconDelete from "~icons/mdi/delete";
  import { _ } from "svelte-i18n";
  import { onMount } from "svelte";

  let {
    packIndex,
    packInfo,
    packMetadata,
    allPackMetadata,
    allPackInfo,
    onMovePack,
  }: {
    packIndex: number;
    packInfo: PackInfo;
    packMetadata: PackMetadata;
    allPackMetadata: Record<string, PackMetadata>;
    allPackInfo: PackInfo[];
    onMovePack: (dst: number, src: number) => void;
  } = $props();

  let packConflicts: Set<String> = $state(new Set());

  onMount(() => {
    packConflicts = findPackConflicts();
  });

  function findPackConflicts(): Set<String> {
    let conflicts: Set<String> = new Set();
    for (const filePath of packMetadata.fileList) {
      for (const [packName, packMetadata] of Object.entries(allPackMetadata)) {
        if (packName === packInfo.name) {
          continue;
        }
        if (packMetadata.fileList.includes(filePath)) {
          conflicts.add(filePath);
        }
      }
    }
    return conflicts;
  }

  function numTexturesInPack(): number {
    return packMetadata.fileList.length;
  }

  function tagNameToColor(
    tagName: string,
  ):
    | undefined
    | "red"
    | "yellow"
    | "green"
    | "indigo"
    | "purple"
    | "pink"
    | "blue"
    | "gray"
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
      return "gray";
    }
  }
</script>

<Card
  horizontal={true}
  img={convertFileSrc(packMetadata.coverImagePath)}
  size="xl"
>
  <div class="w-full pl-4 py-2">
    <h2 class="text-xl font-bold tracking-tight text-white">
      {packMetadata.name}
      <span class="text-xs text-gray-500"></span>
    </h2>
    <p class="font-bold text-xs text-gray-500">
      {packMetadata.version} by {packMetadata.author}
    </p>
    <p class="font-bold text-gray-500 text-xs">
      {packMetadata.releaseDate}
    </p>
    <p class="font-bold text-gray-500 text-xs">
      {$_("features_textures_replacedCount")} - {numTexturesInPack()}
    </p>
    <p class="mt-2 mb-4 font-normal text-gray-400 leading-tight">
      {packMetadata.description}
    </p>
    {#if packMetadata.tags.length > 0}
      <div class="flex flex-row gap-2">
        {#each packMetadata.tags as tag}
          <Badge border color={tagNameToColor(tag)}>{tag}</Badge>
        {/each}
      </div>
    {/if}
    <div class="mt-2 flex flex-row gap-2">
      <Toggle color="orange" bind:checked={packInfo.enabled}
        >{packInfo.enabled
          ? $_("features_textures_enabled")
          : $_("features_textures_disabled")}</Toggle
      >
    </div>
    <div class="mt-2 flex flex-row gap-2">
      {#if packInfo.enabled}
        {#if packIndex !== 0}
          <Button
            outline
            class="!p-1.5 rounded-md border-blue-500 text-blue-500 hover:bg-blue-600"
            aria-label={$_("features_textures_moveUp_buttonAlt")}
            onclick={() => {
              onMovePack(packIndex - 1, packIndex);
            }}
          >
            <IconArrowUp />
          </Button>
        {/if}
        {#if packIndex !== allPackInfo.length - 1}
          <Button
            outline
            class="!p-1.5 rounded-md border-blue-500 text-blue-500 hover:bg-blue-600"
            aria-label={$_("features_textures_moveDown_buttonAlt")}
            onclick={() => {
              onMovePack(packIndex + 1, packIndex);
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
          packInfo.toBeDeleted = true;
          packInfo.enabled = false;
        }}
      >
        <IconDelete />
      </Button>
    </div>
    {#if packConflicts.size > 0}
      <Accordion flush class="mt-2">
        <AccordionItem class="p-2">
          {#snippet header()}{$_(
              "features_textures_conflictsDetected",
            )}{/snippet}
          <p class="mb-2 text-gray-500 dark:text-gray-400 text-xs">
            {[...packConflicts].join("\n").trim()}
          </p>
        </AccordionItem>
      </Accordion>
    {/if}
  </div>
</Card>
