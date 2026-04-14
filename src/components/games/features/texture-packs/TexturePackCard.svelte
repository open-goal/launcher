<script lang="ts">
  import type { PackInfo, PackMetadata } from "$lib/features/texture-packs";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import {
    Card,
    Badge,
    Button,
    Accordion,
    AccordionItem,
    Checkbox,
  } from "flowbite-svelte";
  import IconArrowUp from "~icons/mdi/arrow-up";
  import IconArrowDown from "~icons/mdi/arrow-down";
  import IconDelete from "~icons/mdi/delete";
  import { _ } from "svelte-i18n";
  import { onMount } from "svelte";
  import placeholder from "$assets/images/mod-thumbnail-placeholder.webp";

  let {
    packIndex,
    packInfo,
    packMetadata,
    allPackMetadata,
    allPackInfo,
    onMovePack,
    onToggleEnabled,
    onDeletePack,
  }: {
    packIndex: number;
    packInfo: PackInfo;
    packMetadata: PackMetadata;
    allPackMetadata: Record<string, PackMetadata>;
    allPackInfo: PackInfo[];
    onMovePack: (dst: number, src: number) => void;
    onToggleEnabled: (packName: string, enabled: boolean) => void;
    onDeletePack: (packName: string) => void;
  } = $props();

  let packConflicts: Set<string> = $state(new Set());

  onMount(() => {
    packConflicts = findPackConflicts();
  });

  function findPackConflicts(): Set<string> {
    let conflicts: Set<string> = new Set();
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

  const coverImg = $derived(
    packMetadata?.coverImagePath
      ? convertFileSrc(packMetadata.coverImagePath)
      : placeholder,
  );

  const numTexturesInPack = $derived(packMetadata?.fileList.length);
  const index = $derived(packIndex);
</script>

<Card
  horizontal={true}
  img={coverImg}
  size="xl"
  class="flex flex-row flow-grow flex-1"
>
  <div class="flex flex-col flex-1 mx-4 my-2">
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
      {$_("features_textures_replacedCount")} - {numTexturesInPack}
    </p>
    <p class="mt-2 mb-4 font-normal text-gray-400 leading-tight">
      {packMetadata.description}
    </p>

    {#if packMetadata.tags}
      <div class="flex flex-row gap-2 pb-4">
        {#each packMetadata.tags as tag}
          <Badge border color={tagNameToColor(tag)}>{tag}</Badge>
        {/each}
      </div>
    {/if}

    <div class="flex flex-row gap-2">
      <Button
        outline
        class="rounded-md border-blue-500 text-blue-500 hover:bg-blue-600"
        aria-label={$_("features_textures_moveUp_buttonAlt")}
        onclick={() => {
          onMovePack(index - 1, index);
        }}
        disabled={!packInfo.enabled || allPackInfo.length <= 1 || index == 0}
      >
        <IconArrowUp />
      </Button>

      <Button
        outline
        class="rounded-md border-blue-500 text-blue-500 hover:bg-blue-600"
        aria-label={$_("features_textures_moveDown_buttonAlt")}
        onclick={() => {
          onMovePack(index + 1, index);
        }}
        disabled={!packInfo.enabled ||
          allPackInfo.length <= 1 ||
          index == allPackInfo.length - 1}
      >
        <IconArrowDown />
      </Button>

      <Button
        outline
        class="rounded-md border-red-500 text-red-500 hover:bg-red-600"
        aria-label={$_("features_textures_deletePack_buttonAlt")}
        onclick={() => {
          onDeletePack(packInfo.name);
        }}
      >
        <IconDelete />
      </Button>

      <Checkbox
        color="orange"
        class="h-full w-14 cursor-pointer"
        checked={packInfo.enabled}
        onchange={() => {
          onToggleEnabled(packInfo.name, !packInfo.enabled);
        }}
      >
        {packInfo.enabled
          ? $_("features_textures_enabled")
          : $_("features_textures_disabled")}
      </Checkbox>
    </div>

    {#if packConflicts?.size > 0}
      <Accordion flush>
        <AccordionItem>
          {#snippet header()}
            {$_("features_textures_conflictsDetected")}
          {/snippet}
          <p class="text-gray-500 dark:text-gray-400 text-xs">
            {[...packConflicts].join("\n").trim()}
          </p>
        </AccordionItem>
      </Accordion>
    {/if}
  </div>
</Card>
