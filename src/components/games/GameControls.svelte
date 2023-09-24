<script lang="ts">
  import { getInternalName, SupportedGame } from "$lib/constants";
  import { openDir } from "$lib/rpc/window";
  import IconCog from "~icons/mdi/cog";
  import { configDir, join } from "@tauri-apps/api/path";
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
  import { resetGameSettings, uninstallGame } from "$lib/rpc/game";
  import { platform } from "@tauri-apps/api/os";
  import { getLaunchGameString, launchGame, openREPL } from "$lib/rpc/binaries";
  import { _ } from "svelte-i18n";
  import { navigate } from "svelte-navigator";
  import { toastStore } from "$lib/stores/ToastStore";

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
      "settings",
    );
    savesDir = await join(
      await configDir(),
      "OpenGOAL",
      getInternalName(activeGame),
      "saves",
    );
  });
</script>

<div class="flex flex-col justify-end items-end mt-auto">
  <!-- TOOO - time played -->
  <h1
    class="tracking-tighter text-2xl font-bold pb-3 text-orange-500 text-outline pointer-events-none"
  >
    {$_(`gameName_${getInternalName(activeGame)}`)}
  </h1>
  <div class="flex flex-row gap-2">
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      on:click={async () => {
        launchGame(getInternalName(activeGame), false);
      }}>{$_("gameControls_button_play")}</Button
    >
    <Button
      class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      >{$_("gameControls_button_features")}</Button
    >
    <Dropdown placement="top-end" class="!bg-slate-900">
      <DropdownItem
        on:click={async () => {
          navigate(`/${getInternalName(activeGame)}/features/texture_packs`);
        }}
      >
        {$_("gameControls_button_features_textures")}
      </DropdownItem>
    </Dropdown>
    <Button
      class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
    >
      {$_("gameControls_button_advanced")}
    </Button>
    <Dropdown placement="top-end" class="!bg-slate-900">
      <DropdownItem
        on:click={async () => {
          launchGame(getInternalName(activeGame), true);
        }}>{$_("gameControls_button_playInDebug")}</DropdownItem
      >
      {#if !isLinux}
        <DropdownItem
          on:click={async () => {
            openREPL(getInternalName(activeGame));
          }}>{$_("gameControls_button_openREPL")}</DropdownItem
        >
      {/if}
      <DropdownDivider />
      <DropdownItem
        on:click={async () => {
          dispatch("job", {
            type: "decompile",
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
            type: "compile",
          });
        }}
        >{$_("gameControls_button_compile")}
        <!-- NOTE - this is a bug in flowbite-svelte, it's not replacing the default class but just appending -->
        <Helper helperClass="!text-neutral-400 !text-xs"
          >{$_("gameControls_button_compile_helpText")}
        </Helper></DropdownItem
      >
    </Dropdown>
    <Button
      class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
    >
      <IconCog />
    </Button>
    <Dropdown placement="top-end" class="!bg-slate-900">
      <!-- TODO - screenshot folder? how do we even configure where those go? -->
      <DropdownItem
        on:click={async () => {
          await openDir(settingsDir);
        }}>{$_("gameControls_button_openSettingsFolder")}</DropdownItem
      >
      <DropdownItem
        on:click={async () => {
          await openDir(savesDir);
        }}>{$_("gameControls_button_openSavesFolder")}</DropdownItem
      >
      <DropdownDivider />
      <DropdownItem
        on:click={async () => {
          const launchString = await getLaunchGameString(
            getInternalName(activeGame),
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
      <!-- TODO - verify installation -->
      <!-- <DropdownItem>Verify&nbsp;Install</DropdownItem> -->
      <DropdownItem
        on:click={async () => {
          await resetGameSettings(getInternalName(activeGame));
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
            await uninstallGame(getInternalName(activeGame));
            dispatch("change");
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
