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
    Checkbox,
    Dropdown,
    DropdownItem,
    DropdownDivider,
    Helper,
    Indicator,
    Tooltip,
  } from "flowbite-svelte";
  import { platform } from "@tauri-apps/api/os";
  import {
    getInstallationDirectory,
    setCheckForLatestModVersion,
    getCheckForLatestModVersion,
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
  let modVersionListSorted: string[] = [];
  let modAssetUrlsSorted: string[] = [];
  let currentlyInstalledVersion: string = "";
  let numberOfVersionsOutOfDate = 0;
  let userPlatform: string = "";
  let checkForLatestModVersionChecked = false;

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

  onMount(async () => {
    checkForLatestModVersionChecked = await getCheckForLatestModVersion();
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
    // Get a list of available versions, this is how we see if we're on the latest!
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
      // ensure versions are sorted by date desc (newest first)
      let versions = relevantSourceData.mods[modName].versions;
      versions.sort((a, b) => {
        return Date.parse(b.publishedDate) - Date.parse(a.publishedDate);
      });

      for (const version of versions) {
        if (
          isVersionSupportedOnPlatform(userPlatform, version) &&
          version.supportedGames !== null &&
          version.supportedGames.includes(getInternalName(activeGame))
        ) {
          modVersionListSorted = [...modVersionListSorted, version.version];
          const assetUrl = getModAssetUrl(userPlatform, version);
          if (assetUrl !== undefined) {
            modAssetUrlsSorted.push(assetUrl);
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

    for (const version of modVersionListSorted) {
      if (version === currentlyInstalledVersion) {
        break;
      }
      numberOfVersionsOutOfDate = numberOfVersionsOutOfDate + 1;
    }
  });

  async function toggleCheckForLatestModVersion() {
    checkForLatestModVersionChecked = !checkForLatestModVersionChecked;
    await setCheckForLatestModVersion(checkForLatestModVersionChecked);
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
    {#if modVersionListSorted[0] === currentlyInstalledVersion || !checkForLatestModVersionChecked}
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
            modAssetUrlsSorted[0],
            modName,
            modSource,
            modVersionListSorted[0],
          );
        }}>{$_("gameControls_update_mod")}</Button
      >
    {/if}
    {#if modVersionListSorted.length > 0}
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
        class="!bg-slate-900 overflow-y-auto px-2 py-2 max-h-[300px]"
      >
        <!-- wrap checkbox in div so that both box and text get tooltip -->
        <div id="checkbox_always_use_newest"><Checkbox color="orange" checked={checkForLatestModVersionChecked} on:change={toggleCheckForLatestModVersion}>
          {$_("gameControls_always_use_newest")}
        </Checkbox></div>
        <Tooltip triggeredBy="#checkbox_always_use_newest">{$_("gameControls_always_use_newest_tooltip")}</Tooltip>
        <DropdownDivider />
        {#each modVersionListSorted as version, i}
          {#if version === currentlyInstalledVersion}
            <DropdownItem class="text-orange-400 cursor-auto">
              {version} {$_("gameControls_active")}
            </DropdownItem>
          {:else}
            <DropdownItem
              on:click={async () => {
                await addModFromUrl(
                  modAssetUrlsSorted[i],
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