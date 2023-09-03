<script lang="ts">
  import type { VersionFolders } from "$lib/rpc/versions";
  import {
    VersionStore,
    type VersionStoreIFace,
  } from "$lib/stores/VersionStore";
  import type { ReleaseInfo } from "$lib/utils/github";
  import IconSave from "~icons/mdi/content-save";
  import IconRefresh from "~icons/mdi/refresh";
  import IconFolderOpen from "~icons/mdi/folder-open";
  import IconGitHub from "~icons/mdi/github";
  import IconDownload from "~icons/mdi/download";
  import IconDeleteForever from "~icons/mdi/delete-forever";
  import {
    Button,
    Radio,
    Spinner,
    TabItem,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Tooltip,
  } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";
  import { _ } from "svelte-i18n";

  export let name: string;
  export let description: string;
  export let releaseList: ReleaseInfo[];
  export let loaded: boolean;
  export let releaseType: VersionFolders;
  export let initiallyOpen: boolean;

  const tabItemActiveClasses =
    "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  const tabItemInactiveClasses =
    "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";

  const dispatch = createEventDispatcher();

  function changesPending(versionStore: VersionStoreIFace): boolean {
    return (
      versionStore.selectedVersions[releaseType] !== null &&
      versionStore.selectedVersions[releaseType] !== "" &&
      versionStore.selectedVersions[releaseType] !==
        versionStore.activeVersionName
    );
  }
</script>

<TabItem
  open={initiallyOpen}
  activeClasses={tabItemActiveClasses}
  inactiveClasses={tabItemInactiveClasses}
>
  <span slot="title">{name}</span>
  {#if !loaded}
    <div class="flex flex-col justify-center items-center">
      <Spinner color="yellow" size={"12"} />
    </div>
  {:else}
    <div class="flex items-center mb-2">
      <div class="grow">
        <p class="text-sm text-gray-400 dark:text-gray-300">
          {description}
        </p>
      </div>
      <div class="flex">
        {#if changesPending($VersionStore)}
          <Button
            class="!p-2 mr-2 rounded-md dark:bg-green-500 hover:dark:bg-green-600 text-slate-900"
            on:click={() => dispatch("versionChange")}
          >
            <IconSave aria-label={$_("settings_versions_icon_save_altText")} />
          </Button>
        {/if}
        <Button
          class="!p-2 mr-2 rounded-md dark:bg-orange-500 hover:dark:bg-orange-600 text-slate-900"
          on:click={() => dispatch("refreshVersions")}
        >
          <IconRefresh
            aria-label={$_("settings_versions_icon_refresh_altText")}
          />
        </Button>
        <Button
          class="!p-2 rounded-md dark:bg-orange-500 hover:dark:bg-orange-600  text-slate-900"
          on:click={() => dispatch("openVersionFolder")}
        >
          <IconFolderOpen
            aria-label={$_("settings_versions_icon_openFolder_altText")}
          />
        </Button>
      </div>
    </div>
    <Table>
      <TableHead>
        <TableHeadCell>
          <span class="sr-only">Select</span>
        </TableHeadCell>
        <TableHeadCell>
          <span class="sr-only">Controls</span>
        </TableHeadCell>
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
      <TableBody tableBodyClass="divide-y">
        {#each releaseList as release (release.version)}
          <TableBodyRow>
            <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
              {#if release.isDownloaded}
                <Radio
                  class="disabled:cursor-not-allowed p-0"
                  bind:group={$VersionStore.selectedVersions[releaseType]}
                  value={release.version}
                  disabled={!release.isDownloaded}
                  name={`${releaseType}-release`}
                />
              {/if}
            </TableBodyCell>
            <TableBodyCell
              class="px-6 py-2 whitespace-nowrap font-medium"
              style="line-height: 0;"
            >
              <Button
                class="py-0 dark:bg-transparent hover:dark:bg-transparent focus:ring-0 focus:ring-offset-0 disabled:opacity-50"
                disabled={release.pendingAction ||
                  (!release.isDownloaded &&
                    release.downloadUrl !== undefined &&
                    release.invalid)}
                on:click={async () => {
                  if (release.isDownloaded) {
                    dispatch("removeVersion", { version: release.version });
                  } else {
                    dispatch("downloadVersion", {
                      version: release.version,
                      downloadUrl: release.downloadUrl,
                    });
                  }
                }}
              >
                {#if release.isDownloaded}
                  <IconDeleteForever
                    class="text-xl"
                    color="red"
                    aria-label={$_(
                      "settings_versions_icon_removeVersion_altText",
                    )}
                  />
                {:else if release.downloadUrl === undefined}
                  <span>Incompatible</span>
                {:else if release.pendingAction}
                  <Spinner color="yellow" size={"6"} />
                {:else if release.releaseType === "official" && release.downloadUrl !== undefined}
                  <IconDownload
                    class="text-xl"
                    color="#00d500"
                    aria-label={$_(
                      "settings_versions_icon_downloadVersion_altText",
                    )}
                  />
                {/if}
              </Button>
              {#if release.invalid}
                <Tooltip color="red">
                  {#if release.invalidationReasons.length > 0}
                    Release marked as invalid for the following reasons:
                    {#each release.invalidationReasons as reason}
                      <br />
                      - {reason}
                    {/each}
                  {:else}
                    Release marked as invalid
                  {/if}
                </Tooltip>
              {/if}
              {#if release.isDownloaded && release.releaseType == "official"}
                <Button
                  class="py-0 dark:bg-transparent hover:dark:bg-transparent focus:ring-0 focus:ring-offset-0 disabled:opacity-50"
                  disabled={release.pendingAction}
                  on:click={async () => {
                    dispatch("redownloadVersion", {
                      version: release.version,
                      downloadUrl: release.downloadUrl,
                    });
                  }}
                >
                  {#if release.pendingAction}
                    <Spinner color="yellow" size={"6"} />
                  {:else}
                    <IconRefresh
                      class="text-xl"
                      aria-label="Redownload Version"
                    />
                  {/if}
                </Button>
              {/if}
            </TableBodyCell>
            <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium"
              >{release.version}</TableBodyCell
            >
            <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
              {#if release.date}
                {new Date(release.date).toLocaleDateString()}
              {/if}
            </TableBodyCell>
            <TableBodyCell class="px-6 py-2 whitespace-nowrap font-medium">
              {#if release.githubLink}
                <a
                  class="inline-block"
                  href={release.githubLink}
                  target="_blank"
                  rel="noreferrer"
                >
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
</TabItem>
