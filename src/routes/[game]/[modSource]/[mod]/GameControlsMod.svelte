<script lang="ts">
  import { revealItemInDir } from "@tauri-apps/plugin-opener";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import IconCog from "~icons/mdi/cog";
  import { join } from "@tauri-apps/api/path";
  import { createEventDispatcher, onMount } from "svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import {
    Button,
    Checkbox,
    Dropdown,
    DropdownItem,
    DropdownDivider,
    Helper,
    Tooltip,
  } from "flowbite-svelte";
  import { platform } from "@tauri-apps/plugin-os";
  import {
    setCheckForLatestModVersion,
    getCheckForLatestModVersion,
  } from "$lib/rpc/config";
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
  import { getModSourcesData } from "$lib/rpc/cache";
  import {
    getModAssetUrl,
    isVersionSupportedOnPlatform,
  } from "$lib/features/mods";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { activeGame, modInfoStore } from "$lib/stores/AppStore";

  const dispatch = createEventDispatcher();
  let gameDataDir: string | undefined = undefined;
  let extractedAssetsDir: string | undefined = undefined;
  let settingsDir: string | undefined = $state(undefined);
  let savesDir: string | undefined = $state(undefined);
  let modVersionListSorted: string[] = $state([]);
  let modAssetUrlsSorted: string[] = $state([]);
  let currentlyInstalledVersion: string = $state("");
  let userPlatform: string = platform();
  let checkForLatestModVersionChecked = $state(false);

  onMount(async () => {
    checkForLatestModVersionChecked = await getCheckForLatestModVersion();
    gameDataDir = await join(
      installationDir,
      "features",
      $activeGame,
      "mods",
      $modInfoStore?.source,
      $modInfoStore?.name,
      "data",
    );
    extractedAssetsDir = await join(gameDataDir, "decompiler_out", $activeGame);
    settingsDir = await join(
      installationDir,
      "features",
      $activeGame,
      "mods",
      $modInfoStore?.source,
      "_settings",
      $modInfoStore?.name,
      "OpenGOAL",
      $activeGame,
      "settings",
    );
    if (!(await pathExists(settingsDir))) {
      settingsDir = undefined;
    }
    savesDir = await join(
      installationDir,
      "features",
      $activeGame,
      "mods",
      $modInfoStore?.source,
      "_settings",
      $modInfoStore?.name,
      "OpenGOAL",
      $activeGame,
      "saves",
    );
    // Get a list of available versions, this is how we see if we're on the latest!
    let sourceData = await getModSourcesData();

    let relevantSourceData = undefined;
    for (const [sourceUrl, sourceDataEntry] of Object.entries(sourceData)) {
      if (sourceDataEntry.sourceName === $modInfoStore?.source) {
        relevantSourceData = sourceDataEntry;
      }
    }
    if (
      relevantSourceData !== undefined &&
      Object.keys(relevantSourceData.mods).includes($modInfoStore?.name)
    ) {
      // ensure versions are sorted by date desc (newest first)
      let versions = relevantSourceData.mods[$modInfoStore?.name].versions;
      versions.sort((a, b) => {
        return Date.parse(b.publishedDate) - Date.parse(a.publishedDate);
      });

      for (const version of versions) {
        if (
          isVersionSupportedOnPlatform(userPlatform, version) &&
          version.supportedGames !== null &&
          version.supportedGames.includes($activeGame)
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
    if (
      Object.keys(installedMods).includes($modInfoStore?.source) &&
      Object.keys(installedMods[$modInfoStore?.source]).includes(
        $modInfoStore?.name,
      )
    ) {
      $VersionStore.activeVersionName = currentlyInstalledVersion;
    } else {
      $VersionStore.activeVersionName = "not set!";
    }

    for (const version of modVersionListSorted) {
      if (version === currentlyInstalledVersion) {
        break;
      }
    }
  });

  async function toggleCheckForLatestModVersion() {
    checkForLatestModVersionChecked = !checkForLatestModVersionChecked;
    await setCheckForLatestModVersion(checkForLatestModVersionChecked);
  }
</script>

<div
  class="[margin-left:35%] p-3 rounded-lg flex flex-col justify-end items-end mt-auto [background-color:rgba(0,0,0,.5)]"
>
  <h1
    class="tracking-tighter text-2xl font-bold pb-2 text-orange-500 text-outline pointer-events-none"
  >
    {$modInfoStore?.displayName}
  </h1>
  <h1
    class="tracking-tighter pb-2 font-bold text-outline text-justify [text-align-last:right]"
  >
    {$modInfoStore?.description}
  </h1>
  <p class="pb-2 text-outline">
    {$_("features_mods_tags")}: {$modInfoStore?.tags}
  </p>
  <p class="text-outline">
    {$_("features_mods_authors")}: {$modInfoStore?.authors}
  </p>
</div>
<div class="flex flex-col justify-end items-end mt-3">
  <div class="flex flex-row gap-2">
    <Button
      class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      onclick={async () => {
        navigate(`/${$activeGame}/mods`, {
          replace: true,
        });
      }}><IconArrowLeft />&nbsp;{$_("features_mods_go_back")}</Button
    >
    {#if currentlyInstalledVersion == "" && modVersionListSorted.length == 0}
      <!-- show disabled Install button if no version installed and we have no version list (offline) -->
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        disabled>{$_("gameControls_button_install")}</Button
      >
    {:else if currentlyInstalledVersion == ""}
      <!-- show Install button if no version installed but we're online -->
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          await addModFromUrl(
            modAssetUrlsSorted[0],
            $modInfoStore?.source,
            modVersionListSorted[0],
          );
        }}>{$_("gameControls_button_install")}</Button
      >
    {:else if modVersionListSorted.length == 0 || modVersionListSorted[0] === currentlyInstalledVersion || !checkForLatestModVersionChecked}
      <!-- show Play button if we have no version list (offline), if we're up to date, or we dont want forced updates -->
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          launchMod(
            $activeGame,
            false,
            $modInfoStore?.name,
            $modInfoStore?.source,
          );
        }}>{$_("gameControls_button_play")}</Button
      >
    {:else}
      <!-- otherwise show Update button -->
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          await addModFromUrl(
            modAssetUrlsSorted[0],
            $modInfoStore?.source,
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
      </Button>
      <Dropdown
        simple
        trigger="hover"
        placement="top-end"
        class="!bg-slate-900 overflow-y-auto p-2 max-h-[300px] rounded-none"
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
        <Tooltip triggeredBy="#checkbox_always_use_newest"
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
                await addModFromUrl(
                  modAssetUrlsSorted[i],
                  $modInfoStore?.source,
                  version,
                );
              }}>{version}</DropdownItem
            >
          {/if}
        {/each}
      </Dropdown>
    {/if}
    {#if currentlyInstalledVersion == ""}
      <!-- Disabled "advanced" button if not installed -->
      <Button
        class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
        disabled
      >
        {$_("gameControls_button_advanced")}
      </Button>
    {:else}
      <Button
        class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      >
        {$_("gameControls_button_advanced")}
      </Button>
      <Dropdown
        simple
        trigger="hover"
        placement="top-end"
        class="!bg-slate-900 rounded-none **:w-full"
      >
        <DropdownItem
          onclick={async () => {
            launchMod(
              $activeGame,
              true,
              $modInfoStore?.name,
              $modInfoStore?.source,
            );
          }}>{$_("gameControls_button_playInDebug")}</DropdownItem
        >
        <DropdownItem
          onclick={async () => {
            openREPLForMod(
              $activeGame,
              $modInfoStore?.name,
              $modInfoStore?.source,
            );
          }}>{$_("gameControls_button_openREPL")}</DropdownItem
        >
        <DropdownDivider />
        <DropdownItem
          onclick={async () => {
            dispatch("job", {
              type: "decompileMod",
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
              type: "compileMod",
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
    {/if}
    {#if currentlyInstalledVersion == ""}
      <!-- Disabled cog/settings button if not installed -->
      <Button
        class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
        disabled
      >
        <IconCog />
      </Button>
    {:else}
      <Button
        class="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      >
        <IconCog />
      </Button>
      <Dropdown
        simple
        trigger="hover"
        placement="top-end"
        class="!bg-slate-900 **:w-full"
      >
        <!-- TODO - screenshot folder? how do we even configure where those go? -->
        {#if settingsDir}
          <DropdownItem
            onclick={async () => {
              if (settingsDir) {
                await revealItemInDir(settingsDir);
              }
            }}>{$_("gameControls_button_openSettingsFolder")}</DropdownItem
          >
        {/if}
        {#if savesDir}
          <DropdownItem
            onclick={async () => {
              if (savesDir) {
                await revealItemInDir(savesDir);
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
              $activeGame,
              $modInfoStore?.name,
              $modInfoStore?.source,
            );
            await writeText(launchString);
            toastStore.push($_("toasts_copiedToClipboard"), "info");
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
            await resetModSettings(
              $activeGame,
              $modInfoStore?.name,
              $modInfoStore?.source,
            );
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
              await uninstallMod(
                $activeGame,
                $modInfoStore?.name,
                $modInfoStore?.source,
              );
              navigate(`/${$activeGame}/mods`, {
                replace: true,
              });
            }
          }}
          >{$_("gameControls_button_uninstall")}<Helper
            class="!text-neutral-400 !text-xs"
            >{$_("gameControls_button_uninstall_helpText")}</Helper
          ></DropdownItem
        >
      </Dropdown>
    {/if}
  </div>
</div>
