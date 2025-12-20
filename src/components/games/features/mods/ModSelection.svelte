<script lang="ts">
  import { platform } from "@tauri-apps/plugin-os";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { Button, Input, Spinner } from "flowbite-svelte";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import { getModSourcesData, refreshModSources } from "$lib/rpc/cache";
  import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";
  import { filePrompt } from "$lib/utils/file-dialogs";
  import { extractNewMod, getInstalledMods } from "$lib/rpc/features";
  import { basename } from "@tauri-apps/api/path";
  import { navigate, route } from "/src/router";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import { asJobType } from "$lib/job/jobs";
  import InstalledModList from "./InstalledModList.svelte";
  import UninstalledModList from "./UninstalledModList.svelte";

  let activeGame: SupportedGame | undefined = $state(undefined);
  let loaded = $state(false);
  let modFilter = $state("");
  let addingMod = $state(false);
  let addingFromFile = $state(false);
  let installedMods: Record<string, Record<string, string>> = $state({});
  let sourceData: Record<string, ModSourceData> = $state({});

  const onWindows = platform() === "windows";

  onMount(async () => {
    await refreshModSources();
    sourceData = await getModSourcesData();
    loaded = true;
  });

  const gameParam = $derived(route.params.game_name);
  $effect(() => {
    const activeGameFromParam = toSupportedGame(gameParam);
    if (activeGameFromParam) {
      activeGame = activeGameFromParam;
      getInstalledMods(activeGame).then((val) => {
        installedMods = val;
      });
    }
  });

  async function addModFromFile(activeGame: SupportedGame) {
    addingMod = true;
    addingFromFile = true;
    let modArchivePath;
    if (onWindows) {
      modArchivePath = await filePrompt(["zip"], "ZIP", "Select a mod");
    } else {
      modArchivePath = await filePrompt(["gz"], "TAR", "Select a mod");
    }
    if (modArchivePath === null) {
      addingMod = false;
      addingFromFile = false;
      return;
    }
    // extract the file into install_dir/features/<game>/_local/zip-name
    await extractNewMod(activeGame, modArchivePath, "_local");
    let modName = await basename(
      modArchivePath,
      onWindows ? ".zip" : ".tar.gz",
    );
    if (!onWindows && modName.endsWith(".tar")) {
      modName = modName.substring(0, modName.indexOf(".tar"));
    }
    // install it
    navigate("/job/:job_type", {
      params: {
        job_type: asJobType("installModLocally"),
      },
      search: {
        activeGame: activeGame,
        modName: modName,
        modSourceName: "_local",
        modVersion: "local",
      },
    });
  }
</script>

<div class="flex flex-col h-full bg-[#1e1e1e]">
  {#if !loaded || !activeGame}
    <div class="flex flex-col h-full justify-center items-center">
      <Spinner color="yellow" size={"12"} />
    </div>
  {:else}
    <div class="pb-20 overflow-y-auto p-4">
      <div class="flex flex-row gap-2 items-center">
        <Button
          outline
          class="flex-shrink border-solid rounded text-white hover:dark:text-slate-900 hover:bg-white font-semibold px-2 py-2"
          onclick={async () => {
            if (activeGame) {
              navigate(`/:game_name`, { params: { game_name: activeGame } });
            }
          }}
          aria-label={$_("features_backToGamePage_buttonAlt")}
        >
          <IconArrowLeft />
        </Button>
        <Button
          class="flex-shrink border-solid rounded bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2"
          onclick={() => {
            if (activeGame) {
              addModFromFile(activeGame);
            }
          }}
          aria-label={$_("features_mods_addFromFile_buttonAlt")}
          disabled={addingMod}
        >
          {#if addingFromFile}
            <Spinner class="mr-3" size="4" color="yellow" />
          {/if}
          {$_("features_mods_addFromFile")}</Button
        >
      </div>
      <div class="mt-4">
        <Input
          placeholder={$_("features_mods_filter_placeholder")}
          bind:value={modFilter}
        />
      </div>
      <h2 class="font-bold mt-2">{$_("features_mods_installed_header")}</h2>
      <InstalledModList
        {activeGame}
        modList={installedMods}
        {modFilter}
        modSourceData={sourceData}
      ></InstalledModList>
      <h2 class="font-bold mt-5">{$_("features_mods_available_header")}</h2>
      <UninstalledModList
        {activeGame}
        installedModList={installedMods}
        {modFilter}
        modSourceData={sourceData}
      ></UninstalledModList>
    </div>
  {/if}
</div>
