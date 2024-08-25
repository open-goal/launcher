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
    Indicator,
  } from "flowbite-svelte";
  import { platform } from "@tauri-apps/api/os";
  import {
    getInstallationDirectory,
    setModAutoUpdate,
    getModAutoUpdate,
  } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { navigate } from "svelte-navigator";
  import { toastStore } from "$lib/stores/ToastStore";
  import {
    getInstalledMods,
    getLaunchModString,
    launchMod,
    openREPLForMod,
    resetModSettings,
    uninstallMod,
  } from "$lib/rpc/features";
  import { pathExists } from "$lib/rpc/util";
  import { getModSourcesData } from "$lib/rpc/cache";
  import {
    getModAssetUrl,
    isVersionSupportedOnPlatform,
  } from "$lib/features/mods";

  export let activeGame: SupportedGame;
  export let modName: string = "";
  export let modDisplayName: string = "";
  export let modSource: string = "";

  const dispatch = createEventDispatcher();
  let gameDataDir: string | undefined = undefined;
  let settingsDir: string | undefined = undefined;
  let savesDir: string | undefined = undefined;
  let isLinux = false;
  let modVersionList: string[] = [];
  let modAssetUrls: string[] = [];
  let currentlyInstalledVersion: string = "";
  let numberOfVersionsOutOfDate = 0;
  let userPlatform: string = "";
  let autoUpdateChecked = false;

  async function addModFromUrl(
    url: string,
    modName: string,
    sourceName: string,
    modVersion: string,
  ) {
    // install it immediately
    // - prompt user for iso if it doesn't exist
    // - decompile
    // - compile
    dispatch("job", {
      type: "installModExternal",
      modDownloadUrl: url,
      modSourceName: sourceName,
      modName: modName,
      modVersion: modVersion,
    });
  }

  function getNewestVersion(versions: string[]): string {
    const versionsCopy = [...versions];

    return (
      versionsCopy
        .sort((a, b) => {
          const aParts = a.split(".").map(Number);
          const bParts = b.split(".").map(Number);

          for (let i = 0; i < Math.max(aParts.length, bParts.length); i++) {
            const aPart = aParts[i] || 0;
            const bPart = bParts[i] || 0;

            if (aPart !== bPart) {
              return aPart - bPart;
            }
          }
          return 0;
        })
        .pop() || ""
    );
  }

  onMount(async () => {
    autoUpdateChecked = await getModAutoUpdate();
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
    // Get a list of available versions.
    let sourceData = await getModSourcesData();
    userPlatform = await platform();

    let relevantSourceData = undefined;
    for (const [sourceUrl, sourceDataEntry] of Object.entries(sourceData)) {
      if (sourceDataEntry.sourceName === modSource) {
        relevantSourceData = sourceDataEntry;
      }
    }
    if (
      relevantSourceData !== undefined &&
      Object.keys(relevantSourceData.mods).includes(modName)
    ) {
      for (const version of relevantSourceData.mods[modName].versions) {
        if (
          isVersionSupportedOnPlatform(userPlatform, version) &&
          version.supportedGames !== null &&
          version.supportedGames.includes(getInternalName(activeGame))
        ) {
          modVersionList = [...modVersionList, version.version];
          const assetUrl = getModAssetUrl(userPlatform, version);
          if (assetUrl !== undefined) {
            modAssetUrls.push(assetUrl);
          }
        }
      }
    }

    // get current installed version
    let installedMods = await getInstalledMods(getInternalName(activeGame));
    if (
      Object.keys(installedMods).includes(modSource) &&
      Object.keys(installedMods[modSource]).includes(modName)
    ) {
      currentlyInstalledVersion = installedMods[modSource][modName];
    }

    for (const version of modVersionList) {
      if (version === currentlyInstalledVersion) {
        break;
      }
      numberOfVersionsOutOfDate = numberOfVersionsOutOfDate + 1;
    }
  });

  async function toggleAutoUpdate() {
    autoUpdateChecked = !autoUpdateChecked;
    await setModAutoUpdate(autoUpdateChecked);
  }
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
      }}><IconArrowLeft />&nbsp;{$_("features_mods_go_back")}</Button
    >
    {#if modVersionList.length > 0}
      <Button
        class="relative text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      >
        {$_("features_mods_versions")}
        {#if numberOfVersionsOutOfDate > 0}
          <Indicator
            color="red"
            border
            size="xl"
            placement="top-right"
            class="text-xs font-bold"
          >
            {numberOfVersionsOutOfDate}
          </Indicator>
        {/if}
      </Button>
      <Dropdown
        placement="top-end"
        class="!bg-slate-900 overflow-y-auto max-h-[300px]"
      >
        <div class="p-2">
          <label class="flex items-center space-x-2">
            <input
              type="checkbox"
              class="form-checkbox text-orange-500 bg-slate-700 rounded focus:ring-orange-500"
              id="autoUpdate"
              checked={autoUpdateChecked}
              on:change={toggleAutoUpdate}
            />
            <span class="text-sm text-white font-semibold"
              >Get Newest Version</span
            >
          </label>
        </div>
        <DropdownDivider />
        {#each modVersionList as version, i}
          {#if version === currentlyInstalledVersion}
            <DropdownItem class="text-orange-400 cursor-auto">
              {version} (current)
            </DropdownItem>
          {:else}
            <DropdownItem
              on:click={async () => {
                await addModFromUrl(
                  modAssetUrls[i],
                  modName,
                  modSource,
                  version,
                );
              }}>{version}</DropdownItem
            >
          {/if}
        {/each}
      </Dropdown>
    {/if}
    {#if getNewestVersion(modVersionList) === currentlyInstalledVersion || !autoUpdateChecked}
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => {
          launchMod(getInternalName(activeGame), false, modName, modSource);
        }}>{$_("gameControls_button_play")}</Button
      >
    {:else}
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => {
          await addModFromUrl(
            modAssetUrls[0],
            modName,
            modSource,
            getNewestVersion(modVersionList),
          );
        }}>{$_("gameControls_update_mod")}</Button
      >
    {/if}
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