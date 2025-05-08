<script lang="ts">
  import { openDir } from "$lib/rpc/window";
  import IconCog from "~icons/mdi/cog";
  import { configDir, join } from "@tauri-apps/api/path";
  import { createEventDispatcher, onMount } from "svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import {
    Button,
    Dropdown,
    DropdownItem,
    DropdownDivider,
    Helper,
    Tooltip,
  } from "flowbite-svelte";
  import { resetGameSettings, uninstallGame } from "$lib/rpc/game";
  import {
    getLaunchGameString,
    launchGame,
    launchGameWithCustomExecutable,
    openREPL,
  } from "$lib/rpc/binaries";
  import {
    doesActiveToolingVersionMeetMinimum,
    getInstallationDirectory,
    getPlaytime,
  } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { navigate } from "svelte-navigator";
  import { listen } from "@tauri-apps/api/event";
  import { toastStore } from "$lib/stores/ToastStore";
  import { activeGame } from "$lib/stores/AppStore";

  const dispatch = createEventDispatcher();
  let gameDataDir: string | undefined = undefined;
  let settingsDir: string | undefined = undefined;
  let savesDir: string | undefined = undefined;
  let playtime = "";
  let textureSupportEnabled = true;

  onMount(async () => {
    let installationDir = await getInstallationDirectory();
    if (installationDir !== null) {
      gameDataDir = await join(installationDir, "active", $activeGame, "data");
    }
    settingsDir = await join(
      await configDir(),
      "OpenGOAL",
      $activeGame,
      "settings",
    );
    savesDir = await join(await configDir(), "OpenGOAL", $activeGame, "saves");

    textureSupportEnabled = await doesActiveToolingVersionMeetMinimum(0, 2, 13);
  });

  // format the time from the settings file which is stored as seconds
  function formatPlaytime(playtimeRaw: number) {
    // calculate the number of hours and minutes
    const hours = Math.floor(playtimeRaw / 3600);
    const minutes = Math.floor((playtimeRaw % 3600) / 60);

    // initialize the formatted playtime string
    let formattedPlaytime = "";

    // add the hours to the formatted playtime string
    if (hours > 0) {
      if (hours > 1) {
        formattedPlaytime += `${hours} ${$_(`gameControls_timePlayed_hours`)}`;
      } else {
        formattedPlaytime += `${hours} ${$_(`gameControls_timePlayed_hour`)}`;
      }
    }

    // add the minutes to the formatted playtime string
    if (minutes > 0) {
      // add a comma if there are already hours in the formatted playtime string
      if (formattedPlaytime.length > 0) {
        formattedPlaytime += ", ";
      }
      if (minutes > 1) {
        formattedPlaytime += `${minutes} ${$_(
          `gameControls_timePlayed_minutes`,
        )}`;
      } else {
        formattedPlaytime += `${minutes} ${$_(
          `gameControls_timePlayed_minute`,
        )}`;
      }
    }

    // return the formatted playtime string
    return formattedPlaytime;
  }

  // listen for the custom playtiemUpdated event from the backend and then refresh the playtime on screen
  listen<string>("playtimeUpdated", (event) => {
    getPlaytime($activeGame).then((result) => {
      playtime = formatPlaytime(result);
    });
  });
</script>

<div class="flex flex-col justify-end items-end mt-auto">
  <h1
    class="tracking-tighter text-2xl font-bold pb-3 text-orange-500 text-outline pointer-events-none"
  >
    {$_(`gameName_${$activeGame}`)}
  </h1>
  {#if playtime}
    <h1 class="pb-4 text-xl text-outline tracking-tighter font-extrabold">
      {`${$_(`gameControls_timePlayed_label`)} ${playtime}`}
    </h1>
  {/if}
  <div class="flex flex-row gap-2">
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      on:click={async () => {
        launchGame($activeGame, false);
      }}>{$_("gameControls_button_play")}</Button
    >
    <Button
      class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      >{$_("gameControls_button_features")}</Button
    >
    <Dropdown trigger="hover" placement="top-end" class="!bg-slate-900">
      <DropdownItem
        disabled={!textureSupportEnabled}
        on:click={async () => {
          navigate(`/${$activeGame}/texture_packs`);
        }}
      >
        {$_("gameControls_button_features_textures")}
      </DropdownItem>
      {#if !textureSupportEnabled}
        <Tooltip>{$_("gameControls_button_features_textures_disabled")}</Tooltip
        >
      {/if}
      <DropdownItem
        on:click={async () => {
          navigate(`/${$activeGame}/mods`);
        }}
      >
        {$_("gameControls_button_features_mods")}
      </DropdownItem>
    </Dropdown>
    <Button
      class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
    >
      {$_("gameControls_button_advanced")}
    </Button>
    <Dropdown trigger="hover" placement="top-end" class="!bg-slate-900">
      <DropdownItem
        on:click={async () => {
          launchGame($activeGame, true);
        }}>{$_("gameControls_button_playInDebug")}</DropdownItem
      >
      <DropdownItem
        on:click={async () => {
          launchGameWithCustomExecutable($activeGame);
        }}>Launch with Custom Executable</DropdownItem
      >
      <DropdownItem
        on:click={async () => {
          openREPL($activeGame);
        }}>{$_("gameControls_button_openREPL")}</DropdownItem
      >
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
    <Dropdown trigger="hover" placement="top-end" class="!bg-slate-900">
      <!-- TODO - screenshot folder? how do we even configure where those go? -->
      <DropdownItem
        on:click={async () => {
          if (settingsDir) {
            await openDir(settingsDir);
          }
        }}>{$_("gameControls_button_openSettingsFolder")}</DropdownItem
      >
      <DropdownItem
        on:click={async () => {
          if (savesDir) {
            await openDir(savesDir);
          }
        }}>{$_("gameControls_button_openSavesFolder")}</DropdownItem
      >
      <DropdownDivider />
      <DropdownItem
        on:click={async () => {
          const launchString = await getLaunchGameString($activeGame);
          await writeText(launchString);
          toastStore.makeToast($_("toasts_copiedToClipboard"), "info");
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
          await resetGameSettings($activeGame);
        }}>{$_("gameControls_button_resetSettings")}</DropdownItem
      >
      <DropdownItem
        on:click={async () => {
          // Get confirmation
          // TODO - probably move these confirms into the actual launcher itself
          const confirmed = await confirm(
            $_("gameControls_button_uninstall_confirmation"),
            { title: "OpenGOAL Launcher", kind: "warning" },
          );
          if (confirmed) {
            await uninstallGame($activeGame);
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
