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
          invalid: false,
          invalidationReasons: [],
        },
      ];
    }
    versionsLoaded = true;
  }

  async function onSaveVersionChange(evt: any) {
    const success = await saveActiveVersionChange(
      "devel",
      $VersionStore.selectedVersions.devel,
    );
    if (success) {
      $VersionStore.activeVersionType = "devel";
      $VersionStore.activeVersionName = $VersionStore.selectedVersions.devel;
      $VersionStore.selectedVersions.official = null;
      $VersionStore.selectedVersions.unofficial = null;
      toastStore.makeToast("Saved game version!", "info");
    }
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
  name={$_("settings_versions_devel_tabName")}
  description={$_("settings_versions_devel_description")}
  releaseList={releases}
  loaded={versionsLoaded}
  releaseType="devel"
  on:openVersionFolder={onOpenVersionFolder}
  on:refreshVersions={refreshVersionList}
  on:versionChange={onSaveVersionChange}
  on:removeVersion={onRemoveVersion}
/>
