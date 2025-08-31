<script lang="ts">
  import {
    Alert,
    Button,
    Input,
    Label,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
  } from "flowbite-svelte";
  import IconPlus from "~icons/mdi/plus";
  import { _ } from "svelte-i18n";
  import { invalidateAll } from "$app/navigation";
  import { addModSource } from "$lib/rpc/features";
  import IconDeleteForever from "~icons/mdi/delete-forever";
  import { removeModSource } from "$lib/rpc/features";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
  const sources = $derived(data.sources);
  let newSourceURL = $state("");
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
      <Input type="url" id="default-input" bind:value={newSourceURL} />
    </div>
    <Button
      class="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-3 py-2 ml-2"
      disabled={!newSourceURL}
      onclick={async () => {
        await addModSource(newSourceURL);
        await invalidateAll();
      }}
      ><IconPlus
        class="text-xl"
        color="green"
        aria-label={$_("settings_mods_icon_addSource_buttonAltText")}
      />{$_("settings_mods_icon_addSource_buttonText")}</Button
    >
  </div>
  <div class="mt-2">
    <Table>
      <TableBody class="divide-y bg-slate-700">
        {#each sources as source, i}
          <TableBodyRow class="flex items-center bg-slate-700">
            <TableBodyCell
              class="px-4 whitespace-nowrap font-medium text-gray-900 dark:text-white text-wrap"
              >{source}</TableBodyCell
            >
            <TableBodyCell
              class="flex ml-auto justify-end px-4 whitespace-nowrap font-medium dark:text-white"
              ><Button
                class="p-2 hover:text-red-500 text-slate-800 dark:text-gray-200"
                onclick={async () => {
                  await removeModSource(source);
                  await invalidateAll();
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
  </div>
</div>
