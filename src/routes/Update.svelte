<script>
  import { relaunch } from "@tauri-apps/plugin-process";
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
  import IconGitHub from "~icons/mdi/github";
  import { _ } from "svelte-i18n";

  $: launcherUpdateInfo = $UpdateStore?.launcher;

  let updating = false;
  let showChanges = false;
  let showDependencyChanges = false;

  // TODO - add the timestamp, tauri doesn't use an ISO timestamp!

  async function updateHandler() {
    updating = true;
    // TODO - migrate!
    // await installUpdate();
    await relaunch();
  }
</script>

<!-- TODO - the pb-20 is a bit of a hack (also done on settings) figure out the actual problem with the DOM -->
<div class="flex flex-col h-full bg-slate-900 p-4 gap-3 overflow-y-auto pb-20">
  {#if $UpdateStore.launcher.updateAvailable}
    <h1 class="font-semibold text-xl text-orange-500">
      {$_("settings_tabs_general")}
    </h1>
    <p>
      {$_("update_versionLabel")}:&nbsp;<strong
        >{launcherUpdateInfo.versionNumber}</strong
      >
    </p>
    <p class="text-sm">
      {$_("update_description")}
    </p>
    <div class="flex flex-row mt-1 gap-3">
      <Button
        class="border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
        onclick={async () => await updateHandler()}
        disabled={updating}
      >
        {#if updating}
          <Spinner class="mr-3" size="4" color="white" />
        {/if}
        {$_("update_button_doUpdate")}
      </Button>
      <Button
        class="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2"
        onclick={() => (showChanges = !showChanges)}
        >{$_("update_button_viewChangelog")}</Button
      >
      <Toggle
        checked={showDependencyChanges}
        color="orange"
        on:change={(evt) => {
          showDependencyChanges = evt.target.checked;
        }}>{$_("update_button_hideDependencyChanges")}</Toggle
      >
    </div>
    {#if showChanges}
      <Table hoverable={true}>
        <caption
          class="p-2 font-semibold text-right text-gray-900 bg-white dark:text-white dark:bg-gray-800"
        ></caption>
        <TableHead>
          <TableHeadCell
            >{$_("update_changelog_header_contributor")}</TableHeadCell
          >
          <TableHeadCell
            >{$_("update_changelog_header_description")}</TableHeadCell
          >
          <TableHeadCell
            >{$_("update_changelog_header_pullRequest")}</TableHeadCell
          >
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
                >
                  <IconGitHub />
                </a>
              </TableBodyCell>
            </TableBodyRow>
          {/each}
        </TableBody>
      </Table>
    {/if}
  {:else}
    <h1 class="font-semibold text-xl text-orange-500">
      {$_("update_alreadyUpToDate")}
    </h1>
  {/if}
</div>
