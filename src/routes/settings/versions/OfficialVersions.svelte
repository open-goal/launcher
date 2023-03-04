<script lang="ts">
  import { onMount } from "svelte";
  import {
    downloadOfficialVersion,
    getActiveVersion,
    getActiveVersionFolder,
    listDownloadedVersions,
    openVersionFolder,
    removeVersion,
    saveActiveVersionChange,
  } from "$lib/rpc/versions";
  import { listOfficialReleases, type ReleaseInfo } from "$lib/utils/github";
  import VersionList from "./VersionList.svelte";
  import { VersionStore } from "$lib/stores/VersionStore";

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
        },
      ];
    }
    // TODO - "no releases found"

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
        },
      ];
    }

    // Sort releases by published date
    releases = releases.sort((a, b) =>
      b.date.localeCompare(a.date)
    );
    versionsLoaded = true;
  }

  async function saveOfficialVersionChange(evt) {
    await saveActiveVersionChange("official", $VersionStore.selectedVersions.official);
    // TODO if save was successful
    $VersionStore.activeVersionType = "official";
    $VersionStore.activeVersionName = $VersionStore.selectedVersions.official;
    $VersionStore.selectedVersions.unofficial = undefined;
    $VersionStore.selectedVersions.devel = undefined;
  }

  async function openOfficialVersionFolder(evt) {
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
    await downloadOfficialVersion(event.detail.version, event.detail.downloadUrl);
    // TODO - indicate success or failure (toast)
    // Then mark it as downloaded
    for (const release of releases) {
      if (release.version === event.detail.version) {
        release.pendingAction = false;
        release.isDownloaded = true;
      }
    }
    releases = releases;
  }

  async function onRemoveVersion(event: any) {
    // Mark that release as being downloaded
    for (const release of releases) {
      if (release.version === event.detail.version) {
        release.pendingAction = true;
      }
    }
    releases = releases;
    await removeVersion(event.detail.version, "official");
    // TODO - indicate success or failure (toast)
    // Then mark it as downloaded
    for (const release of releases) {
      if (release.version === event.detail.version) {
        release.pendingAction = false;
        release.isDownloaded = false;
      }
    }
    // TODO - handle removing the active version / selected version
    releases = releases;
  }
</script>

<VersionList
  initiallyOpen={true}
  name="Official"
  description="Official versions are from the `jak-project` GitHub repository"
  releaseList={releases}
  loaded={versionsLoaded}
  releaseType="official"
  on:openVersionFolder={openOfficialVersionFolder}
  on:refreshVersions={refreshVersionList}
  on:versionChange={saveOfficialVersionChange}
  on:removeVersion={onRemoveVersion}
  on:downloadVersion={onDownloadVersion}
/>
