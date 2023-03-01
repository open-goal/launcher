<script lang="ts">
  import { getGameTitle, getInternalName, SupportedGame } from "$lib/constants";
  import { openDir } from "$lib/rpc/commands";
  import Icon from "@iconify/svelte";
  import { configDir, join } from "@tauri-apps/api/path";
  import { createEventDispatcher, onMount } from "svelte";
  import { confirm } from "@tauri-apps/api/dialog";
  import {
    Button,
    Dropdown,
    DropdownItem,
    DropdownDivider,
    Helper,
  } from "flowbite-svelte";
  import {
    launchGame,
    openREPL,
    resetGameSettings,
    uninstallGame,
  } from "$lib/rpc/game";
  import { platform } from "@tauri-apps/api/os";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();
  let settingsDir = undefined;
  let savesDir = undefined;
  let isLinux = false;

  onMount(async () => {
    isLinux = (await platform()) === "linux";
    settingsDir = await join(
      await configDir(),
      "OpenGOAL",
      getInternalName(activeGame),
      "settings"
    );
    savesDir = await join(
      await configDir(),
      "OpenGOAL",
      getInternalName(activeGame),
      "saves"
    );
  });
</script>

<div class="flex flex-col justify-end items-end mt-auto">
  <!-- TOOO - time played -->
  <h1
    class="tracking-tighter text-2xl font-bold pb-3 text-orange-500 text-outline pointer-events-none"
  >
    {getGameTitle(activeGame)}
  </h1>
  <div class="flex flex-row gap-2">
    <Button
      btnClass="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      on:click={async () => {
        launchGame(getInternalName(activeGame), false);
      }}>Play</Button
    >
    <!-- TODO - texture replacements left out for now, get everything else working end-to-end first -->
    <!-- <Button
      btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-5 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      ><Chevron placement="top">Features</Chevron></Button
    >
    <Dropdown placement="top-end">
      <DropdownItem>Texture&nbsp;Replacements</DropdownItem>
    </Dropdown> -->
    <Button
      btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
    >
      Advanced
    </Button>
    <Dropdown placement="top-end" frameClass="!bg-slate-900">
      <DropdownItem
        on:click={async () => {
          launchGame(getInternalName(activeGame), true);
        }}>Play&nbsp;in&nbsp;Debug&nbsp;Mode</DropdownItem
      >
      {#if !isLinux}
        <DropdownItem
          on:click={async () => {
            openREPL(getInternalName(activeGame));
          }}>Open REPL</DropdownItem
        >
      {/if}
      <DropdownDivider />
      <DropdownItem
        on:click={async () => {
          dispatch("job", {
            type: "decompile",
          });
        }}
        >Decompile
        <!-- NOTE - this is a bug in flowbite-svelte, it's not replacing the default class but just appending -->
        <Helper helperClass="!text-neutral-400 !text-xs"
          >Extracts game assets (ie. to apply texture replacements)</Helper
        ></DropdownItem
      >
      <DropdownItem
        on:click={async () => {
          dispatch("job", {
            type: "compile",
          });
        }}
        >Compile
        <!-- NOTE - this is a bug in flowbite-svelte, it's not replacing the default class but just appending -->
        <Helper helperClass="!text-neutral-400 !text-xs"
          >Rebuild the game. (ie. after modifying OpenGOAL source code)
        </Helper></DropdownItem
      >
    </Dropdown>
    <Button
      btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
    >
      <Icon icon="material-symbols:settings" width={24} height={24} />
    </Button>
    <Dropdown placement="top-end" frameClass="!bg-slate-900">
      <!-- TODO - screenshot folder? how do we even configure where those go? -->
      <DropdownItem
        on:click={async () => {
          await openDir(settingsDir);
        }}>Open&nbsp;Settings&nbsp;Folder</DropdownItem
      >
      <DropdownItem
        on:click={async () => {
          await openDir(savesDir);
        }}>Open&nbsp;Saves&nbsp;Folder</DropdownItem
      >
      <DropdownDivider />
      <!-- TODO - verify installation -->
      <!-- <DropdownItem>Verify&nbsp;Install</DropdownItem> -->
      <DropdownItem
        on:click={async () => {
          await resetGameSettings(getInternalName(activeGame));
        }}>Reset Settings</DropdownItem
      >
      <DropdownItem
        on:click={async () => {
          // Get confirmation
          // TODO - probably move these confirms into the actual launcher itself
          const confirmed = await confirm(
            "Are you sure you want to uninstall?",
            { title: "OpenGOAL Launcher", type: "warning" }
          );
          if (confirmed) {
            await uninstallGame(getInternalName(activeGame));
            dispatch("change");
          }
        }}
        >Uninstall<Helper helperClass="!text-neutral-400 !text-xs"
          >This will not delete any saves or settings</Helper
        ></DropdownItem
      >
    </Dropdown>
  </div>
</div>
