<script lang="ts">
  import { Button, TabItem, Radio, Spinner } from "flowbite-svelte";
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import {
    downloadOfficialVersion,
    getActiveVersion,
    listDownloadedVersions,
    openVersionFolder,
    saveActiveVersionChange,
  } from "$lib/rpc/versions";
  import {
    listOfficialReleases,
    type OfficialRelease,
  } from "$lib/utils/github";

  let componentLoaded = false;
  let currentOfficialVersion = undefined;
  let selectedOfficialVersion = undefined;

  const tabItemActiveClasses =
    "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  const tabItemInactiveClasses =
    "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";

  let officialReleases: OfficialRelease[] = [];

  onMount(async () => {
    // TODO - check when this is null
    currentOfficialVersion = await getActiveVersion();
    selectedOfficialVersion = currentOfficialVersion;
    await refreshOfficialVersionList(undefined);
    componentLoaded = true;
  });

  async function refreshOfficialVersionList(evt) {
    // Check the backend to see if the folder has any versions
    const installedVersions = await listDownloadedVersions(
      "official"
    );
    officialReleases = [];
    for (const version of installedVersions) {
      officialReleases = [
        ...officialReleases,
        {
          version: version,
          date: undefined,
          githubLink: undefined,
          downloadUrl: undefined,
          isDownloaded: true,
          isBeingDownloaded: false,
        },
      ];
    }
    // TODO - confirmation on deletion / tauri command to handle that
    // TODO - "no releases found"

    // Merge that with the actual current releases on github
    const githubReleases = await listOfficialReleases();
    for (const release of githubReleases) {
      // Look to see if we already have this release downloaded and we just have to fill in some metadata about it
      let foundExistingRelease = false;
      for (const existingRelease of officialReleases) {
        if (existingRelease.version === release.version) {
          existingRelease.date = release.date;
          existingRelease.githubLink = release.githubLink;
          existingRelease.downloadUrl =
            "https://github.com/open-goal/jak-project/releases/download/v0.1.32/opengoal-windows-v0.1.32.zip";
          foundExistingRelease = true;
          break;
        }
      }
      if (foundExistingRelease) {
        continue;
      }
      officialReleases = [
        ...officialReleases,
        {
          version: release.version,
          date: release.date,
          githubLink: release.githubLink,
          downloadUrl:
            "https://github.com/open-goal/jak-project/releases/download/v0.1.32/opengoal-windows-v0.1.32.zip",
          isDownloaded: false,
          isBeingDownloaded: false,
        },
      ];
    }

    // Sort releases by published date
    officialReleases = officialReleases.sort((a, b) =>
      b.date.localeCompare(a.date)
    );
    selectedOfficialVersion = "v0.1.32";
  }

  async function saveOfficialVersionChange(evt) {
    await saveActiveVersionChange(
      "official",
      selectedOfficialVersion
    );
    currentOfficialVersion = selectedOfficialVersion;
  }

  async function openOfficialVersionFolder(evt) {
    openVersionFolder("official");
  }

  async function onDownloadOfficialVersion(version: String, url: String) {
    // Mark that release as being downloaded
    for (const release of officialReleases) {
      if (release.version === version) {
        release.isBeingDownloaded = true;
      }
    }
    officialReleases = officialReleases;
    await downloadOfficialVersion(version, url);
    // TODO - indicate success or failure
    // Then mark it as downloaded
    for (const release of officialReleases) {
      if (release.version === version) {
        release.isBeingDownloaded = false;
        release.isDownloaded = true;
      }
    }
    officialReleases = officialReleases;
  }
</script>

<!-- TODO - clean up duplication here with some components for each page -->

<TabItem
  open
  activeClasses={tabItemActiveClasses}
  inactiveClasses={tabItemInactiveClasses}
>
  <span slot="title">Official</span>
  {#if !componentLoaded}
    <div class="flex flex-col justify-center items-center">
      <Spinner color="yellow" size={"12"} />
    </div>
  {:else}
    <div class="flex items-center mb-2">
      <div class="grow">
        <p class="text-sm text-gray-400 dark:text-gray-300">
          Official versions are from the `jak-project` GitHub repository
        </p>
      </div>
      <div class="flex">
        {#if currentOfficialVersion != selectedOfficialVersion}
          <Button
            btnClass="!p-2 mr-2 rounded-md dark:bg-green-500 hover:dark:bg-green-600 text-slate-900"
            on:click={saveOfficialVersionChange}
          >
            <Icon
              icon="material-symbols:save"
              width="20"
              height="20"
              alt="save official version change"
            />
          </Button>
        {/if}
        <Button
          btnClass="!p-2 mr-2 rounded-md dark:bg-orange-500 hover:dark:bg-orange-600 text-slate-900"
          on:click={refreshOfficialVersionList}
        >
          <Icon
            icon="material-symbols:refresh"
            width="20"
            height="20"
            alt="refresh official version list"
          />
        </Button>
        <Button
          btnClass="!p-2 rounded-md dark:bg-orange-500 hover:dark:bg-orange-600  text-slate-900"
          on:click={openOfficialVersionFolder}
        >
          <Icon
            icon="material-symbols:folder-open-rounded"
            width="20"
            height="20"
            alt="open official version folder"
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
        {#each officialReleases as release (release.version)}
          <TableBodyRow>
            <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium">
              {#if release.isDownloaded}
                <Radio
                  class="disabled:cursor-not-allowed p-0"
                  bind:group={selectedOfficialVersion}
                  value={release.version}
                  disabled={!release.isDownloaded}
                  name="official-release"
                />
              {/if}
            </TableBodyCell>
            <TableBodyCell
              tdClass="px-6 py-2 whitespace-nowrap font-medium"
              style="line-height: 0;"
            >
              <Button
                btnClass="dark:bg-transparent hover:dark:bg-transparent focus:ring-0 focus:ring-offset-0"
                on:click={() =>
                  onDownloadOfficialVersion(
                    release.version,
                    release.downloadUrl
                  )}
              >
                {#if release.isDownloaded}
                  <Icon
                    icon="ic:baseline-delete-forever"
                    width="24"
                    height="24"
                    color="red"
                  />
                {:else if release.isBeingDownloaded}
                  <Spinner color="yellow" size={"6"} />
                {:else}
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
            <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium"
              >{new Date(release.date).toLocaleDateString()}</TableBodyCell
            >
            <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium"
              ><a
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
            </TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  {/if}
</TabItem>
