<script lang="ts">
  import {
    getInstallationDirectory,
  } from "$lib/rpc/config";
  import { getInternalName, fromRoute, SupportedGame } from "$lib/constants";
  import { useParams } from "svelte-navigator";
  import {
    downloadUnofficialVersion,
    getActiveVersion,
    getActiveVersionFolder,
    getModDict,
    listUnofficialDownloadedVersions,
    openUnofficialVersionFolder,
    removeVersion,
  } from "$lib/rpc/versions";
  import { listReleases, type ReleaseInfo } from "$lib/utils/github";
  import Icon from "@iconify/svelte";
  import { 
    Button, 
    Label,
    Spinner,
    Select,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell, } from "flowbite-svelte";
  import ModVersionList from "./ModVersionList.svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";

  export let game_name;
  export let mod_id;
  const params = useParams();

  let modDict = {};
  let modList = [];
  let selectedMod = null;
  
  let versionsLoaded = false;
  let releases: ReleaseInfo[] = [];

  onMount(async () => {
    console.log("loading with selected id:", mod_id);
    modDict = await getModDict(game_name);
    for (let id in modDict) {
      modList = [
        ...modList,
        {
          value: id,
          name: modDict[id]["name"]
        },
      ];
    }
    
    if (mod_id != null && mod_id != undefined && mod_id != "" && modDict.hasOwnProperty(mod_id)) {
      selectedMod = modDict[mod_id];
    }
    console.log("loading with selected:", selectedMod);
    refreshVersionList();
  });

  async function setSelectedMod(value: string) {
    mod_id = value;
    if (modDict.hasOwnProperty(mod_id)) {
      selectedMod = modDict[mod_id];
      console.log("switching to:", selectedMod);
      location.href = `/${game_name}/mods/${mod_id}`;
    } else {
      // error?
      mod_id = "";
      selectedMod = {};
    }
  }

  async function refreshVersionList() {
    versionsLoaded = false;
    // Reset store to defaults (TODO, move this to a store method)
    // $VersionStore.activeVersionType = await getActiveVersionFolder();
    // $VersionStore.activeVersionName = await getActiveVersion();
    // if ($VersionStore.activeVersionType === "official") {
    //   $VersionStore.selectedVersions.official = $VersionStore.activeVersionName;
    // }
    // Check the backend to see if the folder has any versions
    const installedVersions = await listUnofficialDownloadedVersions(`${mod_id}`);
    releases = [];
    for (const version of installedVersions) {
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
    // TODO - "no releases found"

    // Merge that with the actual current releases on github
    const githubReleases = await listReleases("unofficial", selectedMod["repo"]);
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
          releaseType: "unofficial",
          version: `${release.version}`,
          date: release.date,
          githubLink: release.githubLink,
          downloadUrl: release.downloadUrl,
          isDownloaded: false,
          pendingAction: false,
        },
      ];
    }

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

    versionsLoaded = true;
  }

  async function onDownloadVersion(event: any) {
    // Mark that release as being downloaded
    for (const release of releases) {
      if (release.version === event.detail.version) {
        release.pendingAction = true;
      }
    }
    releases = releases;
    const success = await downloadUnofficialVersion(
      event.detail.version,
      mod_id,
      event.detail.downloadUrl
    );
    // Then mark it as downloaded
    for (const release of releases) {
      if (release.version === event.detail.version) {
        release.pendingAction = false;
        release.isDownloaded = success;
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
    const ok = await removeVersion(event.detail.version, `unofficial/${mod_id}`);
    if (ok) {
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
    await onRemoveVersion(event);
    await onDownloadVersion(event);
  }

</script>

<div class="flex flex-col h-full p-5">
  <div class="flex flex-col gap-5">
    <div class="flex flex-row" style="justify-content: space-between;">
      <Button
        btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
        on:click={() => history.back()}
      >
        <Icon
          icon="ic:baseline-arrow-back"
          color="#ffffff"
          width="20"
          height="20"
        />
      </Button>
      <div class="flex gap-3">
      <Button
        btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
        on:click={() => location.reload()}
      >
        <Icon
          icon="ic:baseline-refresh"
          color="#ffffff"
          width="20"
          height="20"
        />
      </Button>
      <Button
        btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
        href="/{game_name}/mod_lists"
      >
        Manage Mod Lists
      </Button>
      </div>
    </div>
    <Label>Select Mod
      <Select
        class="mt-2"
        value={mod_id}
        items={modList}
        on:change={async (evt) => {
          setSelectedMod(evt.target.value);
        }}
      />
    </Label>

    {#if mod_id != null && mod_id != undefined && mod_id != ""}
      <ModVersionList
        initiallyOpen={true}
        game_name={game_name}
        mod_id={mod_id}
        releaseList={releases}
        loaded={versionsLoaded}
        releaseType="unofficial"
        on:openVersionFolder={() => openUnofficialVersionFolder(`${mod_id}`)}
        on:refreshVersions={refreshVersionList}
        on:removeVersion={onRemoveVersion}
        on:downloadVersion={onDownloadVersion}
        on:redownloadVersion={onRedownloadVersion}
      />
    {/if}
  </div>
</div>