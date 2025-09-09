<script lang="ts">
  import {
    Button,
    Checkbox,
    Dropdown,
    DropdownDivider,
    DropdownItem,
    Helper,
    Tooltip,
  } from "flowbite-svelte";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import IconCog from "~icons/mdi/cog";
  import type { PageProps } from "./mod/$types";
  import { _ } from "svelte-i18n";
  import { goto, invalidateAll } from "$app/navigation";
  import {
    getLaunchModString,
    launchMod,
    openREPLForMod,
    resetModSettings,
    uninstallMod,
  } from "$lib/rpc/features";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { revealItemInDir } from "@tauri-apps/plugin-opener";
  import { installModExternal, runJob } from "$lib/utils/jobs";

  let { data }: PageProps = $props();
  const config = $derived(data.config);
  const game = $derived(data.game);
  const mod = $derived(data.modInfo);
  const modName = $derived(data.modName);
  const source = $derived(data.source);
  const installedVersion = $derived(data.installedVersion);
  const versions = $derived(data.versions);
  const gameDataDir = data.gameDataDir;
  const extractedAssetsDir = data.extractedAssetsDir;

  // temporary while i figure out what the hell this code is doing
  const modVersionListSorted = mod.versions;
  // console.log(modVersionListSorted);
  let checkForLatestModVersionChecked = false;
</script>

<!-- BACKGROUND -->
<img
  class="absolute right-0 top-0 w-screen h-screen -z-100"
  src={mod.coverArtUrl ||
    mod.perGameConfig[game].coverArtUrl ||
    `/images/mod/coverart-placeholder.webp`}
  alt=""
/>

<div class="absolute right-4 bottom-4 z-0">
  <h1
    class="tracking-tighter text-2xl font-bold pb-2 text-orange-500 text-outline pointer-events-none"
  >
    {mod.displayName}
  </h1>
  <h1
    class="tracking-tighter pb-2 font-bold text-outline text-justify [text-align-last:right]"
  >
    {mod.description}
  </h1>
  <p class="pb-2 text-outline">
    {$_("features_mods_tags")}: {mod.tags}
  </p>
  <p class="text-outline">
    {$_("features_mods_authors")}: {mod.authors}
  </p>
  <div class="flex flex-col justify-end items-end mt-3">
    <div class="flex flex-row gap-2">
      <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          // goto(`/${game}/mods`);
          window.history.back();
        }}><IconArrowLeft />&nbsp;{$_("features_mods_go_back")}</Button
      >
      {#if installedVersion == "" && modVersionListSorted.length == 0}
        <!-- show disabled Install button if no version installed and we have no version list (offline) -->
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
          disabled>{$_("gameControls_button_install")}</Button
        >
      {:else if installedVersion == ""}
        <!-- show Install button if no version installed but we're online -->
        <!-- <Button
        class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          await addModFromUrl(
            modAssetUrlsSorted[0],
            mod.source,
            modVersionListSorted[0],
          );
        }}>{$_("gameControls_button_install")}</Button
      > -->
      {:else if modVersionListSorted.length == 0 || modVersionListSorted[0] === installedVersion || !checkForLatestModVersionChecked}
        <!-- show Play button if we have no version list (offline), if we're up to date, or we dont want forced updates -->
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
          onclick={async () => {
            launchMod(game, false, mod.name, mod.source);
          }}>{$_("gameControls_button_play")}</Button
        >
      {:else}
        <!-- otherwise show Update button -->
        <Button
          class="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
          onclick={async () => {
            await addModFromUrl(
              modAssetUrlsSorted[0],
              mod.source,
              modVersionListSorted[0],
            );
          }}>{$_("gameControls_update_mod")}</Button
        >
      {/if}
      {#if versions.length > 0}
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
            <Checkbox color="orange" checked={checkForLatestModVersionChecked}>
              {$_("gameControls_always_use_newest")}
            </Checkbox>
          </div>
          <Tooltip triggeredBy="#checkbox_always_use_newest"
            >{$_("gameControls_always_use_newest_tooltip")}</Tooltip
          >
          <DropdownDivider />
          {#each versions as { version, url }}
            {#if version === installedVersion}
              <DropdownItem class="text-orange-400 w-full">
                {version}
                {$_("gameControls_active")}
              </DropdownItem>
            {:else}
              <DropdownItem
                class="w-full"
                onclick={async () => {
                  await runJob(
                    installModExternal(game, modName, source, version, url),
                  );
                }}>{version}</DropdownItem
              >
            {/if}
          {/each}
        </Dropdown>
      {/if}
      {#if installedVersion == ""}
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
              launchMod(game, true, mod.name, mod.source);
            }}>{$_("gameControls_button_playInDebug")}</DropdownItem
          >
          <DropdownItem
            onclick={async () => {
              openREPLForMod(game, mod.name, mod.source);
            }}>{$_("gameControls_button_openREPL")}</DropdownItem
          >
          <DropdownDivider />
          <DropdownItem
            onclick={async () => {
              // decompile mod
            }}
            >{$_("gameControls_button_decompile")}
            <Helper class="!text-neutral-400 !text-xs"
              >{$_("gameControls_button_decompile_helpText")}</Helper
            ></DropdownItem
          >
          <DropdownItem
            onclick={async () => {
              // compile mod
            }}
            >{$_("gameControls_button_compile")}
            <Helper class="!text-neutral-400 !text-xs"
              >{$_("gameControls_button_compile_helpText")}
            </Helper></DropdownItem
          >
          {#if gameDataDir}
            <DropdownDivider />
            <DropdownItem
              onclick={async () => {
                await revealItemInDir(gameDataDir);
              }}>{$_("gameControls_button_openGameFolder")}</DropdownItem
            >
          {/if}
          {#if extractedAssetsDir}
            <DropdownItem
              onclick={async () => {
                await revealItemInDir(extractedAssetsDir);
              }}
              >{$_(
                "gameControls_button_openExtractedAssetsFolder",
              )}</DropdownItem
            >
          {/if}
        </Dropdown>
      {/if}
      {#if installedVersion == ""}
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
                game,
                mod.name,
                mod.source,
              );
              await writeText(launchString);
              // toastStore.push($_("toasts_copiedToClipboard"), "info");
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
              await resetModSettings(game, mod.name, mod.source);
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
                await uninstallMod(game, mod.name, mod.source);
                goto(`/${game}/mods`);
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
</div>
