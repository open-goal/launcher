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
    listUnofficialDownloadedVersions,
    openUnofficialVersionFolder,
    removeVersion,
  } from "$lib/rpc/versions";
  import { type ReleaseInfo } from "$lib/utils/github";
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
  import { getModDict } from "$lib/utils/mods";

  export let game_name;
  export let mod_composite_id;
  const params = useParams();

  let modDict = {};
  let modList = [];
  let selectedMod = null;
  
  let versionsLoaded = false;
  let releases: ReleaseInfo[] = [];

  async function refreshModListsAndDict() {
    modDict = await getModDict(game_name);
    // refresh modList for dropdown
    modList.length = 0;
    for (let composite_id in modDict) {
      let list_id = composite_id.split("$$", 1)[0];
      let displayName = modDict[composite_id].name + " (" + list_id+  ")";

      modList = [
        ...modList,
        {
          value: composite_id,
          name: displayName
        }
      ];
    }
    console.log("reloaded with modlist: ", modList);
  }

  onMount(async () => {
    console.log("loading with selected id:", mod_composite_id);
    await refreshModListsAndDict();
    
    if (mod_composite_id != null && mod_composite_id != undefined && mod_composite_id != "" && modDict.hasOwnProperty(mod_composite_id)) {
      selectedMod = modDict[mod_composite_id];
    }
    console.log("loading with selected mod:", selectedMod);
    refreshVersionList();
  });

  async function setSelectedMod(value: string) {
    mod_composite_id = value;
    if (modDict.hasOwnProperty(mod_composite_id)) {
      selectedMod = modDict[mod_composite_id];
      console.log("switching to:", selectedMod);
      location.href = `/${game_name}/mods/${mod_composite_id}`;
    } else {
      // error?
      mod_composite_id = "";
      selectedMod = {};
    }
  }

  async function refreshVersionList() {
    versionsLoaded = false;
    releases.length = 0;

    if (selectedMod != null && selectedMod != undefined) {
      // Check the backend to see if the folder has any versions
      const installedVersions = await listUnofficialDownloadedVersions(`${mod_composite_id}`);
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

      // merge known versions from mod list with installed versions
      for (const v of selectedMod.versions) {
        console.log(selectedMod, v);
        if (v.games.indexOf(game_name) == -1) {
          // current game not supported
          continue;
        }

        let foundExistingRelease = false;
        for (const existingRelease of releases) {
          // found it! update some metadata
          if (existingRelease.version === v.version) {
            // existingRelease.date = v.date;
            // existingRelease.githubLink = v.githubLink;
            existingRelease.downloadUrl = v.windows_bundle_url; // TODO linux
            foundExistingRelease = true;
            break;
          }
        }

        // not installed, add to list
        if (!foundExistingRelease) {
          releases = [
            ...releases,
            {
              releaseType: "unofficial",
              version: v.version,
              date: undefined,
              githubLink: undefined,
              downloadUrl: v.windows_bundle_url, // TODO linux
              isDownloaded: false,
              pendingAction: false,
            },
          ];
        }
      }

      // Sort releases by published date
      // releases = releases.sort((a, b) => {
      //   if (a.date === undefined) {
      //     return 1;
      //   }
      //   if (b.date === undefined) {
      //     return -1;
      //   }
      //   return b.date.localeCompare(a.date);
      // });

      versionsLoaded = true;
    }
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
      mod_composite_id,
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
    const ok = await removeVersion(event.detail.version, `unofficial/${mod_composite_id}`);
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
        value={mod_composite_id}
        items={modList}
        on:change={async (evt) => {
          setSelectedMod(evt.target.value);
        }}
      />
    </Label>

    {#if mod_composite_id != null && mod_composite_id != undefined && mod_composite_id != ""}
      <ModVersionList
        initiallyOpen={true}
        game_name={game_name}
        mod_id={mod_composite_id}
        releaseList={releases}
        loaded={versionsLoaded}
        releaseType="unofficial"
        on:openVersionFolder={() => openUnofficialVersionFolder(`${mod_composite_id}`)}
        on:refreshVersions={refreshVersionList}
        on:removeVersion={onRemoveVersion}
        on:downloadVersion={onDownloadVersion}
        on:redownloadVersion={onRedownloadVersion}
      />
    {/if}
  </div>
</div>