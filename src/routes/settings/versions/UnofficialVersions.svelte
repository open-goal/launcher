<script lang="ts">
  import { onMount } from "svelte";
  import {
    getActiveVersion,
    getActiveVersionFolder,
    listDownloadedVersions,
    openVersionFolder,
    removeVersion,
  } from "$lib/rpc/versions";
  import VersionList from "./VersionList.svelte";
  import type { ReleaseInfo } from "$lib/utils/github";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { saveActiveVersionChange } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";

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
    if ($VersionStore.activeVersionType === "unofficial") {
      $VersionStore.selectedVersions.unofficial =
        $VersionStore.activeVersionName;
    }
    // Check the backend to see if the folder has any versions
    const installedVersions = await listDownloadedVersions("unofficial");
    releases = [];
    for (const version of installedVersions) {
      // TODO - mods have no standardized metadata (i think?), can't do much here!
      releases = [
        ...releases,
        {
          releaseType: "unofficial",
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
    const success = await saveActiveVersionChange(
      "unofficial",
      $VersionStore.selectedVersions.unofficial
    );
    if (success) {
      $VersionStore.activeVersionType = "unofficial";
      $VersionStore.activeVersionName =
        $VersionStore.selectedVersions.unofficial;
      $VersionStore.selectedVersions.official = null;
      $VersionStore.selectedVersions.devel = null;
    }
  }

  async function onOpenVersionFolder(evt: any) {
    openVersionFolder("unofficial");
  }

  async function onRemoveVersion(event: any) {
    // Mark that release as being downloaded
    for (const release of releases) {
      if (release.version === event.detail.version) {
        release.pendingAction = true;
      }
    }
    releases = releases;
    await removeVersion(event.detail.version, "unofficial");
    refreshVersionList();
  }
</script>

<VersionList
  initiallyOpen={false}
  name={$_("settings_versions_unofficial_tabName")}
  description={$_("settings_versions_unofficial_description")}
  releaseList={releases}
  loaded={versionsLoaded}
  releaseType="unofficial"
  on:openVersionFolder={onOpenVersionFolder}
  on:refreshVersions={refreshVersionList}
  on:versionChange={onSaveVersionChange}
  on:removeVersion={onRemoveVersion}
/>
