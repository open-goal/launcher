<script>
  import { installUpdate } from "@tauri-apps/api/updater";
  import { relaunch } from "@tauri-apps/api/process";
  import {
    Button,
    Spinner,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Toggle,
  } from "flowbite-svelte";
  import { UpdateStore } from "$lib/stores/AppStore";
  import Icon from "@iconify/svelte";

  $: launcherUpdateInfo = $UpdateStore?.launcher;

  let updating = false;
  let showChanges = false;
  let showDependencyChanges = false;

  // TODO - add the timestamp, tauri doesn't use an ISO timestamp!

  async function updateHandler() {
    updating = true;
    await installUpdate();
    await relaunch();
  }
</script>

<div class="flex flex-col h-full bg-slate-900 p-4 gap-3">
  {#if $UpdateStore.launcher.updateAvailable}
    <h1 class="font-semibold text-xl text-orange-500">
      Launcher Update Available
    </h1>
    <p>
      Version: <strong>{launcherUpdateInfo.versionNumber}</strong>
    </p>
    <p class="text-sm">
      View the changes below and click the button to update to the latest
      version. The launcher will restart when finished.
    </p>
    <div class="flex flex-row mt-1 gap-3">
      <Button
        btnClass="border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
        on:click={async () => await updateHandler()}
        disabled={updating}
      >
        {#if updating}
          <Spinner class="mr-3" size="4" color="white" />
        {/if}
        Update Launcher
      </Button>
      <Button
        btnClass="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2"
        on:click={() => (showChanges = !showChanges)}>View Changelog</Button
      >
      <Toggle checked={showDependencyChanges}>Dependency Changes</Toggle>
    </div>
    {#if showChanges}
      <Table hoverable={true}>
        <caption
          class="p-2 font-semibold text-right text-gray-900 bg-white dark:text-white dark:bg-gray-800"
        />
        <TableHead>
          <TableHeadCell>Contributor</TableHeadCell>
          <TableHeadCell>Description</TableHeadCell>
          <TableHeadCell>Pull Request</TableHeadCell>
        </TableHead>
        <TableBody tableBodyClass="divide-y">
          {#each launcherUpdateInfo.changeLog["changes"].filter((note) => {
            if (showDependencyChanges) {
              return true;
            } else {
              return !note.contributor.includes("dependabot");
            }
          }) as note}
            <TableBodyRow>
              <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-bold"
                >{note.contributor}</TableBodyCell
              >
              <TableBodyCell tdClass="px-6 py-2"
                >{note.description}</TableBodyCell
              >
              <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium">
                <a
                  class="inline-block"
                  href={note.pullRequestUrl}
                  target="_blank"
                  rel="noreferrer"
                  ><Icon
                    class="inline"
                    icon="mdi:github"
                    width="24"
                    height="24"
                  /></a
                >
              </TableBodyCell>
            </TableBodyRow>
          {/each}
        </TableBody>
      </Table>
    {/if}
  {:else}
    <h1 class="font-semibold text-xl text-orange-500">You're Up to Date!</h1>
  {/if}
</div>
