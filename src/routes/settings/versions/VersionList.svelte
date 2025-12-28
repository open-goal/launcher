<script lang="ts">
  import type { ReleaseInfo } from "$lib/utils/github";
  import IconRefresh from "~icons/mdi/refresh";
  import IconFolderOpen from "~icons/mdi/folder-open";
  import IconGitHub from "~icons/mdi/github";
  import IconDownload from "~icons/mdi/download";
  import IconDeleteForever from "~icons/mdi/delete-forever";
  import {
    Alert,
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
  import { revealItemInDir } from "@tauri-apps/plugin-opener";
  import { getInstallationDirectory } from "$lib/rpc/config";
  import { versionState } from "/src/state/VersionState.svelte";

  let {
    description,
    releaseList,
    loaded,
    onVersionChange,
    onRemoveVersion,
    onRefreshVersions,
    onDownloadVersion,
  }: {
    description: string;
    releaseList: ReleaseInfo[];
    loaded: boolean;
    onVersionChange: (version: string) => Promise<void>;
    onRemoveVersion: (version: string) => Promise<void>;
    onRefreshVersions: () => Promise<void>;
    onDownloadVersion: (version: string, downloadUrl: string) => Promise<void>;
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

{#if !loaded}
  <div class="flex flex-col justify-center items-center">
    <Spinner color="yellow" size="12" />
  </div>
{:else}
  <div class="flex items-center mb-2">
    <div class="grow">
      <p class="text-sm text-gray-400 text-gray-300">
        {description}
      </p>
    </div>
    <div class="flex">
      <Button
        class="!p-2 mr-2 rounded-md bg-orange-500 hover:bg-orange-600 text-slate-900"
        onclick={() => onRefreshVersions()}
      >
        <IconRefresh
          aria-label={$_("settings_versions_icon_refresh_altText")}
        />
      </Button>
      <Button
        class="!p-2 rounded-md bg-orange-500 hover:bg-orange-600 text-slate-900"
        onclick={async () =>
          revealItemInDir(
            (await getInstallationDirectory()) + "/versions/official/",
          )}
      >
        <IconFolderOpen
          aria-label={$_("settings_versions_icon_openFolder_altText")}
        />
      </Button>
    </div>
  </div>

  {#if releaseList.length === 0}
    <Alert class="bg-slate-900 flex-grow text-red-400">
      {$_("settings_versions_noReleasesFound")}
    </Alert>
  {:else}
    <Table>
      <TableHead class="bg-slate-400">
        <TableHeadCell></TableHeadCell>
        <TableHeadCell></TableHeadCell>
        <TableHeadCell
          >{$_("settings_versions_table_header_version")}</TableHeadCell
        >
        <TableHeadCell
          >{$_("settings_versions_table_header_date")}</TableHeadCell
        >
        <TableHeadCell
          >{$_("settings_versions_table_header_changes")}</TableHeadCell
        >
      </TableHead>
      <TableBody class="divide-y *:text-white">
        {#each releaseList as release (release.version)}
          <TableBodyRow class="bg-slate-700">
            <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
              {#if release.isDownloaded}
                <Radio
                  bind:group={versionState.activeToolingVersion}
                  value={release.version}
                  onchange={() => {
                    onVersionChange(release.version);
                  }}
                  disabled={!release.isDownloaded}
                  color="orange"
                  class="disabled:cursor-not-allowed p-0"
                />
              {/if}
            </TableBodyCell>
            <TableBodyCell
              class="px-6 py-2 whitespace-nowrap font-medium"
              style="line-height: 0;"
            >
              <Button
                class="py-0 bg-transparent focus:ring-0 disabled:opacity-50"
                disabled={release.pendingAction}
                onclick={async () => handleAction(release)}
              >
                {#if release.isDownloaded}
                  <IconDeleteForever
                    class="text-xl"
                    color="red"
                    aria-label={$_(
                      "settings_versions_icon_removeVersion_altText",
                    )}
                  />
                {:else if release.pendingAction}
                  <Spinner color="yellow" size="6" />
                {:else}
                  <IconDownload
                    class="text-xl"
                    color="#00d500"
                    aria-label={$_(
                      "settings_versions_icon_downloadVersion_altText",
                    )}
                  />
                {/if}
              </Button>
              {#if release.isDownloaded}
                <Button
                  class="py-0 bg-transparent focus:ring-0 disabled:opacity-50"
                  disabled={release.pendingAction}
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
            </TableBodyCell>
            <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
              {release.version}
            </TableBodyCell>
            <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
              {#if release.date}
                {new Date(release.date).toLocaleDateString()}
              {/if}
            </TableBodyCell>
            <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
              {#if release.githubLink}
                <a href={release.githubLink} target="_blank" rel="noreferrer">
                  <IconGitHub
                    class="text-xl"
                    aria-label={$_(
                      "settings_versions_icon_githubRelease_altText",
                    )}
                  />
                </a>
              {/if}
            </TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  {/if}
{/if}
