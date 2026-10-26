<script lang="ts">
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import IconGitHub from "~icons/mdi/github";
  import { _ } from "svelte-i18n";
  import { isDependencyChange, type ChangelogEntry } from "$lib/utils/github";

  let {
    changes,
    showDependencyChanges,
  }: { changes: ChangelogEntry[]; showDependencyChanges: boolean } = $props();
</script>

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
    <TableHeadCell>{$_("update_changelog_header_description")}</TableHeadCell>
    <TableHeadCell class="w-40"
      >{$_("update_changelog_header_pullRequest")}</TableHeadCell
    >
  </TableHead>
  <TableBody class="divide-y divide-zinc-700/70!">
    {#each changes.filter((note) => showDependencyChanges || !isDependencyChange(note)) as note}
      <TableBodyRow
        class="bg-zinc-800/80! text-white transition-colors hover:bg-zinc-700/80! border-y!"
      >
        <TableBodyCell class="px-6 py-3 font-bold wrap-break-word"
          >{note.contributor}</TableBodyCell
        >
        <TableBodyCell class="px-6 py-3 whitespace-normal wrap-break-word"
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
