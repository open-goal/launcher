<script lang="ts">
  import IconCog from "~icons/mdi/cog";
  import { configDir, join } from "@tauri-apps/api/path";
  import { revealItemInDir } from "@tauri-apps/plugin-opener";
  import { createEventDispatcher, onMount } from "svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import {
    Button,
    Dropdown,
    DropdownItem,
    DropdownDivider,
    Helper,
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
  } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { toastStore } from "$lib/stores/ToastStore";
  import Playtime from "./Playtime.svelte";
  import { navigate } from "/src/router";
  import { activeGameState } from "/src/state/ActiveGameState.svelte";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";

  const activeGame = $derived(activeGameState.game);
  let textureSupportEnabled = $state(true);
  let gameDataDir: string | undefined = $state(undefined);
  let extractedAssetsDir: string | undefined = $state(undefined);
  let settingsDir: string | undefined = $state(undefined);
  let savesDir: string | undefined = $state(undefined);

  const dispatch = createEventDispatcher();

  onMount(async () => {
    textureSupportEnabled = await doesActiveToolingVersionMeetMinimum(0, 2, 13);
  });

  $effect(() => {
    refreshDirectories(activeGame);
  });

  async function refreshDirectories(activeGame: SupportedGame | undefined) {
    if (!activeGame) {
      return;
    }
    savesDir = await join(await configDir(), "OpenGOAL", activeGame, "saves");
    settingsDir = await join(
      await configDir(),
      "OpenGOAL",
      activeGame,
      "settings",
    );
    let installationDir = await getInstallationDirectory();
    if (installationDir !== null) {
      gameDataDir = await join(installationDir, "active", activeGame, "data");
      extractedAssetsDir = await join(
        gameDataDir,
        "decompiler_out",
        activeGame,
      );
    }
  }
</script>

{#if activeGame}
  <div class="flex flex-col justify-end items-end mt-auto">
    <h1
      class="tracking-tighter text-2xl font-bold pb-3 text-orange-500 text-outline pointer-events-none"
    >
      {$_(`gameName_${activeGame}`)}
    </h1>
    <Playtime></Playtime>
    <div class="flex flex-row gap-2">
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          launchGame(activeGame, false);
        }}>{$_("gameControls_button_play")}</Button
      >
      <Button
        class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
        >{$_("gameControls_button_features")}</Button
      >
      <Dropdown
        simple
        trigger="hover"
        placement="top"
        class="!bg-slate-900 dark:text-white **:w-full"
      >
        <DropdownItem
          hidden={!textureSupportEnabled}
          disabled={!textureSupportEnabled}
          onclick={async () => {
            navigate(`/:game_name/texture_packs`, {
              params: { game_name: activeGame },
            });
          }}
        >
          {$_("gameControls_button_features_textures")}
        </DropdownItem>
        <DropdownItem
          onclick={async () => {
            navigate(`/:game_name/mods`, { params: { game_name: activeGame } });
          }}
        >
          {$_("gameControls_button_features_mods")}
        </DropdownItem>
      </Dropdown>
      <Button
        class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      >
        {$_("gameControls_button_advanced")}
      </Button>
      <Dropdown
        simple
        trigger="hover"
        placement="top"
        class="!bg-slate-900 dark:text-white **:w-full"
      >
        <DropdownItem
          onclick={async () => {
            launchGame(activeGame, true);
          }}>{$_("gameControls_button_playInDebug")}</DropdownItem
        >
        <DropdownItem
          onclick={async () => {
            launchGameWithCustomExecutable(activeGame);
          }}>{$_("gameControls_button_customExe")}</DropdownItem
        >
        <DropdownItem
          onclick={async () => {
            openREPL(activeGame);
          }}>{$_("gameControls_button_openREPL")}</DropdownItem
        >
        <DropdownDivider />
        <DropdownItem
          onclick={async () => {
            dispatch("job", {
              type: "decompile",
            });
          }}
          >{$_("gameControls_button_decompile")}
          <!-- NOTE - this is a bug in flowbite-svelte, it's not replacing the default class but just appending -->
          <Helper class="!text-neutral-400 !text-xs"
            >{$_("gameControls_button_decompile_helpText")}</Helper
          ></DropdownItem
        >
        <DropdownItem
          onclick={async () => {
            dispatch("job", {
              type: "compile",
            });
          }}
          >{$_("gameControls_button_compile")}
          <!-- NOTE - this is a bug in flowbite-svelte, it's not replacing the default class but just appending -->
          <Helper class="!text-neutral-400 !text-xs"
            >{$_("gameControls_button_compile_helpText")}
          </Helper></DropdownItem
        >
        <DropdownDivider />
        <DropdownItem
          onclick={async () => {
            if (gameDataDir) {
              await revealItemInDir(gameDataDir);
            }
          }}>{$_("gameControls_button_openGameFolder")}</DropdownItem
        >
        <DropdownItem
          onclick={async () => {
            if (extractedAssetsDir) {
              await revealItemInDir(extractedAssetsDir);
            }
          }}>{$_("gameControls_button_openExtractedAssetsFolder")}</DropdownItem
        >
      </Dropdown>
      <Button
        class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      >
        <IconCog />
      </Button>
      <Dropdown
        simple
        trigger="hover"
        placement="top"
        class="!bg-slate-900 dark:text-white **:w-full"
      >
        <!-- TODO - screenshot folder? how do we even configure where those go? -->
        <DropdownItem
          onclick={async () => {
            if (settingsDir) {
              await revealItemInDir(settingsDir);
            }
          }}>{$_("gameControls_button_openSettingsFolder")}</DropdownItem
        >
        <DropdownItem
          onclick={async () => {
            if (savesDir) {
              await revealItemInDir(savesDir);
            }
          }}>{$_("gameControls_button_openSavesFolder")}</DropdownItem
        >
        <DropdownDivider />
        <DropdownItem
          onclick={async () => {
            const launchString = await getLaunchGameString(activeGame);
            await writeText(launchString);
            toastStore.makeToast($_("toasts_copiedToClipboard"), "info");
          }}
          >{$_("gameControls_button_copyExecutableCommand")}<Helper
            class="!text-neutral-400 !text-xs"
            >{$_("gameControls_button_copyExecutableCommand_helpText_1")}<br
            />{$_(
              "gameControls_button_copyExecutableCommand_helpText_2",
            )}</Helper
          ></DropdownItem
        >
        <DropdownDivider />
        <DropdownItem
          onclick={async () => {
            await resetGameSettings(activeGame);
          }}>{$_("gameControls_button_resetSettings")}</DropdownItem
        >
        <DropdownItem
          onclick={async () => {
            // Get confirmation
            // TODO - probably move these confirms into the actual launcher itself
            const confirmed = await confirm(
              $_("gameControls_button_uninstall_confirmation"),
              { title: "OpenGOAL Launcher", kind: "warning" },
            );
            if (confirmed) {
              await uninstallGame(activeGame);
              dispatch("change");
            }
          }}
          >{$_("gameControls_button_uninstall")}<Helper
            class="!text-neutral-400 !text-xs"
            >{$_("gameControls_button_uninstall_helpText")}</Helper
          ></DropdownItem
        >
      </Dropdown>
    </div>
  </div>
{/if}
