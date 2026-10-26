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

<Table hoverable={true} class="mt-3">
  <TableHead>
    <TableHeadCell>{$_("update_changelog_header_contributor")}</TableHeadCell>
    <TableHeadCell>{$_("update_changelog_header_description")}</TableHeadCell>
    <TableHeadCell>{$_("update_changelog_header_pullRequest")}</TableHeadCell>
  </TableHead>
  <TableBody class="divide-y">
    {#each changes.filter((note) => showDependencyChanges || !isDependencyChange(note)) as note}
      <TableBodyRow>
        <TableBodyCell class="px-6 py-2 whitespace-nowrap font-bold"
          >{note.contributor}</TableBodyCell
        >
        <TableBodyCell
          class="px-6 py-2 max-w-80 whitespace-normal wrap-break-word"
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
