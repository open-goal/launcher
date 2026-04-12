<script lang="ts">
  import IconCog from "~icons/mdi/cog";
  import { configDir, join } from "@tauri-apps/api/path";
  import { openPath } from "@tauri-apps/plugin-opener";
  import { onMount } from "svelte";
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
  import { route } from "/src/router";
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
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { asJobType } from "$lib/job/jobs";

  let {
    activeGame,
  }: {
    activeGame: SupportedGame;
  } = $props();
  let textureSupportEnabled = $state(true);
  let gameDataDir: string | undefined = $state(undefined);
  let extractedAssetsDir: string | undefined = $state(undefined);
  let settingsDir: string | undefined = $state(undefined);
  let savesDir: string | undefined = $state(undefined);

  onMount(async () => {
    textureSupportEnabled = await doesActiveToolingVersionMeetMinimum(0, 2, 13);
    await refreshDirectories();
  });

  async function refreshDirectories() {
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

  let name = $_(`gameName_${activeGame}`);
  // this doesnt work for the following languages: ja-JP, he-IL
  let [title, subtitle] = name.split(":");
  subtitle = subtitle?.trimStart();
</script>

<div class="flex flex-col items-end mt-auto ml-auto">
  <div class="flex flex-col pb-2 items-end text-shadow-lg">
    <p
      class="text-3xl font-semibold tracking-wider text-gray-100 pointer-events-none uppercase"
    >
      {title}
    </p>
    <p class="text-2xl font-medium text-amber-500">{subtitle}</p>
    <Playtime {activeGame}></Playtime>
  </div>

  <div class="flex flex-col mt-auto ml-auto">
    <div class="flex flex-col gap-2 w-[320px]">
      <Button
        class="font-semibold text-gray-200 h-[48px] text-3xl border-solid border border-[#1e3a66] rounded bg-[#13294b] hover:bg-[#183763] hover:border-[#2a4a7c] hover:text-white"
        onclick={async () => {
          launchGame(activeGame, false);
        }}>{$_("gameControls_button_play")}</Button
      >
      <div class="grid grid-cols-[1fr_1fr_40px] gap-2">
        <Button
          class="font-medium text-gray-200 h-[40px] text-center focus:ring-0 focus:outline-none border-solid border border-[#2a2a2a] rounded bg-[#0b0b0b] hover:bg-[#141414] hover:border-[#3a3a3a] hover:text-white"
          >{$_("gameControls_button_features")}</Button
        >
        <Dropdown
          simple
          trigger="hover"
          placement="top"
          class="dark:!bg-slate-900 dark:text-white **:w-full"
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
              navigate(`/:game_name/mods`, {
                params: { game_name: activeGame },
              });
            }}
          >
            {$_("gameControls_button_features_mods")}
          </DropdownItem>
        </Dropdown>
        <Button
          class="font-medium text-gray-200 h-[40px] text-center focus:ring-0 focus:outline-none border-solid border border-[#2a2a2a] rounded bg-[#0b0b0b] hover:bg-[#141414] hover:border-[#3a3a3a] hover:text-white"
        >
          {$_("gameControls_button_advanced")}
        </Button>
        <Dropdown
          simple
          trigger="hover"
          placement="top"
          class="dark:!bg-slate-900 dark:text-white **:w-full"
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
              navigate("/job/:job_type", {
                params: { job_type: asJobType("decompile") },
                search: {
                  activeGame: activeGame,
                  returnTo: route.pathname,
                },
              });
            }}
            >{$_("gameControls_button_decompile")}
            <!-- NOTE - this is a bug in flowbite-svelte, it's not replacing the default class but just appending -->
            <Helper class="dark:!text-neutral-400 !text-xs"
              >{$_("gameControls_button_decompile_helpText")}</Helper
            ></DropdownItem
          >
          <DropdownItem
            onclick={async () => {
              navigate("/job/:job_type", {
                params: { job_type: asJobType("compile") },
                search: {
                  activeGame: activeGame,
                  returnTo: route.pathname,
                },
              });
            }}
            >{$_("gameControls_button_compile")}
            <!-- NOTE - this is a bug in flowbite-svelte, it's not replacing the default class but just appending -->
            <Helper class="dark:!text-neutral-400 !text-xs"
              >{$_("gameControls_button_compile_helpText")}
            </Helper></DropdownItem
          >
          <DropdownDivider />
          <DropdownItem
            onclick={async () => {
              if (gameDataDir) {
                await openPath(gameDataDir);
              }
            }}>{$_("gameControls_button_openGameFolder")}</DropdownItem
          >
          <DropdownItem
            onclick={async () => {
              if (extractedAssetsDir) {
                await openPath(extractedAssetsDir);
              }
            }}
            >{$_("gameControls_button_openExtractedAssetsFolder")}</DropdownItem
          >
        </Dropdown>
        <Button
          class="text-gray-200 h-[40px] w-[40px] p-0 focus:ring-0 focus:outline-none border-solid border border-[#2a2a2a] rounded bg-[#0b0b0b] hover:bg-[#141414] hover:border-[#3a3a3a] hover:text-white"
        >
          <IconCog />
        </Button>
        <Dropdown
          simple
          trigger="hover"
          placement="top-end"
          class="dark:!bg-slate-900 dark:text-white **:w-full"
        >
          <!-- TODO - screenshot folder? how do we even configure where those go? -->
          <DropdownItem
            onclick={async () => {
              if (settingsDir) {
                await openPath(settingsDir);
              }
            }}>{$_("gameControls_button_openSettingsFolder")}</DropdownItem
          >
          <DropdownItem
            onclick={async () => {
              if (savesDir) {
                await openPath(savesDir);
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
              class="dark:!text-neutral-400 !text-xs"
              >{$_("gameControls_button_copyExecutableCommand_helpText_1")}<br
              />{$_(
                "gameControls_button_copyExecutableCommand_helpText_2",
              )}</Helper
            ></DropdownItem
          >
          <DropdownDivider />
          <!-- TODO - clicking this button provides zero feedback -->
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
                const successfulUninstall = await uninstallGame(activeGame);
                if (successfulUninstall) {
                  navigate("/:game_name/", {
                    params: { game_name: activeGame },
                  });
                }
              }
            }}
            >{$_("gameControls_button_uninstall")}<Helper
              class="dark:!text-neutral-400 !text-xs"
              >{$_("gameControls_button_uninstall_helpText")}</Helper
            ></DropdownItem
          >
        </Dropdown>
      </div>
    </div>
  </div>
</div>
