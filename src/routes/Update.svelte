<script>
  import { installUpdate } from "@tauri-apps/api/updater";
  import { relaunch } from "@tauri-apps/api/process";
  import {
    Button,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { UpdateStore } from "$lib/stores/AppStore";

  let disabled = false;
  let notesBool = true;

  async function updateHandler() {
    disabled = true;
    await installUpdate();
    await relaunch();
  }

  $: jakProjectNotes = $UpdateStore?.jak_project;
  $: launcherNotes = $UpdateStore?.launcher;
</script>

<div class="ml-20">
  <div class="flex flex-col h-[560px] max-h-[560px] p-8 gap-2">
    {#if $UpdateStore.shouldUpdate}
      <Table hoverable={true}>
        <caption
          class="p-2 font-semibold text-right text-gray-900 bg-white dark:text-white dark:bg-gray-800"
        >
          <Button
            class="!rounded-none w-48"
            on:click={() => (notesBool = !notesBool)}
            >Read {notesBool ? "Launcher" : "Jak-Project"} updates</Button
          >
        </caption>
        <TableHead>
          <TableHeadCell>Contributor</TableHeadCell>
          <TableHeadCell>Description</TableHeadCell>
          <TableHeadCell>Pull Request</TableHeadCell>
        </TableHead>
        <TableBody class="divide-y">
          {#each notesBool ? jakProjectNotes : launcherNotes as note}
            <TableBodyRow>
              <TableBodyCell>{note.contributor}</TableBodyCell>
              <TableBodyCell tdClass="overflow-clip"
                >{note.description}</TableBodyCell
              >
              <TableBodyCell>
                <Button
                  class="!rounded-none"
                  target="_blank"
                  rel="noreferrer noopener"
                  outline
                  href={note.pullRequestUrl}>Github</Button
                >
              </TableBodyCell>
            </TableBodyRow>
          {/each}
        </TableBody>
      </Table>
      <div class="flex flex-col">
        <Button
          class="!rounded-none !bg-[#222222] border-none !text-white hover:!text-yellow-300 !text-2xl"
          on:click={async () => await updateHandler()}
          {disabled}>Update</Button
        >
      </div>
    {:else}
      <p>You're Up to Date!</p>
    {/if}
  </div>
</div>
