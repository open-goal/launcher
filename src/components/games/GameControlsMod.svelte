<script lang="ts">
  import { openPath } from "@tauri-apps/plugin-opener";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import OpenInNew from "~icons/mdi/open-in-new";
  import IconCog from "~icons/mdi/cog";
  import { join } from "@tauri-apps/api/path";
  import { onDestroy, onMount } from "svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { confirm } from "@tauri-apps/plugin-dialog";
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
  import {
    getInstallationDirectory,
    setCheckForLatestModVersion,
    getCheckForLatestModVersion,
  } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { toastStore } from "$lib/stores/ToastStore";
  import {
    getInstalledMods,
    getLaunchModString,
    launchMod,
    openREPLForMod,
    resetModSettings,
    uninstallMod,
  } from "$lib/rpc/features";
  import { exists } from "@tauri-apps/plugin-fs";
  import { getModSourcesData } from "$lib/rpc/cache";
  import { getModAssetUrl } from "$lib/features/mods";
  import { navigate, route } from "/src/router";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import { getModInfo } from "$lib/rpc/bindings/utils/ModInfo";
  import { asJobType } from "$lib/job/jobs";
  import { versionState } from "/src/state/VersionState.svelte";

  let {
    activeGame,
    modName,
    modSource,
  }: { activeGame: SupportedGame; modName: string; modSource: string } =
    $props();

  let gameDataDir: string | undefined = undefined;
  let extractedAssetsDir: string | undefined = undefined;
  let settingsDir: string | undefined = $state(undefined);
  let savesDir: string | undefined = $state(undefined);
  let modVersionListSorted: string[] = $state([]);
  let modAssetUrlsSorted: string[] = $state([]);
  let currentlyInstalledVersion: string = $state("");
  let numberOfVersionsOutOfDate = $state(0);
  let checkForLatestModVersionChecked = $state(false);
  let modInfo: ModInfo | undefined = $state(undefined);

  async function addModFromUrl(url: string, modVersion: string) {
    navigate("/job/:job_type", {
      params: {
        job_type: asJobType("installModFromUrl"),
      },
      search: {
        activeGame: activeGame,
        modName: modName,
        modSourceName: modSource,
        modDownloadUrl: url,
        modVersion: modVersion,
        returnTo: route.pathname,
      },
    });
  }

  onMount(async () => {
    checkForLatestModVersionChecked = await getCheckForLatestModVersion();
    modInfo = await getModInfo(activeGame, modName, modSource);
    await initDirectories(modInfo);
    await sortModVersions(modInfo);
  });

  onDestroy(() => {
    versionState.displayModVersion = false;
    versionState.activeModVersionInfo.installedVersion = undefined;
    versionState.activeModVersionInfo.installed = true;
  });

  async function initDirectories(modInfo: ModInfo) {
    let installationDir = await getInstallationDirectory();
    if (installationDir !== null) {
      gameDataDir = await join(
        installationDir,
        "features",
        activeGame,
        "mods",
        modInfo.source,
        modInfo.name,
        "data",
      );
      extractedAssetsDir = await join(
        gameDataDir,
        "decompiler_out",
        activeGame,
      );
      settingsDir = await join(
        installationDir,
        "features",
        activeGame,
        "mods",
        modInfo.source,
        "_settings",
        modInfo.name,
        "OpenGOAL",
        activeGame,
        "settings",
      );
      if (!(await exists(settingsDir))) {
        settingsDir = undefined;
      }
      savesDir = await join(
        installationDir,
        "features",
        activeGame,
        "mods",
        modInfo.source,
        "_settings",
        modInfo.name,
        "OpenGOAL",
        activeGame,
        "saves",
      );
      if (!(await exists(savesDir))) {
        savesDir = undefined;
      }
    }
  }

  async function sortModVersions(modInfo: ModInfo) {
    // Get a list of available versions, this is how we see if we're on the latest!
    let sourceData = await getModSourcesData();
    let relevantSourceData = undefined;
    for (const [sourceUrl, sourceDataEntry] of Object.entries(sourceData)) {
      if (sourceDataEntry.sourceName === modInfo.source) {
        relevantSourceData = sourceDataEntry;
      }
    }

    modVersionListSorted = [];
    numberOfVersionsOutOfDate = 0;

    if (
      relevantSourceData !== undefined &&
      Object.keys(relevantSourceData.mods).includes(modInfo.name)
    ) {
      const mod = relevantSourceData.mods[modInfo.name];
      if (mod !== undefined) {
        // ensure versions are sorted by date desc (newest first)
        let versions = mod.versions;
        versions.sort((a, b) => {
          return Date.parse(b.publishedDate) - Date.parse(a.publishedDate);
        });
        for (const version of versions) {
          if (
            version.supportedGames !== null &&
            version.supportedGames.includes(activeGame)
          ) {
            modVersionListSorted = [...modVersionListSorted, version.version];
            const assetUrl = getModAssetUrl(version);
            if (assetUrl) {
              modAssetUrlsSorted.push(assetUrl);
            }
          }
        }
      }
    }

    // get current installed version
    let installedMods = await getInstalledMods(activeGame);
    if (
      Object.keys(installedMods).includes(modInfo.source) &&
      Object.keys(installedMods[modInfo.source]).includes(modInfo.name)
    ) {
      currentlyInstalledVersion = installedMods[modInfo.source][modInfo.name];
      versionState.displayModVersion = true;
      versionState.activeModVersionInfo.installedVersion =
        currentlyInstalledVersion;
      versionState.activeModVersionInfo.installed = true;
    } else {
      versionState.displayModVersion = true;
      versionState.activeModVersionInfo.installedVersion = undefined;
      versionState.activeModVersionInfo.installed = false;
    }

    for (const version of modVersionListSorted) {
      if (version === currentlyInstalledVersion) {
        break;
      }
      numberOfVersionsOutOfDate++;
    }
  }

  async function toggleCheckForLatestModVersion() {
    checkForLatestModVersionChecked = !checkForLatestModVersionChecked;
    await setCheckForLatestModVersion(checkForLatestModVersionChecked);
  }
