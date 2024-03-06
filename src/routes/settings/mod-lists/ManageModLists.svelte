<script lang="ts">
  import { type VersionFolders } from "$lib/rpc/versions";
  import {
    VersionStore,
    type VersionStoreIFace,
  } from "$lib/stores/VersionStore";
  import { Label, Input, Checkbox, Modal } from "flowbite-svelte";
  import {
    addModList,
    removeModList,
    getModLists,
    type ModList,
  } from "$lib/rpc/mods";
  // import Icon from "@iconify/svelte";
  import {
    Button,
    Radio,
    Spinner,
    TabItem,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { _ } from "svelte-i18n";

  export let game_name: String;
  export let releaseType: VersionFolders;
  let modLists: ModList[] = [];
  let addModListModal;
  let newModListUrl;
  let newModListId;

  const tabItemActiveClasses =
    "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  const tabItemInactiveClasses =
    "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";

  const dispatch = createEventDispatcher();

  function changesPending(versionStore: VersionStoreIFace): boolean {
    return (
      releaseType != null &&
      versionStore.selectedVersions[releaseType] !== null &&
      versionStore.selectedVersions[releaseType] !== "" &&
      versionStore.selectedVersions[releaseType] !==
        versionStore.activeVersionName
    );
  }

  async function handleAddModListSave() {
    try {
      const resp = await fetch(newModListUrl);
      const newModJson = await resp.json();
      console.log("downloaded json from ", newModListUrl);
      console.log(newModJson);
      await addModList(newModListUrl, newModListId, JSON.stringify(newModJson));
      addModListModal = false;
      newModListId = null;
      newModListUrl = null;
      await refreshModLists();
      return true;
    } catch (e) {
      console.log("ERROR: ", e);
    }

    return false;
  }

  async function handleRemoveModList(modListId) {
    await removeModList(modListId);
    refreshModLists();
  }

  async function refreshModLists() {
    modLists.length = 0;
    modLists = [...modLists, ...(await getModLists())];
  }

  onMount(async () => {
    refreshModLists();
  });
</script>

<Modal bind:open={addModListModal} size="xs">
  <form
    class="flex flex-col space-y-6"
    action="about:blank"
    on:submit={(event) => {
      event.preventDefault();
      handleAddModListSave();
    }}
  >
    <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">
      Add Mod List
    </h3>
    <Label class="space-y-2">
      <span>URL:</span>
      <Input
        type="url"
        name="url"
        bind:value={newModListUrl}
        placeholder="all-things-andy-gavin.com/og_mod_list.json"
        required
      />
    </Label>
    <Label class="space-y-2">
      <span>Give this list a unique ID:</span>
      <Input
        name="id"
        bind:value={newModListId}
        placeholder="andys_mod_list"
        required
      />
    </Label>
    <Button
      btnClass="!p-2 mr-2 rounded-md dark:bg-green-500 hover:dark:bg-green-600 text-slate-900"
      type="submit">Save</Button
    >
  </form>
</Modal>
<div class="flex flex-col p-5">
  <div class="flex flex-col gap-5">
    <div class="flex flex-row" style="justify-content: space-between;">
      <Button
        btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
        on:click={() => history.back()}
      >
        icon!
        <!-- <Icon
          icon="ic:baseline-arrow-back"
          color="#ffffff"
          width="20"
          height="20"
        /> -->
      </Button>
      <div class="flex gap-3">
        <Button
          btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
          on:click={() => (addModListModal = true)}
        >
          Add Mod List
        </Button>
      </div>
    </div>
    <Label
      >Installed Mod Lists
      {#if modLists.length == 0}
        <div class="flex flex-row gap-2 p-2 justify-center">
          icon!
          <!-- <Icon
                icon="material-symbols:warning"
                width="25"
                height="25"
                color="yellow"
              /> -->
          You haven't added any mod lists yet!
        </div>
      {:else}
        <div class="flex flex-row gap-2 p-2 justify-center">
          <Table style="table-layout: fixed;">
            <TableHead>
              <TableHeadCell style="width: 30%">ID</TableHeadCell>
              <TableHeadCell style="width: 55%">URL</TableHeadCell>
              <TableHeadCell style="width: 15%">Remove</TableHeadCell>
            </TableHead>
            <TableBody tableBodyClass="divide-y">
              {#each modLists as modList (modList.identifier)}
                <TableBodyRow>
                  <TableBodyCell
                    tdClass="px-6 py-2 whitespace-nowrap font-medium"
                    >{modList.identifier}</TableBodyCell
                  >
                  <TableBodyCell
                    tdClass="px-6 py-2 whitespace-nowrap font-medium"
                    style="width: 55%; overflow: hidden; text-overflow: ellipsis"
                  >
                    <a href={modList.url} target="_blank" rel="noreferrer">
                      {modList.url}
                    </a>
                  </TableBodyCell>
                  <TableBodyCell
                    tdClass="px-6 py-2 whitespace-nowrap font-medium text-center"
                    style="line-height: 0;"
                  >
                    <Button
                      btnClass="dark:bg-transparent hover:dark:bg-transparent focus:ring-0 focus:ring-offset-0 disabled:opacity-50"
                      on:click={async (event) => {
                        if (event["pointerId"] >= 0) {
                          handleRemoveModList(modList.identifier);
                        }
                      }}
                    >
                      <Icon
                        icon="ic:baseline-delete-forever"
                        width="24"
                        height="24"
                        color="red"
                        alt={$_("settings_versions_icon_removeVersion_altText")}
                      />
                    </Button>
                  </TableBodyCell>
                </TableBodyRow>
              {/each}
            </TableBody>
          </Table>
        </div>
      {/if}
    </Label>
  </div>
</div>
