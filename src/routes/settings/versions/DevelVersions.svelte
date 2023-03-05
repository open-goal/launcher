<script lang="ts">
  import { onMount } from "svelte";
  import {
    getActiveVersion,
    getActiveVersionFolder,
    listDownloadedVersions,
    openVersionFolder,
    removeVersion,
    saveActiveVersionChange,
  } from "$lib/rpc/versions";
  import VersionList from "./VersionList.svelte";
  import type { ReleaseInfo } from "$lib/utils/github";
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
    if ($VersionStore.activeVersionType === "devel") {
      $VersionStore.selectedVersions.devel = $VersionStore.activeVersionName;
    }
    // Check the backend to see if the folder has any versions
    const installedVersions = await listDownloadedVersions("devel");
    releases = [];
    for (const version of installedVersions) {
      releases = [
        ...releases,
        {
          releaseType: "devel",
          version: version,
          date: undefined,
          githubLink: undefined,
          downloadUrl: undefined,
          isDownloaded: true,
          pendingAction: false,
        },
      ];
    }
    versionsLoaded = true;
  }

  async function onSaveVersionChange(evt: any) {
    await saveActiveVersionChange(
      "devel",
      $VersionStore.selectedVersions.devel
    );
    // TODO if save was successful
    $VersionStore.activeVersionType = "devel";
    $VersionStore.activeVersionName = $VersionStore.selectedVersions.devel;
    $VersionStore.selectedVersions.official = null;
    $VersionStore.selectedVersions.unofficial = null;
  }

  async function onOpenVersionFolder(evt: any) {
    openVersionFolder("devel");
  }

  async function onRemoveVersion(event: any) {
    // Mark that release as being downloaded
    for (const release of releases) {
      if (release.version === event.detail.version) {
        release.pendingAction = true;
      }
    }
    releases = releases;
    await removeVersion(event.detail.version, "devel");
    refreshVersionList();
  }
</script>

<VersionList
  initiallyOpen={false}
  name="Development"
  description="This list serves as a convenient area to stage, manage and test new releases (either official or unofficial) This list will always require manual management via it's respective folder"
  releaseList={releases}
  loaded={versionsLoaded}
  releaseType="devel"
  on:openVersionFolder={onOpenVersionFolder}
  on:refreshVersions={refreshVersionList}
  on:versionChange={onSaveVersionChange}
  on:removeVersion={onRemoveVersion}
/>