</script>

{#if modInfo && modInfo.name !== undefined && modInfo.source !== undefined}
  <div
    class="[margin-left:35%] p-3 rounded-lg flex flex-col justify-end items-end mt-auto [background-color:rgba(0,0,0,.5)]"
  >
    {#if modInfo && modInfo.websiteUrl != undefined}
      <!-- Have website, make name a link and show icon -->
      <a
        class="inline-flex tracking-tighter font-bold pb-2 text-outline text-orange-500 hover:text-orange-600 items-center"
        target="_blank"
        rel="noreferrer"
        href={modInfo.websiteUrl}
      >
        <h1 class="text-2xl">
          {modInfo.displayName}
        </h1>
        &nbsp;<OpenInNew />
      </a>
    {:else}
      <!-- No website, just show name -->
      <h1
        class="tracking-tighter text-2xl font-bold pb-2 text-orange-500 text-outline pointer-events-none"
      >
        {modInfo.displayName}
      </h1>
    {/if}
    <h1
      class="tracking-tighter pb-2 font-bold text-outline text-justify [text-align-last:right]"
    >
      {modInfo.description}
    </h1>
    <!-- hiding this because it's bloat -->
    <!-- <p class="pb-2 text-outline">
      {$_("features_mods_tags")}: {modInfo.tags}
    </p> -->
    <p class="text-outline">
      {$_("features_mods_authors")}: {modInfo.authors}
    </p>
  </div>
  <div class="flex flex-col justify-end items-end mt-3">
    <div class="flex flex-row gap-2">
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          navigate(`/:game_name/mods`, { params: { game_name: activeGame } });
        }}><IconArrowLeft />&nbsp;{$_("features_mods_go_back")}</Button
      >
      {#if !currentlyInstalledVersion && modVersionListSorted.length == 0}
        <!-- show disabled Install button if no version installed and we have no version list (offline) -->
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800 text-sm text-white font-semibold px-5 py-2"
          disabled>{$_("gameControls_button_install")}</Button
        >
      {:else if !currentlyInstalledVersion}
        <!-- show Install button if no version installed but we're online -->
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800 text-sm text-white font-semibold px-5 py-2"
          onclick={async () => {
            await addModFromUrl(modAssetUrlsSorted[0], modVersionListSorted[0]);
          }}>{$_("gameControls_button_install")}</Button
        >
      {:else if modVersionListSorted.length == 0 || modVersionListSorted[0] === currentlyInstalledVersion || !checkForLatestModVersionChecked}
        <!-- show Play button if we have no version list (offline), if we're up to date, or we dont want forced updates -->
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800 text-sm text-white font-semibold px-5 py-2"
          onclick={async () => {
            launchMod(activeGame, false, modName, modSource);
          }}>{$_("gameControls_button_play")}</Button
        >
      {:else}
        <!-- otherwise show Update button -->
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800 text-sm text-white font-semibold px-5 py-2"
          onclick={async () => {
            await addModFromUrl(modAssetUrlsSorted[0], modVersionListSorted[0]);
          }}>{$_("gameControls_update_mod")}</Button
        >
      {/if}
      {#if modVersionListSorted.length > 0}
        <Button
          class="relative text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800"
        >
          {$_("features_mods_versions")}
          {#if numberOfVersionsOutOfDate > 0}
            <Indicator color="red" border size="xl" placement="top-right">
              <span class="text-xs font-bold">{numberOfVersionsOutOfDate}</span>
            </Indicator>
          {/if}
        </Button>
        <Dropdown
          simple
          trigger="hover"
          placement="top-end"
          class="dark:!bg-slate-900 overflow-y-auto p-2 max-h-[300px] rounded"
        >
          <!-- wrap checkbox in div so that both box and text get tooltip -->
          <div id="checkbox_always_use_newest">
            <Checkbox
              color="orange"
              checked={checkForLatestModVersionChecked}
              onchange={toggleCheckForLatestModVersion}
            >
              {$_("gameControls_always_use_newest")}
            </Checkbox>
          </div>
          <Tooltip
            triggeredBy="#checkbox_always_use_newest"
            class="max-w-[300px]"
            >{$_("gameControls_always_use_newest_tooltip")}</Tooltip
          >
          <DropdownDivider />
          {#each modVersionListSorted as version, i}
            {#if version === currentlyInstalledVersion}
              <DropdownItem class="text-orange-400 w-full">
                {version}
                {$_("gameControls_active")}
              </DropdownItem>
            {:else}
              <DropdownItem
                class="w-full"
                onclick={async () => {
                  await addModFromUrl(modAssetUrlsSorted[i], version);
                }}>{version}</DropdownItem
              >
            {/if}
          {/each}
        </Dropdown>
      {/if}
      {#if !currentlyInstalledVersion}
        <!-- Disabled "advanced" button if not installed -->
        <Button
          class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800"
          disabled
        >
          {$_("gameControls_button_advanced")}
        </Button>
      {:else}
        <Button
          class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800"
        >
          {$_("gameControls_button_advanced")}
        </Button>
        <Dropdown
          simple
          trigger="hover"
          placement="top-end"
          class="dark:!bg-slate-900 rounded **:w-full"
        >
          <DropdownItem
            onclick={async () => {
              launchMod(activeGame, true, modName, modSource);
            }}>{$_("gameControls_button_playInDebug")}</DropdownItem
          >
          <DropdownItem
            onclick={async () => {
              openREPLForMod(activeGame, modName, modSource);
            }}>{$_("gameControls_button_openREPL")}</DropdownItem
          >
          <DropdownDivider />
          <DropdownItem
            onclick={async () => {
              navigate("/job/:job_type", {
                params: {
                  job_type: asJobType("decompileMod"),
                },
                search: {
                  activeGame: activeGame,
                  modName: modName,
                  modSourceName: modSource,
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
                params: {
                  job_type: asJobType("compileMod"),
                },
                search: {
                  activeGame: activeGame,
                  modName: modName,
                  modSourceName: modSource,
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
      {/if}
      {#if !currentlyInstalledVersion}
        <!-- Disabled cog/settings button if not installed -->
        <Button
          class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800"
          disabled
        >
          <IconCog />
        </Button>
      {:else}
        <Button
          class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800"
        >
          <IconCog />
        </Button>
        <Dropdown
          simple
          trigger="hover"
          placement="top-end"
          class="dark:!bg-slate-900 **:w-full"
        >
          <!-- TODO - screenshot folder? how do we even configure where those go? -->
          {#if settingsDir}
            <DropdownItem
              onclick={async () => {
                if (settingsDir) {
                  await openPath(settingsDir);
                }
              }}>{$_("gameControls_button_openSettingsFolder")}</DropdownItem
            >
          {/if}
          {#if savesDir}
            <DropdownItem
              onclick={async () => {
                if (savesDir) {
                  await openPath(savesDir);
                }
              }}>{$_("gameControls_button_openSavesFolder")}</DropdownItem
            >
          {/if}
          {#if settingsDir || savesDir}
            <DropdownDivider />
          {/if}
          <DropdownItem
            onclick={async () => {
              const launchString = await getLaunchModString(
                activeGame,
                modName,
                modSource,
              );
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
          <DropdownItem
            onclick={async () => {
              await resetModSettings(activeGame, modName, modSource);
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
                await uninstallMod(activeGame, modName, modSource);
                navigate(`/:game_name/mods`, {
                  params: { game_name: activeGame },
                });
              }
            }}
            >{$_("gameControls_button_uninstall")}<Helper
              class="dark:!text-neutral-400 !text-xs"
              >{$_("gameControls_button_uninstall_helpText")}</Helper
            ></DropdownItem
          >
        </Dropdown>
      {/if}
    </div>
  </div>
{/if}
