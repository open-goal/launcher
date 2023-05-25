<script lang="ts">
  import {
    getInstallationDirectory,
  } from "$lib/rpc/config";
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
  import { createEventDispatcher } from "svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";

  const dispatch = createEventDispatcher();

  let currentInstallationDirectory = "";
  let currentModList;
  let availableModLists = [
    {
      value: "ogmll",
      name: "OG Mod Launcher list",
    },
    {
      value: "gelato",
      name: "Andy Gavin's list",
    }
  ];
  let modList = [
    {
      id: "orbhunt",
      name: "Orb Hunt",
      downloadUrl: "tbd",
      isDownloaded: true,
      updateAvailable: false,
      pendingAction: false
    },
    {
      id: "rando",
      name: "Checkpoint Randomizer",
      downloadUrl: "tbd",
      isDownloaded: true,
      updateAvailable: true,
      pendingAction: false
    },
    {
      id: "ngminus",
      name: "NG-",
      downloadUrl: "tbd",
      isDownloaded: false,
      updateAvailable: false,
      pendingAction: true
    },
    {
      id: "flutflut",
      name: "FlutFlut Legacy",
      downloadUrl: "tbd",
      isDownloaded: false,
      updateAvailable: false,
      pendingAction: false
    }
  ];

  onMount(async () => {
    // currentInstallationDirectory = await getInstallationDirectory();
    
    currentModList = "ogmll";
  });
</script>

<div class="flex flex-col h-full p-5">
  <div class="flex flex-row gap-2">
    <Button
      btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      on:click={() => history.back()}
    >
      <Icon
        icon="ic:baseline-arrow-back"
        color="#ffffff"
        width="24"
        height="24"
      />
    </Button>
  </div>
  <div class="flex flex-col gap-5 mt-2">
    <div>
      <Label>Selected Mod List
      <div class="flex">
        <Select
          class="mt-2" style="display:inline"
          items={availableModLists}
          bind:value={currentModList}
          on:change={async (evt) => {
            // setLocale(evt.target.value);
          }}
        />
        <Button
          btnClass="!p-2 rounded-md text-slate-900 mt-2"
          on:click={async () => {
            // go to new page to add/edit/remove mod lists
          }}>
          <Icon
            icon="ic:baseline-settings"
            width=40
            height=40
            color="gray"
          />
        </Button>
      </div>
      </Label>
    </div>
    <Label>Available Mods
      <Table class="mt-2">
        <TableHead>
          <TableHeadCell>Mod Name</TableHeadCell>
          <TableHeadCell style="text-align: center;">Manage</TableHeadCell>
          <TableHeadCell style="text-align: center;">Launch</TableHeadCell>
          <TableHeadCell style="text-align: center;">More Info</TableHeadCell>
        </TableHead>
        <TableBody tableBodyClass="divide-y">
          {#each modList as mod (mod.id)}
            <TableBodyRow>
              <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium">
                {mod.name}
              </TableBodyCell>
              <TableBodyCell
                tdClass="px-6 py-2 whitespace-nowrap font-medium"
                style="line-height: 0; text-align: center;"
              >
                {#if mod.isDownloaded}
                  <Button
                    btnClass="dark:bg-transparent hover:dark:bg-transparent focus:ring-0 focus:ring-offset-0 disabled:opacity-50"
                    disabled={mod.pendingAction}
                    on:click={async () => {
                      // if (mod.isDownloaded) {
                      //   dispatch("removeVersion", { version: mod.version });
                      // }
                    }}
                  >
                    <Icon
                      icon="ic:baseline-delete-forever"
                      width="24"
                      height="24"
                      color="red"
                    />
                  </Button>
                    {#if mod.updateAvailable}
                      <Button
                        btnClass="dark:bg-transparent hover:dark:bg-transparent focus:ring-0 focus:ring-offset-0 disabled:opacity-50"
                        disabled={mod.pendingAction}
                        on:click={async () => {
                          // if (mod.updateAvailable) {
                          //   dispatch("downloadVersion", {
                          //     version: mod.version,
                          //     downloadUrl: mod.downloadUrl,
                          //   });
                          // }
                        }}
                      >
                        <Icon
                          icon="ic:baseline-download"
                          width="24"
                          height="24"
                          color="#dd8800"
                        />
                      </Button>
                    {/if}
                  {:else if mod.pendingAction}
                    <Spinner color="yellow" size={"6"} />
                  {:else if mod.downloadUrl !== undefined}
                    <Button
                      btnClass="dark:bg-transparent hover:dark:bg-transparent focus:ring-0 focus:ring-offset-0 disabled:opacity-50"
                      disabled={mod.pendingAction}
                      on:click={async () => {
                        // if (!mod.isDownloaded) {
                        //   dispatch("downloadVersion", {
                        //     version: mod.version,
                        //     downloadUrl: mod.downloadUrl,
                        //   });
                        // }
                      }}
                    >
                      <Icon
                        icon="ic:baseline-download"
                        color="#00d500"
                        width="24"
                        height="24"
                      />
                    </Button>
                  {/if}
              </TableBodyCell>
              <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium" style="text-align: center;">
                <Button
                  btnClass="dark:bg-transparent hover:dark:bg-transparent focus:ring-0 focus:ring-offset-0 disabled:opacity-50"
                  disabled={mod.pendingAction}
                  on:click={async () => {
                    // if (mod.isDownloaded) {
                    //   dispatch("removeVersion", { version: mod.version });
                    // } else {
                    //   dispatch("downloadVersion", {
                    //     version: mod.version,
                    //     downloadUrl: mod.downloadUrl,
                    //   });
                    // }
                  }}
                >
                {#if mod.isDownloaded}
                <Icon
                  icon="ic:baseline-play-circle"
                  width="24"
                  height="24"
                  color="green"
                />
                {/if}
                </Button>
              </TableBodyCell>
              <TableBodyCell tdClass="px-6 py-2 whitespace-nowrap font-medium" style="text-align: center;">
                <a
                  class="inline-block"
                  href="./mods/{mod.id}"
                  ><Icon
                    class="inline"
                    icon="ic:baseline-more-horiz"
                    width="24"
                    height="24"
                  /></a
                >
              </TableBodyCell>
            </TableBodyRow>
          {/each}
        </TableBody>
      </Table>
    </Label>
  </div>
</div>