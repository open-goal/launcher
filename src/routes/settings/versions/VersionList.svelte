<script lang="ts">
  import type { VersionFolders } from "$lib/rpc/versions";
  import {
    VersionStore,
    type VersionStoreIFace,
  } from "$lib/stores/VersionStore";
  import type { ReleaseInfo } from "$lib/utils/github";
  import Icon from "@iconify/svelte";
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
  } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";

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
            btnClass="!p-2 mr-2 rounded-md dark:bg-green-500 hover:dark:bg-green-600 text-slate-900"
            on:click={() => dispatch("versionChange")}
          >
            <Icon
              icon="material-symbols:save"
              width="20"
              height="20"
              alt="save version change"
            />
          </Button>
        {/if}
        <Button
          btnClass="!p-2 mr-2 rounded-md dark:bg-orange-500 hover:dark:bg-orange-600 text-slate-900"
          on:click={() => dispatch("refreshVersions")}
        >
          <Icon
            icon="material-symbols:refresh"
            width="20"
            height="20"
            alt="refresh version list"
          />
        </Button>
        <Button
          btnClass="!p-2 rounded-md dark:bg-orange-500 hover:dark:bg-orange-600  text-slate-900"
          on:click={() => dispatch("openVersionFolder")}
        >
          <Icon
            icon="material-symbols:folder-open-rounded"
            width="20"
            height="20"
            alt="open version folder"
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
        <TableHeadCell>Version</TableHeadCell>
        <TableHeadCell>Date</TableHeadCell>
        <TableHeadCell>Changes</TableHeadCell>
      </TableHead>
      <TableBody tableBodyClass="divide-y">
        {#each releaseList as release (release.version)}
          <TableBodyRow>
            <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium">
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
              tdClass="px-6 py-2 whitespace-nowrap font-medium"
              style="line-height: 0;"
            >
              <Button
                btnClass="dark:bg-transparent hover:dark:bg-transparent focus:ring-0 focus:ring-offset-0"
                disabled={release.pendingAction}
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
                  <Icon
                    icon="ic:baseline-delete-forever"
                    width="24"
                    height="24"
                    color="red"
                  />
                {:else if release.pendingAction}
                  <Spinner color="yellow" size={"6"} />
                {:else if release.releaseType === "official"}
                  <Icon
                    icon="ic:baseline-download"
                    color="#00d500"
                    width="24"
                    height="24"
                  />
                {/if}
              </Button>
            </TableBodyCell>
            <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium"
              >{release.version}</TableBodyCell
            >
            <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium">
              {#if release.date}
                {new Date(release.date).toLocaleDateString()}
              {/if}
            </TableBodyCell>
            <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium">
              {#if release.githubLink}
                <a
                  class="inline-block"
                  href={release.githubLink}
                  target="_blank"
                  rel="noreferrer"
                  ><Icon
                    class="inline"
                    icon="mdi:github"
                    width="24"
                    height="24"
                  /></a
                >
              {/if}
            </TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  {/if}
</TabItem>
