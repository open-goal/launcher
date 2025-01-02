<script lang="ts">
  import { onMount } from "svelte";
  import {
    downloadOfficialVersion,
    getActiveVersion,
    getActiveVersionFolder,
    listDownloadedVersions,
    openVersionFolder,
    removeVersion,
  } from "$lib/rpc/versions";
  import { listOfficialReleases, type ReleaseInfo } from "$lib/utils/github";
  import VersionList from "./VersionList.svelte";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { UpdateStore } from "$lib/stores/AppStore";
  import { saveActiveVersionChanges } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { toastStore } from "$lib/stores/ToastStore";

  let versionsLoaded = false;
  let releases: ReleaseInfo[] = [];

  onMount(async () => {
    await refreshVersionList();
  });

  async function refreshVersionList() {
    versionsLoaded = false;
    // Reset store to defaults (TODO, move this to a store method)
    $VersionStore.activeVersionType = await getActiveVersionFolder();
    $VersionStore.activeVersionName = await getActiveVersion();
    if ($VersionStore.activeVersionType === "official") {
      $VersionStore.selectedVersions.official = $VersionStore.activeVersionName;
    }
    // Check the backend to see if the folder has any versions
    const installedVersions = await listDownloadedVersions("official");
    releases = [];
    for (const version of installedVersions) {
      releases = [
        ...releases,
        {
          releaseType: "official",
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
          releaseType: "official",
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
      $UpdateStore.selectedTooling.updateAvailable = !releases[0].isDownloaded;
    }
    versionsLoaded = true;
  }

  async function saveOfficialVersionChange() {
    const success = await saveActiveVersionChanges(
      "official",
      $VersionStore.selectedVersions.official,
    );
    if (success) {
      $VersionStore.activeVersionType = "official";
      $VersionStore.activeVersionName = $VersionStore.selectedVersions.official;
      toastStore.makeToast($_("toasts_savedToolingVersion"), "info");
    }
  }

  async function openOfficialVersionFolder() {
    openVersionFolder("official");
  }

  async function onDownloadVersion(event: any) {
    // Mark that release as being downloaded
    for (const release of releases) {
      if (release.version === event.detail.version) {
        release.pendingAction = true;
      }
    }
    releases = releases;
    const success = await downloadOfficialVersion(
      event.detail.version,
      event.detail.downloadUrl,
    );
    // Then mark it as downloaded
    for (const release of releases) {
      if (release.version === event.detail.version) {
        release.pendingAction = false;
        release.isDownloaded = success;
        // If they downloaded the latest, get rid of the notification
        if ($UpdateStore.selectedTooling.updateAvailable) {
          if (event.detail.version === releases[0].version) {
            $UpdateStore.selectedTooling.updateAvailable = false;
          }
        }
      }
    }
    releases = releases;
    $VersionStore.selectedVersions.official = event.detail.version;
    await saveOfficialVersionChange();
  }

  async function onRemoveVersion(event: any) {
    // Mark that release as being downloaded
    for (const release of releases) {
      if (release.version === event.detail.version) {
        release.pendingAction = true;
      }
    }
    releases = releases;
    const ok = await removeVersion(event.detail.version, "official");
    if (ok) {
      // Update the store, if we removed the active version
      if (
        $VersionStore.activeVersionName === event.detail.version &&
        $VersionStore.activeVersionType === "official"
      ) {
        $VersionStore.activeVersionName = null;
        $VersionStore.activeVersionType = null;
        $VersionStore.selectedVersions.official = null;
      }

      // Then mark it as not downloaded
      for (const release of releases) {
        if (release.version === event.detail.version) {
          release.pendingAction = false;
          release.isDownloaded = false;
        }
      }
      releases = releases;
    }
  }

  async function onRedownloadVersion(event: any) {
    // If we are redownloading the version that is currently selected (but not active, get rid of the selection)
    if ($VersionStore.activeVersionType === "official") {
      $VersionStore.selectedVersions.official = $VersionStore.activeVersionName;
    }
    await onRemoveVersion(event);
    await onDownloadVersion(event);
  }
</script>

<VersionList
  description={$_("settings_versions_official_description")}
  releaseList={releases}
  loaded={versionsLoaded}
  releaseType="official"
  on:openVersionFolder={openOfficialVersionFolder}
  on:refreshVersions={refreshVersionList}
  on:versionChange={saveOfficialVersionChange}
  on:removeVersion={onRemoveVersion}
  on:downloadVersion={onDownloadVersion}
  on:redownloadVersion={onRedownloadVersion}
/>
