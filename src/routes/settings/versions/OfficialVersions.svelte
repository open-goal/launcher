<script lang="ts">
  import { onMount } from "svelte";
  import {
    downloadOfficialVersion,
    getActiveVersion,
    listDownloadedVersions,
    removeVersion,
  } from "$lib/rpc/versions";
  import { listOfficialReleases, type ReleaseInfo } from "$lib/utils/github";
  import VersionList from "./VersionList.svelte";
  import { UpdateStore } from "$lib/stores/AppStore";
  import { saveActiveVersionChange } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { toastStore } from "$lib/stores/ToastStore";
  import { versionState } from "/src/state/VersionState.svelte";

  let versionsLoaded = false;
  let releases: ReleaseInfo[] = [];

  onMount(async () => {
    await refreshVersionList();
  });

  async function refreshVersionList() {
    versionsLoaded = false;
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

    versionsLoaded = true;
  }

  async function saveOfficialVersionChange(version: string) {
    const success = await saveActiveVersionChange(version);
    if (success) {
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
    const success = await downloadOfficialVersion(version, downloadUrl);
    if (success) {
      versionState.activeToolingVersion = version;
      // Then mark it as downloaded
      releases = releases.map((release) => {
        if (release.version !== version) return release;

        const isLatest = version === releases[0]?.version;

        if ($UpdateStore.selectedTooling.updateAvailable && isLatest) {
          $UpdateStore.selectedTooling.updateAvailable = false;
        }

        return {
          ...release,
          pendingAction: false,
          isDownloaded: success,
        };
      });
      await saveActiveVersionChange(version);
    } else {
      releases = releases.map((release) =>
        release.version === version
          ? { ...release, pendingAction: false }
          : release,
      );
    }
  }

  async function onRemoveVersion(version: string) {
    // Mark that release as being removed
    releases = releases.map((release) =>
      release.version === version
        ? { ...release, pendingAction: true }
        : release,
    );
    const success = await removeVersion(version);
    if (success) {
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
    } else {
      releases = releases.map((release) =>
        release.version === version
          ? { ...release, pendingAction: false }
          : release,
      );
    }
  }
</script>

<VersionList
  description={$_("settings_versions_official_description")}
  releaseList={releases}
  loaded={versionsLoaded}
  onVersionChange={saveOfficialVersionChange}
  {onRemoveVersion}
  onRefreshVersions={refreshVersionList}
  {onDownloadVersion}
/>
