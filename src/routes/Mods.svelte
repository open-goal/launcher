<script lang="ts">
  import { platform } from "@tauri-apps/plugin-os";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { Button, Input, Spinner } from "flowbite-svelte";
  import IconArrowLeft from "~icons/mdi/arrow-left";
  import {
    getModSourcesData,
    refreshModSources,
    getAvailableMods,
  } from "$lib/rpc/cache";
  import type { ModSourceData } from "$lib/rpc/bindings/ModSourceData";
  import { filePrompt } from "$lib/utils/file-dialogs";
  import { extractNewMod, getInstalledModsByGame } from "$lib/rpc/features";
  import { basename } from "@tauri-apps/api/path";
  import { navigate, route } from "/src/router";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import { asJobType } from "$lib/job/jobs";
  import InstalledMods from "../components/mods/Installed.svelte";
  import AvailableMods from "../components/mods/Available.svelte";
  import ModCard from "../components/mods/Card.svelte";

  // let activeGame: SupportedGame | undefined = $state(undefined);
  let loaded = $state(false);
  let mods = $state();
  let modFilter = $state("");
  let addingMod = $state(false);
  let addingFromFile = $state(false);
  let installedMods: Record<string, Record<string, string>> = $state({});
  let sourceData: Record<string, ModSourceData> = $state({});

  const onWindows = platform() === "windows";

  onMount(async () => {
    await refreshModSources();
    sourceData = await getModSourcesData();
    mods = await getAvailableMods();
    loaded = true;
  });

  // const gameParam = $derived(route.params.game_name);
  // $effect(() => {
  //   console.log(gameParam);
  //   const activeGameFromParam = toSupportedGame(gameParam);
  //   if (activeGameFromParam) {
  //     activeGame = activeGameFromParam;
  //     getInstalledModsByGame(activeGame).then((val) => {
  //       installedMods = val;
  //     });
  //   }
  // });

  // async function addModFromFile(activeGame: SupportedGame) {
  //   addingMod = true;
  //   addingFromFile = true;
  //   let modArchivePath;
  //   if (onWindows) {
  //     modArchivePath = await filePrompt(["zip"], "ZIP", "Select a mod");
  //   } else {
  //     modArchivePath = await filePrompt(["gz"], "TAR", "Select a mod");
  //   }
  //   if (modArchivePath === null) {
  //     addingMod = false;
  //     addingFromFile = false;
  //     return;
  //   }
  //   // extract the file into install_dir/features/<game>/_local/zip-name
  //   await extractNewMod(activeGame, modArchivePath, "_local");
  //   let modName = await basename(
  //     modArchivePath,
  //     onWindows ? ".zip" : ".tar.gz",
  //   );
  //   if (!onWindows && modName.endsWith(".tar")) {
  //     modName = modName.substring(0, modName.indexOf(".tar"));
  //   }
  //   // install it
  //   navigate("/job/:job_type", {
  //     params: {
  //       job_type: asJobType("installModLocally"),
  //     },
  //     search: {
  //       activeGame: activeGame,
  //       modName: modName,
  //       modSourceName: "_local",
  //       modVersion: "local",
  //       returnTo: route.pathname,
  //     },
  //   });
  // }
</script>

<div class="flex flex-col h-full bg-neutral-900">
  {#if !loaded}
    <div class="flex flex-col h-full justify-center items-center">
      <Spinner color="yellow" size={"12"} />
    </div>
  {:else}
    <div class="pb-20 p-4 overflow-y-auto">
      <div class="flex flex-row items-stretch gap-2 h-10 mb-4">
        <Button
          outline
          class="w-10 rounded text-white hover:text-slate-900 hover:bg-white font-semibold p-2 text-lg"
          onclick={async () => {
            // if (activeGame) {
            //   navigate(`/:game_name/`, { params: { game_name: activeGame } });
            // }
          }}
          aria-label={$_("features_backToGamePage_buttonAlt")}
        >
          <IconArrowLeft />
        </Button>
        <Button
          class="font-semibold text-sm rounded bg-orange-500 border border-orange-400 hover:bg-orange-400 hover:border-orange-300 text-slate-900 hover:text-slate-800 whitespace-nowrap"
          onclick={() => {
            // if (activeGame) {
            //   addModFromFile(activeGame);
            // }
          }}
          aria-label={$_("features_mods_addFromFile_buttonAlt")}
          disabled={addingMod}
        >
          {#if addingFromFile}
            <Spinner class="mr-3" size="4" color="yellow" />
          {/if}
          {$_("features_mods_addFromFile")}</Button
        >
        <Input
          class="font-normal rounded-sm text-gray-200 bg-neutral-800! border border-neutral-600! focus:border-orange-400!"
          placeholder={$_("features_mods_filter_placeholder")}
          bind:value={modFilter}
        />
      </div>
      {#if Object.keys(installedMods).length}
        <h2 class="font-bold mt-2">{$_("features_mods_installed_header")}</h2>
        <!-- <InstalledMods
          {activeGame}
          modList={installedMods}
          {modFilter}
          modSourceData={sourceData}
        ></InstalledMods> -->
      {/if}
      {#if mods}
        <h1 class="font-bold mt-5">{$_("features_mods_available_header")}</h1>
        {#each Object.entries(mods) as [game, gameMods]}
          <div class="py-2">
            <h2>{$_(`gameName_${game}`)}</h2>

            <div id={game} class="grid grid-cols-4 gap-4 mt-2">
              {#each gameMods as mod}
                <ModCard {mod} activeGame={toSupportedGame(game)!} />
              {/each}
            </div>
          </div>
        {/each}
        <!-- <AvailableMods
        {activeGame}
        installedModList={installedMods}
        {modFilter}
        modSourceData={sourceData}
      ></AvailableMods> -->
      {/if}
    </div>
  {/if}
</div>
