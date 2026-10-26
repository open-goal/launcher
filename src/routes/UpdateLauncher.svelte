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
  import IconCalendar from "~icons/mdi/CalendarMonth";
  import IconGitHub from "~icons/mdi/github";
  import { _ } from "svelte-i18n";
  import { onMount } from "svelte";
  import { check, Update } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { exceptionLog } from "$lib/rpc/logging";
  import {
    listLauncherReleasesSince,
    type LauncherRelease,
  } from "$lib/utils/github";
  import { config } from "/src/state/config.svelte";

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
  // every release between the current version and the update
  let releases: LauncherRelease[] = [];
  let showChanges = true;
  let showDependencyChanges = false;

  // TODO - add the timestamp, tauri doesn't use an ISO timestamp!

  onMount(async () => {
    const update = await check();
    if (update) {
      releases = await listLauncherReleasesSince(update.currentVersion);
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
      // fall back to just the update's own notes if GitHub couldn't be reached
      if (releases.length === 0) {
        releases = [
          {
            version: `v${update.version}`,
            changes: availableUpdateChangelog.changes,
          },
        ];
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
    <div class="flex flex-row items-center mt-3 gap-3">
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
      <Toggle
        checked={showChanges}
        color="orange"
        onchange={(evt: any) => {
          showChanges = evt.target.checked;
        }}>{$_("update_button_viewChangelog")}</Toggle
      >
      <Toggle
        checked={showDependencyChanges}
        disabled={!showChanges}
        color="orange"
        onchange={(evt: any) => {
          showDependencyChanges = evt.target.checked;
        }}>{$_("update_button_hideDependencyChanges")}</Toggle
      >
    </div>
    {#if showChanges}
      {#each releases as release (release.version)}
        {@const changes = release.changes.filter((note) => {
          if (showDependencyChanges) {
            return true;
          } else {
            return !note.contributor.includes("dependabot");
          }
        })}
        {#if changes.length > 0}
          <!-- wrapped so the flex column can't shrink the table away -->
          <div class="mt-6">
            <div class="flex items-center gap-4">
              <h2 class="text-xl font-bold tracking-wide text-gray-200">
                {release.version}
              </h2>
              {#if release.date}
                <p
                  class="flex items-center gap-1 tracking-wide text-gray-300 text-sm"
                >
                  <IconCalendar />
                  {new Date(release.date).toLocaleString(
                    config?.locale || "en-US",
                    { month: "short", day: "numeric", year: "numeric" },
                  )}
                </p>
              {/if}
              {#if release.githubLink}
                <a
                  href={release.githubLink}
                  target="_blank"
                  rel="noreferrer"
                  class="flex items-center gap-1 cursor-pointer tracking-wide text-gray-300 hover:text-white text-sm capitalize"
                >
                  <IconGitHub />
                  {$_("settings_versions_icon_githubRelease_altText")}
                </a>
              {/if}
            </div>
            <!-- fixed layout so the columns line up across every release's table -->
            <Table
              classes={{
                div: "mt-3 pt-2 rounded-md bg-zinc-800 border border-zinc-600/70",
              }}
              class="table-fixed"
            >
              <TableHead class="bg-zinc-800! text-zinc-200!">
                <TableHeadCell class="w-48"
                  >{$_("update_changelog_header_contributor")}</TableHeadCell
                >
                <TableHeadCell
                  >{$_("update_changelog_header_description")}</TableHeadCell
                >
                <TableHeadCell class="w-40"
                  >{$_("update_changelog_header_pullRequest")}</TableHeadCell
                >
              </TableHead>
              <TableBody class="divide-y divide-zinc-700/70!">
                {#each changes as note}
                  <TableBodyRow
                    class="bg-zinc-800/80! text-white transition-colors hover:bg-zinc-700/80! border-y!"
                  >
                    <TableBodyCell class="px-6 py-3 font-bold wrap-break-word"
                      >{note.contributor}</TableBodyCell
                    >
                    <TableBodyCell
                      class="px-6 py-3 whitespace-normal wrap-break-word"
                      >{note.description}</TableBodyCell
                    >
                    <TableBodyCell class="px-6 py-3">
                      <a
                        class="inline-block"
                        href={note.pullRequestUrl}
                        target="_blank"
                        rel="noreferrer"
                      >
                        <IconGitHub class="text-xl" />
                      </a>
                    </TableBodyCell>
                  </TableBodyRow>
                {/each}
              </TableBody>
            </Table>
          </div>
        {/if}
      {/each}
    {/if}
  {:else}
    <h1 class="font-semibold text-xl text-orange-500">
      {$_("update_alreadyUpToDate")}
    </h1>
  {/if}
</div>
