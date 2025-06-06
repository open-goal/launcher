<script lang="ts">
  import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";
  import { getModSourcesData, refreshModSources } from "$lib/rpc/cache";
  import {
    addModSource,
    getModSources,
    removeModSource,
    type ModSource,
  } from "$lib/rpc/features";
  import {
    Label,
    Input,
    Button,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    Alert,
  } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import IconDeleteForever from "~icons/mdi/delete-forever";
  import IconPlus from "~icons/mdi/plus";

  let newSourceURL = "";
  let currentSources: ModSource[] = [];
  let currentSourceData: Record<string, ModSourceData> = {};

  let pageLoaded = false;

  async function refreshModSourceData() {
    currentSources = await getModSources();
    await refreshModSources();
    currentSourceData = await getModSourcesData();
  }

  onMount(async () => {
    await refreshModSourceData();
    pageLoaded = true;
  });
</script>

<div class="flex flex-col gap-2 mt-2">
  <Alert rounded={false} class="border-t-4 text-red-400">
    <span class="font-bold">{$_("settings_mods_warning_header")}</span>
    <br />
    <p>
      {$_("settings_mods_warning_description_part1")}
    </p>
    <p>
      {$_("settings_mods_warning_description_part2")}
    </p>
  </Alert>
  <div>
    <Label for="default-input" class="block mb-2 text-slate-200"
      >{$_("settings_mods_addSource_label")}</Label
    >
  </div>
  <div class="flex">
    <div class="grow">
      <Input id="default-input" bind:value={newSourceURL} />
    </div>
    <Button
      class="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-3 py-2 ml-2"
      disabled={newSourceURL === ""}
      onclick={async () => {
        await addModSource(newSourceURL, currentSourceData);
        await refreshModSourceData();
      }}
      ><IconPlus
        class="text-xl"
        color="green"
        aria-label={$_("settings_mods_icon_addSource_buttonAltText")}
      />{$_("settings_mods_icon_addSource_buttonText")}</Button
    >
  </div>
  <div class="mt-2">
    {#if pageLoaded && currentSources.length > 0}
      <Table>
        <TableBody class="divide-y bg-slate-700">
          {#each currentSources as source, i}
            <TableBodyRow class="flex items-center bg-slate-700">
              <TableBodyCell
                class="px-4 whitespace-nowrap font-medium text-gray-900 dark:text-white text-wrap"
                >{source}</TableBodyCell
              >
              <TableBodyCell
                class="flex ml-auto justify-end px-4 whitespace-nowrap font-medium text-gray-900 dark:text-white text-red-600"
                ><Button
                  class="p-0 m-3 hover:text-red-500 text-slate-800 dark:text-gray-200"
                  onclick={async () => {
                    await removeModSource(source);
                    await refreshModSourceData();
                  }}
                  ><IconDeleteForever
                    class="text-xl"
                    color="red"
                    aria-label={$_("settings_mods_icon_deleteSource_altText")}
                  />
                  {$_("settings_mods_icon_deleteSource_buttonText")}</Button
                ></TableBodyCell
              >
            </TableBodyRow>
          {/each}
        </TableBody>
      </Table>
    {/if}
  </div>
</div>
