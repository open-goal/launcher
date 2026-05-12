<script lang="ts">
  import { openPath } from "@tauri-apps/plugin-opener";
  import IconArrowLeft from "~icons/mdi/arrow-left";
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
  import { setCheckForLatestModVersion } from "$lib/rpc/config";
  import { _ } from "svelte-i18n";
  import { toastStore } from "$lib/stores/ToastStore";
  import {
    getLaunchModString,
    launchMod,
    openREPLForMod,
    resetModSettings,
    uninstallMod,
  } from "$lib/rpc/features";
  import { exists } from "@tauri-apps/plugin-fs";
  import { getModSourcesData } from "$lib/rpc/cache";
  import { platform } from "@tauri-apps/plugin-os";
  import { navigate, route } from "/src/router";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import { getModInfo } from "$lib/rpc/ModInfo";
  import { asJobType } from "$lib/job/jobs";
  import { versionState } from "/src/state/VersionState.svelte";
  import { config } from "/src/state/config.svelte";
  import { searchParams } from "sv-router";

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
  let updateCheckEnabled = $state(config?.checkForLatestModVersion);
  let modInfo: ModInfo | undefined = $state(undefined);
  let displayName: string | undefined = $state(undefined);
  let description: string | undefined = $state(undefined);

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
    modInfo = await getModInfo(activeGame, modName, modSource);
    displayName =
      modInfo.perGameConfig?.[activeGame]?.displayName || modInfo.displayName;
    description =
      modInfo.perGameConfig?.[activeGame]?.description || modInfo.description;
    await initDirectories(modInfo);
    await sortModVersions(modInfo);
  });

  onDestroy(() => {
    versionState.displayModVersion = false;
    versionState.activeModVersionInfo.installedVersion = undefined;
    versionState.activeModVersionInfo.installed = true;
  });

  async function initDirectories(modInfo: ModInfo) {
    let installationDir = config?.installationDir;
    if (installationDir) {
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
            const assetUrl = version.assets[platform()];
            if (assetUrl) {
              modAssetUrlsSorted.push(assetUrl);
            }
          }
        }
      }
    }

    // get current installed version
    // TODO: refactor this because it's doing way too much...
    let installedMods = config?.games?.[activeGame]?.mods!;
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
    updateCheckEnabled = !updateCheckEnabled;
    await setCheckForLatestModVersion(updateCheckEnabled);
  }
</script>

{#if modInfo && modInfo.name !== undefined && modInfo.source !== undefined}
  <div
    class="mt-auto ml-auto mb-2 pr-4 max-w-xl text-right border-r-2 border-orange-500/80 bg-linear-to-l from-black/75 via-black/40 to-transparent mask-y-from-95%"
  >
    <div class="flex flex-col items-end z-10">
      {#if modInfo?.websiteUrl}
        <a
          class="mt-2 gap-2 text-3xl font-semibold tracking-tight text-orange-500 hover:text-orange-600 drop-shadow-[0_2px_10px_rgba(0,0,0,0.95)]"
          target="_blank"
          rel="noreferrer"
          href={modInfo.websiteUrl}
        >
          {displayName}
        </a>
      {:else}
        <h1
          class="mt-2 text-3xl font-semibold tracking-tight text-orange-500 pointer-events-none drop-shadow-[0_2px_10px_rgba(0,0,0,0.95)]"
        >
          {displayName}
        </h1>
      {/if}

      <p
        class="mt-2 max-w-150 text-[1.05rem] font-light tracking-tight leading-6 text-white/88 pointer-events-none drop-shadow-[0_2px_10px_rgba(0,0,0,0.95)]"
      >
        {description}
      </p>

      <div class="mt-2 h-px w-full bg-white/10"></div>

      <div
        class="my-2 flex items-center justify-end gap-4 text-sm font-light tracking-wide text-white/75 pointer-events-none"
      >
        <span>
          {$_("features_mods_by")}: {Array.isArray(modInfo.authors)
            ? modInfo.authors.join(", ")
            : modInfo.authors}
        </span>

        {#if modInfo.tags?.length}
          <span class="h-5 w-px bg-white/30"></span>

          <span
            class="rounded-full border border-white/15 bg-black/20 px-3 py-1"
          >
            {Array.isArray(modInfo.tags)
              ? modInfo.tags.slice(0, 2).join(" / ")
              : modInfo.tags}
          </span>
        {/if}
      </div>
    </div>
  </div>
  <div class="flex flex-col justify-end items-end mt-3">
    <div class="flex flex-row gap-2">
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 hover:border-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          navigate((route.search.from as any) ?? "/:game_name/mods", {
            params: { game_name: activeGame },
            search: {
              sort: searchParams.get("sort") ?? "popularity",
              game: searchParams.get("game") ?? "all",
            },
          });
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
      {:else if modVersionListSorted.length == 0 || modVersionListSorted[0] === currentlyInstalledVersion || !updateCheckEnabled}
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
          class="dark:bg-slate-900! overflow-y-auto p-2 max-h-75 rounded"
        >
          <!-- wrap checkbox in div so that both box and text get tooltip -->
          <div id="checkbox_always_use_newest">
            <Checkbox
              color="orange"
              checked={updateCheckEnabled}
              onchange={toggleCheckForLatestModVersion}
            >
              {$_("gameControls_always_use_newest")}
            </Checkbox>
          </div>
          <Tooltip triggeredBy="#checkbox_always_use_newest" class="max-w-75"
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
          class="dark:bg-slate-900! rounded **:w-full"
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
            <Helper class="dark:text-neutral-400! text-xs!"
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
            <Helper class="dark:text-neutral-400! text-xs!"
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
          class="dark:bg-slate-900! **:w-full"
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
              class="dark:text-neutral-400! text-xs!"
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
              class="dark:text-neutral-400! text-xs!"
              >{$_("gameControls_button_uninstall_helpText")}</Helper
            ></DropdownItem
          >
        </Dropdown>
      {/if}
    </div>
  </div>
{/if}
