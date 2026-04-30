<script lang="ts">
  import { onMount } from "svelte";
  import {
    downloadOfficialVersion,
    getActiveVersion,
    listDownloadedVersions,
    removeVersion,
  } from "$lib/rpc/versions";
  import { listOfficialReleases, type ReleaseInfo } from "$lib/utils/github";
  import VersionList from "./versions/VersionList.svelte";
  import { UpdateStore } from "$lib/stores/AppStore";
  import {
    getInstallationDirectory,
    saveActiveVersionChange,
  } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { toastStore } from "$lib/stores/ToastStore";
  import { versionState } from "/src/state/VersionState.svelte";
  import { Alert, Button, Spinner } from "flowbite-svelte";
  import Latest from "./versions/Latest.svelte";
  import { openPath } from "@tauri-apps/plugin-opener";
  import IconFolderOpen from "~icons/mdi/folder-open";
  import IconRefresh from "~icons/mdi/refresh";

  let loading = $state(true);
  let releases: ReleaseInfo[] = $state([]);

  onMount(async () => {
    await refreshVersionList();
    loading = false;
  });

  async function refreshVersionList() {
    loading = true;
    versionState.activeToolingVersion = await getActiveVersion();
    // Check the backend to see if the folder has any versions
    const installedVersions = await listDownloadedVersions();
    releases = [];
    for (const version of installedVersions) {
      releases = [
        ...releases,
        {
          version: version,
          date: undefined,
          githubLink: undefined,
          downloadUrl: undefined,
          isDownloaded: true,
          pendingAction: false,
          invalid: false,
          invalidationReasons: [],
        },
      ];
    }

    // Merge that with the actual current releases on github
    const githubReleases = await listOfficialReleases();
    for (const release of githubReleases) {
      // Look to see if we already have this release downloaded and we just have to fill in some metadata about it
      let foundExistingRelease = false;
      for (const existingRelease of releases) {
        if (existingRelease.version === release.version) {
          existingRelease.date = release.date;
          existingRelease.githubLink = release.githubLink;
          existingRelease.downloadUrl = release.downloadUrl;
          foundExistingRelease = true;
          break;
        }
      }
      if (foundExistingRelease) {
        continue;
      }
      releases = [
        ...releases,
        {
          version: release.version,
          date: release.date,
          githubLink: release.githubLink,
          downloadUrl: release.downloadUrl,
          isDownloaded: false,
          pendingAction: false,
          invalid: release.invalid,
          invalidationReasons: release.invalidationReasons,
        },
      ];
    }

    // filter incompatible releases
    releases = releases.filter((r) => r.downloadUrl !== undefined);
    releases = releases.filter((r) => !r.invalid);

    // if no releases are found, early out
    if (releases.length > 0) {
      // Sort releases by published date
      releases = releases.sort((a, b) => {
        if (a.date === undefined) {
          return 1;
        }
        if (b.date === undefined) {
          return -1;
        }
        return b.date.localeCompare(a.date);
      });

      // If we find the latest when refreshing, get rid of the notification
      if ($UpdateStore.selectedTooling.updateAvailable) {
        $UpdateStore.selectedTooling.updateAvailable =
          !releases[0].isDownloaded;
      }
    }

    loading = false;
  }

  async function saveOfficialVersionChange(version: string) {
    const error = await saveActiveVersionChange(version);
    if (!error) {
      toastStore.makeToast($_("toasts_savedToolingVersion"), "info");
    }
  }

  async function onDownloadVersion(version: string, downloadUrl: string) {
    // Mark that release as being downloaded
    releases = releases.map((release) =>
      release.version === version
        ? { ...release, pendingAction: true }
        : release,
    );
    const error = await downloadOfficialVersion(version, downloadUrl);
    if (error) {
      // ensure the version folder is removed if it failed
      await onRemoveVersion(version);
      releases = releases.map((release) =>
        release.version === version
          ? { ...release, pendingAction: false }
          : release,
      );
      return;
    }

    versionState.activeToolingVersion = version;
    const isLatest = version === releases[0]?.version;

    if ($UpdateStore.selectedTooling.updateAvailable && isLatest) {
      $UpdateStore.selectedTooling.updateAvailable = false;
    }

    releases = releases.map((release) =>
      release.version === version
        ? { ...release, pendingAction: false, isDownloaded: true }
        : release,
    );
    await saveOfficialVersionChange(version);
  }

  async function onRemoveVersion(version: string) {
    // Mark that release as being removed
    releases = releases.map((release) =>
      release.version === version
        ? { ...release, pendingAction: true }
        : release,
    );
    const error = await removeVersion(version);
    if (error) {
      releases = releases.map((release) =>
        release.version === version
          ? { ...release, pendingAction: false }
          : release,
      );
      return;
    }

    // Update the store, if we removed the active version
    if (versionState.activeToolingVersion === version) {
      versionState.activeToolingVersion = undefined;
    }

    // Then mark it as not downloaded
    releases = releases.map((release) =>
      release.version === version
        ? { ...release, pendingAction: false, isDownloaded: false }
        : release,
    );
  }

  const isPending = $derived(releases.some((r) => r.pendingAction));
</script>

<p class="text-sm mt-2 mb-2">{$_("settings_versions_header")}</p>
{#if loading}
  <div class="flex flex-col justify-center items-center">
    <Spinner color="yellow" size="12" />
  </div>
{:else if releases.length > 0}
  <div class="flex items-center mb-2">
    <div class="grow">
      <p class="text-sm text-gray-300">
        {$_("settings_versions_official_description")}
      </p>
    </div>
    <div class="flex">
      <Button
        class="p-2! mr-2 rounded-md bg-orange-500 hover:bg-orange-600 text-slate-900"
        onclick={refreshVersionList}
        disabled={isPending}
      >
        <IconRefresh
          aria-label={$_("settings_versions_icon_refresh_altText")}
        />
      </Button>
      <Button
        class="p-2! rounded-md bg-orange-500 hover:bg-orange-600 text-slate-900"
        onclick={async () =>
          openPath((await getInstallationDirectory()) + "/versions/official/")}
      >
        <IconFolderOpen
          aria-label={$_("settings_versions_icon_openFolder_altText")}
        />
      </Button>
    </div>
  </div>

  <Latest latest={releases[0]} {onRemoveVersion} {onDownloadVersion} {isPending}
  ></Latest>

  <VersionList
    releaseList={releases.slice(1)}
    onVersionChange={saveOfficialVersionChange}
    {onRemoveVersion}
    {onDownloadVersion}
    {isPending}
  />
{:else}
  <Alert class="bg-slate-900 grow text-red-400">
    {$_("settings_versions_noReleasesFound")}
  </Alert>
{/if}
