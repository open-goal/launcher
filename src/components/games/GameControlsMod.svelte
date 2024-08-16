<script lang="ts">
  import { getInternalName, SupportedGame } from "$lib/constants";
  import { openDir } from "$lib/rpc/window";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import IconCog from "~icons/mdi/cog";
  import { join } from "@tauri-apps/api/path";
  import { createEventDispatcher, onMount } from "svelte";
  import { writeText } from "@tauri-apps/api/clipboard";
  import { confirm } from "@tauri-apps/api/dialog";
  import {
    Button,
    Dropdown,
    DropdownItem,
    DropdownDivider,
    Helper,
  } from "flowbite-svelte";
  import { platform } from "@tauri-apps/api/os";
  import { getInstallationDirectory } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { navigate } from "svelte-navigator";
  import { toastStore } from "$lib/stores/ToastStore";
  import {
    getLaunchModString,
    launchMod,
    openREPLForMod,
    resetModSettings,
    uninstallMod,
  } from "$lib/rpc/features";
  import { pathExists } from "$lib/rpc/util";

  export let activeGame: SupportedGame;
  export let modName: string = "";
  export let modDisplayName: string = "";
  export let modSource: string = "";

  const dispatch = createEventDispatcher();
  let gameDataDir: string | undefined = undefined;
  let settingsDir: string | undefined = undefined;
  let savesDir: string | undefined = undefined;
  let isLinux = false;

  onMount(async () => {
    isLinux = (await platform()) === "linux";
    let installationDir = await getInstallationDirectory();
    if (installationDir !== null) {
      gameDataDir = await join(
        installationDir,
        "features",
        getInternalName(activeGame),
        "mods",
        modSource,
        modName,
        "data",
      );
      settingsDir = await join(
        installationDir,
        "features",
        getInternalName(activeGame),
        "mods",
        modSource,
        "_settings",
        modName,
        "OpenGOAL",
        getInternalName(activeGame),
        "settings",
      );
      if (!(await pathExists(settingsDir))) {
        settingsDir = undefined;
      }
      savesDir = await join(
        installationDir,
        "features",
        getInternalName(activeGame),
        "mods",
        modSource,
        "_settings",
        modName,
        "OpenGOAL",
        getInternalName(activeGame),
        "saves",
      );
      if (!(await pathExists(savesDir))) {
        savesDir = undefined;
      }
    }
  });
</script>

<div class="flex flex-col justify-end items-end mt-auto">
  <h1
    class="tracking-tighter text-2xl font-bold pb-3 text-orange-500 text-outline pointer-events-none"
  >
    {modDisplayName}
  </h1>
  <div class="flex flex-row gap-2">
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      on:click={async () => {
        navigate(`/${getInternalName(activeGame)}/features/mods`, {
          replace: true,
        });
      }}><IconArrowLeft />&nbsp;Back</Button
    >
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      on:click={async () => {
        launchMod(getInternalName(activeGame), false, modName, modSource);
      }}>{$_("gameControls_button_play")}</Button
    >
    <Button
      class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
    >
      {$_("gameControls_button_advanced")}
    </Button>
    <Dropdown placement="top-end" class="!bg-slate-900">
      <DropdownItem
        on:click={async () => {
          launchMod(getInternalName(activeGame), true, modName, modSource);
        }}>{$_("gameControls_button_playInDebug")}</DropdownItem
      >
      {#if !isLinux}
        <DropdownItem
          on:click={async () => {
            openREPLForMod(getInternalName(activeGame), modName, modSource);
          }}>{$_("gameControls_button_openREPL")}</DropdownItem
        >
      {/if}
      <DropdownDivider />
      <DropdownItem
        on:click={async () => {
          dispatch("job", {
            type: "decompileMod",
          });
        }}
        >{$_("gameControls_button_decompile")}
        <!-- NOTE - this is a bug in flowbite-svelte, it's not replacing the default class but just appending -->
        <Helper helperClass="!text-neutral-400 !text-xs"
          >{$_("gameControls_button_decompile_helpText")}</Helper
        ></DropdownItem
      >
      <DropdownItem
        on:click={async () => {
          dispatch("job", {
            type: "compileMod",
          });
        }}
        >{$_("gameControls_button_compile")}
        <!-- NOTE - this is a bug in flowbite-svelte, it's not replacing the default class but just appending -->
        <Helper helperClass="!text-neutral-400 !text-xs"
          >{$_("gameControls_button_compile_helpText")}
        </Helper></DropdownItem
      >
      <DropdownDivider />
      <DropdownItem
        on:click={async () => {
          if (gameDataDir) {
            await openDir(gameDataDir);
          }
        }}>{$_("gameControls_button_openGameFolder")}</DropdownItem
      >
    </Dropdown>
    <Button
      class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
    >
      <IconCog />
    </Button>
    <Dropdown placement="top-end" class="!bg-slate-900">
      <!-- TODO - screenshot folder? how do we even configure where those go? -->
      {#if settingsDir}
        <DropdownItem
          on:click={async () => {
            if (settingsDir) {
              await openDir(settingsDir);
            }
          }}>{$_("gameControls_button_openSettingsFolder")}</DropdownItem
        >
      {/if}
      {#if savesDir}
        <DropdownItem
          on:click={async () => {
            if (savesDir) {
              await openDir(savesDir);
            }
          }}>{$_("gameControls_button_openSavesFolder")}</DropdownItem
        >
      {/if}
      {#if settingsDir || savesDir}
        <DropdownDivider />
      {/if}
      <DropdownItem
        on:click={async () => {
          const launchString = await getLaunchModString(
            getInternalName(activeGame),
            modName,
            modSource,
          );
          await writeText(launchString);
          toastStore.makeToast("Copied to clipboard!", "info");
        }}
        >{$_("gameControls_button_copyExecutableCommand")}<Helper
          helperClass="!text-neutral-400 !text-xs"
          >{$_("gameControls_button_copyExecutableCommand_helpText_1")}<br
          />{$_("gameControls_button_copyExecutableCommand_helpText_2")}</Helper
        ></DropdownItem
      >
      <DropdownDivider />
      <DropdownItem
        on:click={async () => {
          await resetModSettings(
            getInternalName(activeGame),
            modName,
            modSource,
          );
        }}>{$_("gameControls_button_resetSettings")}</DropdownItem
      >
      <DropdownItem
        on:click={async () => {
          // Get confirmation
          // TODO - probably move these confirms into the actual launcher itself
          const confirmed = await confirm(
            $_("gameControls_button_uninstall_confirmation"),
            { title: "OpenGOAL Launcher", type: "warning" },
          );
          if (confirmed) {
            await uninstallMod(getInternalName(activeGame), modName, modSource);
            navigate(`/${getInternalName(activeGame)}/features/mods`, {
              replace: true,
            });
          }
        }}
        >{$_("gameControls_button_uninstall")}<Helper
          helperClass="!text-neutral-400 !text-xs"
          >{$_("gameControls_button_uninstall_helpText")}</Helper
        ></DropdownItem
      >
    </Dropdown>
  </div>
</div>
