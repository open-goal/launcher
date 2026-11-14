<script lang="ts">
  import { refreshModSources } from "$lib/rpc/cache";
  import { addModSource, removeModSource } from "$lib/rpc/features";
  import {
    Label,
    Input,
    Button,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Alert,
  } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import IconDeleteForever from "~icons/mdi/delete-forever";
  import IconPlus from "~icons/mdi/plus";
  import { config } from "/src/state/config.svelte";

  let newSourceURL = $state("");
  const currentSources: string[] = $derived(config?.modSources!);

  let pageLoaded = $state(false);

  onMount(async () => {
    await refreshModSources();
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
      class="shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-3 py-2 ml-2"
      disabled={newSourceURL === ""}
      onclick={async () => {
        await addModSource(newSourceURL);
        await refreshModSources();
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
      <Table divClass="rounded-md bg-zinc-800 border border-zinc-600/70">
        <TableHead class="bg-zinc-800! text-zinc-200!">
          <TableHeadCell class="border-b! border-zinc-700/70!">
            {$_("settings_mods_addSource_label")}
          </TableHeadCell>
          <TableHeadCell class="w-32 border-b! border-zinc-700/70!"
          ></TableHeadCell>
        </TableHead>
        <TableBody class="divide-y divide-zinc-700/70!">
          {#each currentSources as source (source)}
            <TableBodyRow
              class="bg-zinc-800/80! text-white transition-colors hover:bg-zinc-700/80! border-y!"
            >
              <TableBodyCell class="px-6 py-3 font-medium break-all"
                >{source}</TableBodyCell
              >
              <TableBodyCell class="w-32 px-6 py-2 text-right">
                <Button
                  class="p-0 bg-transparent text-zinc-200 hover:text-red-500 focus:ring-0"
                  onclick={async () => {
                    await removeModSource(source);
                    await refreshModSources();
                  }}
                >
                  <IconDeleteForever
                    class="text-xl text-red-500"
                    aria-label={$_("settings_mods_icon_deleteSource_altText")}
                  />
                  {$_("settings_mods_icon_deleteSource_buttonText")}
                </Button>
              </TableBodyCell>
            </TableBodyRow>
          {/each}
        </TableBody>
      </Table>
    {/if}
  </div>
</div>
