<script lang="ts">
  import {
    Button,
    Table,
    TableBody,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import type { LayoutProps } from "./$types";
  import { _ } from "svelte-i18n";
  import IconFolderOpen from "~icons/mdi/folder-open";
  import { revealItemInDir } from "@tauri-apps/plugin-opener";

  let { data, children }: LayoutProps = $props();
  const config = $derived(data.config);
</script>

<p class="mt-2 mb-2">{$_("settings_versions_header")}</p>

<div class="flex items-center mb-2">
  <div class="grow">
    <p class="text-gray-300">
      {$_("settings_versions_official_description")}
    </p>
  </div>
  <Button
    class="!p-2 rounded-md bg-orange-500 hover:bg-orange-600 text-slate-900"
    onclick={async () =>
      revealItemInDir(config.installationDir + "/versions/official/")}
  >
    <IconFolderOpen
      aria-label={$_("settings_versions_icon_openFolder_altText")}
    />
  </Button>
</div>

<div class="flex flex-col max-h-96 overflow-y-scroll">
  <Table>
    <TableHead class="bg-zinc-800">
      <TableHeadCell></TableHeadCell>
      <TableHeadCell></TableHeadCell>
      <TableHeadCell
        >{$_("settings_versions_table_header_version")}</TableHeadCell
      >
      <TableHeadCell>{$_("settings_versions_table_header_date")}</TableHeadCell>
      <TableHeadCell
        >{$_("settings_versions_table_header_changes")}</TableHeadCell
      >
    </TableHead>
    <TableBody class="text-gray-300">
      {@render children()}
    </TableBody>
  </Table>
</div>
