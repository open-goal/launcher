<script lang="ts">
  import type { ReleaseInfo } from "$lib/utils/github";
  import IconRefresh from "~icons/mdi/refresh";
  import IconGitHub from "~icons/mdi/github";
  import IconDownload from "~icons/mdi/download";
  import IconDeleteForever from "~icons/mdi/delete-forever";
  import {
    Button,
    Radio,
    Spinner,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import { versionState } from "/src/state/VersionState.svelte";
  import { onMount } from "svelte";
  import { getLocale } from "$lib/rpc/config";

  let locale = $state("en-US");

  onMount(async () => {
    const res = await getLocale();
    if (res) locale = res;
  });

  let {
    releaseList,
    onVersionChange,
    onRemoveVersion,
    onDownloadVersion,
    isPending,
  }: {
    releaseList: ReleaseInfo[];
    onVersionChange: (version: string) => Promise<void>;
    onRemoveVersion: (version: string) => Promise<void>;
    onDownloadVersion: (version: string, downloadUrl: string) => Promise<void>;
    isPending: boolean;
  } = $props();

  const handleAction = async (release: ReleaseInfo) => {
    if (release.isDownloaded) {
      await onRemoveVersion(release.version);
    } else if (release.downloadUrl) {
      await onDownloadVersion(release.version, release.downloadUrl);
    }
  };

  const handleRedownload = async (release: ReleaseInfo) => {
    if (release.downloadUrl) {
      await onRemoveVersion(release.version);
      await onDownloadVersion(release.version, release.downloadUrl);
    }
  };
</script>

<Table divClass="pt-2 rounded-md bg-zinc-800 border border-zinc-600/70">
  <TableHead class="bg-zinc-800! text-zinc-200!">
    <TableHeadCell class="w-12"></TableHeadCell>
    <TableHeadCell class="w-28"></TableHeadCell>
    <TableHeadCell>{$_("settings_versions_table_header_version")}</TableHeadCell
    >
    <TableHeadCell>{$_("settings_versions_table_header_date")}</TableHeadCell>
    <TableHeadCell>{$_("settings_versions_table_header_changes")}</TableHeadCell
    >
  </TableHead>
  <TableBody class="divide-y divide-zinc-700/70!">
    {#each releaseList as release (release.version)}
      <TableBodyRow
        class="bg-zinc-800/80! text-white transition-colors hover:bg-zinc-700/80! border!"
      >
        <TableBodyCell class="px-6 py-3">
          <Radio
            bind:group={versionState.activeToolingVersion}
            value={release.version}
            onchange={() => {
              onVersionChange(release.version);
            }}
            disabled={!release.isDownloaded}
            color="green"
            class="p-0 disabled:cursor-not-allowed"
          />
        </TableBodyCell>

        <TableBodyCell class="px-6 py-2">
          <div class="flex items-center gap-4">
            <Button
              class="p-0 bg-transparent focus:ring-0 disabled:opacity-50"
              disabled={isPending}
              onclick={async () => handleAction(release)}
            >
              {#if release.isDownloaded}
                <IconDeleteForever
                  class="text-xl text-red-500"
                  aria-label={$_(
                    "settings_versions_icon_removeVersion_altText",
                  )}
                />
              {:else if release.pendingAction}
                <Spinner color="yellow" size="6" />
              {:else}
                <IconDownload
                  class="text-xl text-green-500"
                  aria-label={$_(
                    "settings_versions_icon_downloadVersion_altText",
                  )}
                />
              {/if}
            </Button>

            {#if release.isDownloaded}
              <Button
                class="py-0 bg-transparent focus:ring-0 disabled:opacity-50"
                disabled={isPending}
                onclick={() => handleRedownload(release)}
              >
                {#if release.pendingAction}
                  <Spinner color="yellow" size="6" />
                {:else}
                  <IconRefresh
                    class="text-xl"
                    aria-label={$_(
                      "settings_versions_icon_redownloadVersion_altText",
                    )}
                  />
                {/if}
              </Button>
            {/if}
          </div>
        </TableBodyCell>

        <TableBodyCell class="px-6 py-3 whitespace-nowrap font-bold">
          {release.version}
        </TableBodyCell>
        <TableBodyCell class="px-6 py-3 whitespace-nowrap font-medium">
          {#if release.date}
            {new Date(release.date).toLocaleString(locale, {
              month: "short",
              day: "numeric",
              year: "numeric",
            })}
          {/if}
        </TableBodyCell>

        <TableBodyCell class="px-6 py-3">
          {#if release.githubLink}
            <a href={release.githubLink} target="_blank" rel="noreferrer">
              <IconGitHub
                class="text-xl"
                aria-label={$_("settings_versions_icon_githubRelease_altText")}
              />
            </a>
          {/if}
        </TableBodyCell>
      </TableBodyRow>
    {/each}
  </TableBody>
</Table>
