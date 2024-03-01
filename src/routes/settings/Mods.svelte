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
  } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";

  let newSourceURL = "";
  let currentSources: ModSource[] = [];

  let pageLoaded = false;

  onMount(async () => {
    currentSources = await getModSources();
    pageLoaded = true;
  });
</script>

<div class="flex flex-col gap-2 mt-2">
  <div>
    <Label for="default-input" class="block mb-2"
      >{$_("settings_mods_newSourceButton")}</Label
    >
  </div>
  <div class="flex">
    <div class="grow">
      <Input id="default-input" bind:value={newSourceURL} />
    </div>
    <!-- TODO - check that the URL returns a 200 response when adding -->
    <Button
      class="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2 ml-2"
      disabled={newSourceURL === ""}
      on:click={async () => {
        await addModSource(newSourceURL);
        currentSources = await getModSources();
      }}>ICON</Button
    >
  </div>
  <div class="mt-2">
    {#if pageLoaded && currentSources.length > 0}
      <Table striped={true}>
        <TableBody tableBodyClass="divide-y">
          {#each currentSources as source, i}
            <TableBodyRow>
              <TableBodyCell
                ><a href={source.url} target="_blank" rel="noopener noreferrer"
                  >{source.url}</a
                ></TableBodyCell
              >
              <TableBodyCell
                ><Button
                  on:click={async () => {
                    await removeModSource(i);
                    currentSources = await getModSources();
                  }}>Delete</Button
                ></TableBodyCell
              >
            </TableBodyRow>
          {/each}
        </TableBody>
      </Table>
    {:else}
      TODO loading
    {/if}
  </div>
</div>
