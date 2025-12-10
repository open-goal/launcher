<script lang="ts">
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
  import IconGitHub from "~icons/mdi/github";
  import { _ } from "svelte-i18n";
  import { onMount } from "svelte";
  import { check, Update } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { exceptionLog } from "$lib/rpc/logging";

  interface ChangelogEntry {
    contributor: string;
    description: string;
    pullRequestUrl: string;
  }

  interface Changelog {
    changes: ChangelogEntry[];
  }

  let updating = false;
  let availableUpdate: null | Update = null;
  let availableUpdateChangelog: Changelog = {
    changes: [],
  };
  let showChanges = false;
  let showDependencyChanges = false;

  // TODO - add the timestamp, tauri doesn't use an ISO timestamp!

  onMount(async () => {
    const update = await check();
    if (update) {
      availableUpdate = update;
      if (availableUpdate.body) {
        try {
          const updateJson = JSON.parse(availableUpdate.body);
          if (Object.keys(updateJson).includes("changes")) {
            availableUpdateChangelog.changes = updateJson.changes;
          }
        } catch (e) {
          exceptionLog(
            `Could not parse changelog JSON from release metadata - ${JSON.stringify(
              availableUpdate.rawJson,
            )}`,
            e,
          );
        }
      }
    }
  });

  async function updateHandler() {
    if (availableUpdate === null) {
      return;
    }
    updating = true;
    await availableUpdate.downloadAndInstall();
    await relaunch();
  }
</script>

<div class="flex flex-col h-full bg-[#141414] p-4">
  {#if availableUpdate !== null}
    <h1 class="font-semibold text-xl text-orange-500">
      {$_("header_updateAvailable")}
    </h1>
    <p class="mt-2">
      {$_("update_versionLabel")}:&nbsp;<strong
        >{availableUpdate.version}</strong
      >
    </p>
    <p class="text-sm mt-2">
      {$_("update_description")}
    </p>
    <div class="flex flex-row mt-3 gap-3">
      <Button
        class="border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
        onclick={async () => await updateHandler()}
        disabled={updating}
      >
        {#if updating}
          <Spinner class="mr-3" size="4" color="primary" />
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
        onchange={(evt: any) => {
          showDependencyChanges = evt.target.checked;
        }}>{$_("update_button_hideDependencyChanges")}</Toggle
      >
    </div>
    {#if showChanges}
      <Table hoverable={true} class="mt-3">
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
        <TableBody class="divide-y">
          {#each availableUpdateChangelog.changes.filter((note) => {
            if (showDependencyChanges) {
              return true;
            } else {
              return !note.contributor.includes("dependabot");
            }
          }) as note}
            <TableBodyRow>
              <TableBodyCell class="px-6 py-2 whitespace-nowrap font-bold"
                >{note.contributor}</TableBodyCell
              >
              <TableBodyCell
                class="px-6 py-2 max-w-80 whitespace-normal break-words"
                >{note.description}</TableBodyCell
              >
              <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
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
