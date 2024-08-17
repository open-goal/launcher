<script lang="ts">
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

  let pageLoaded = false;

  onMount(async () => {
    currentSources = await getModSources();
    pageLoaded = true;
  });
</script>

<div class="flex flex-col gap-2 mt-2">
  <Alert color="red" rounded={false} class="border-t-4">
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
    <Label for="default-input" class="block mb-2"
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
      on:click={async () => {
        await addModSource(newSourceURL);
        currentSources = await getModSources();
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
      <Table striped={true}>
        <TableBody tableBodyClass="divide-y">
          {#each currentSources as source, i}
            <TableBodyRow class="flex items-center">
              <TableBodyCell
                tdClass="px-6 whitespace-nowrap font-medium text-gray-900 dark:text-white text-wrap"
                >{source}</TableBodyCell
              >
              <TableBodyCell
                tdClass="flex ml-auto justify-end px-6 whitespace-nowrap font-medium text-gray-900 dark:text-white text-red-600"
                ><Button
                  class="p-0 m-3 hover:text-red-500"
                  on:click={async () => {
                    await removeModSource(i);
                    currentSources = await getModSources();
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
