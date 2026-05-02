<script lang="ts">
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { Button, Input, Spinner } from "flowbite-svelte";
  // import IconArrowLeft from "~icons/mdi/arrow-left";
  import { refreshModSources, getAvailableMods } from "$lib/rpc/cache";
  import { navigate, route } from "/src/router";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { toSupportedGame } from "$lib/rpc/SupportedGame";
  import ModCard from "../components/mods/Card.svelte";
  import { platform } from "@tauri-apps/plugin-os";
  import { filePrompt } from "$lib/utils/file-dialogs";
  import { extractNewMod } from "$lib/rpc/features";
  import { basename } from "@tauri-apps/api/path";
  import { asJobType } from "$lib/job/jobs";
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import type { AvailableModsByGame } from "$lib/rpc/bindings/AvailableModsByGame";

  let loaded = $state(false);
  let mods = $state<AvailableModsByGame | undefined>();
  let query = $state("");
  let addingMod = $state(false);
  let addingFromFile = $state(false);

  const activeGame = $derived.by(() => {
    return toSupportedGame(route.params.game_name);
  });

  let game = $state<SupportedGame | "all">(activeGame || "all");
  let sort = $state<"name" | "popularity" | "release" | "author" | "updated">(
    "popularity",
  );

  // TODO: this is gross
  const filteredMods = $derived.by(() => {
    if (!mods) return {};

    const entries =
      game === "all" ? Object.entries(mods) : [[game, mods[game] ?? []]];

    return Object.fromEntries(
      entries.map(([gameKey, mods]) => {
        const search = query.toLowerCase().trim();

        const processed = mods
          .filter(
            (m: ModInfo) =>
              !search ||
              m.displayName.toLowerCase().includes(search) ||
              m.authors?.some((a) => a.toLowerCase().includes(search)),
          )
          .toSorted((a: ModInfo, b: ModInfo) => {
            switch (sort) {
              case "name":
                return a.displayName.localeCompare(b.displayName);
              case "author":
                return (a.authors?.[0] ?? "").localeCompare(
                  b.authors?.[0] ?? "",
                );
              case "release":
                return (
                  Date.parse(b.perGameConfig?.[gameKey]?.releaseDate ?? "0") -
                  Date.parse(a.perGameConfig?.[gameKey]?.releaseDate ?? "0")
                );
              case "popularity":
                return b.downloadCount - a.downloadCount;
              case "updated":
                return (
                  Date.parse(b.versions?.[0]?.publishedDate ?? "0") -
                  Date.parse(a.versions?.[0]?.publishedDate ?? "0")
                );
              default:
                return a.displayName.localeCompare(b.displayName);
            }
          });

        return [gameKey, processed];
      }),
    );
  });

  const installedMods: ModInfo[] = $derived.by(() => {
    if (!filteredMods) return [];

    return Object.entries(filteredMods).flatMap(([game, mods]) =>
      (mods as ModInfo[])
        .filter((mod) => mod.installed)
        .map((mod) => ({
          ...mod,
          game: game as SupportedGame,
        })),
    );
  });

  const isModsEmpty = $derived(() => {
    if (!filteredMods) return true;
    return !Object.values(filteredMods).some((mods) => mods.length > 0);
  });

  const onWindows = platform() === "windows";

  onMount(async () => {
    await refreshModSources();
    mods = await getAvailableMods();
    loaded = true;
  });

  async function addModFromFile() {
    addingMod = true;
    addingFromFile = true;
    let modArchivePath;

    if (!activeGame) return;

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
        returnTo: route.pathname,
      },
    });
  }
</script>

<div class="flex flex-col h-full bg-neutral-900">
  {#if !loaded}
    <div class="flex flex-col h-full justify-center items-center">
      <Spinner color="yellow" size={"12"} />
    </div>
  {:else}
    <div class="pb-20 p-4 overflow-y-auto">
      <div class="flex flex-row items-stretch gap-2 h-10 mb-4">
        <!-- <Button
          outline
          class="w-10 rounded text-white hover:text-slate-900 hover:bg-white font-semibold p-2 text-lg"
          onclick={() => navigate(-1)}
          aria-label={$_("features_backToGamePage_buttonAlt")}
        >
          <IconArrowLeft />
        </Button> -->
        <!-- TODO: Implement add mod from file for the generic mod route (no activeGame) -->
        {#if activeGame}
          <Button
            class="font-semibold text-sm rounded bg-orange-500 border border-orange-400 hover:bg-orange-400 hover:border-orange-300 text-slate-900 hover:text-slate-800 whitespace-nowrap"
            onclick={() => addModFromFile()}
            aria-label={$_("features_mods_addFromFile_buttonAlt")}
            disabled={addingMod}
          >
            {#if addingFromFile}
              <Spinner class="mr-3" size="4" color="yellow" />
            {/if}
            {$_("features_mods_addFromFile")}</Button
          >
        {/if}
        <Input
          class="font-normal rounded-sm text-gray-200 bg-neutral-800! border border-neutral-600! focus:border-orange-400!"
          placeholder={$_("features_mods_filter_placeholder")}
          bind:value={query}
        />

        {#if !activeGame}
          <select
            class="w-56 font-normal text-sm rounded-sm text-gray-200 bg-neutral-800 border border-neutral-600 cursor-pointer"
            bind:value={game}
          >
            <option value="all">{$_("features_mods_all_games")}</option>
            <option value="jak1">{$_("gameName_jak1")}</option>
            <option value="jak2">{$_("gameName_jak2")}</option>
            <option value="jak3">{$_("gameName_jak3")}</option>
          </select>
        {/if}

        <select
          class="w-56 font-normal text-sm rounded-sm text-gray-200 bg-neutral-800 border border-neutral-600 cursor-pointer"
          bind:value={sort}
        >
          <option value="author">{$_("features_mods_authors")}</option>
          <option value="name">{$_("features_mods_name")}</option>
          <option value="release">{$_("features_mods_release_date")}</option>
          <option value="popularity">{$_("features_mods_popularity")}</option>
          <option value="updated">{$_("features_mods_last_updated")}</option>
        </select>
      </div>

      {#if isModsEmpty()}
        <div
          class="flex flex-col items-center justify-center gap-4 p-4 border rounded-md border-zinc-700/60 bg-zinc-900/60 text-center"
        >
          <p class="text-slate-400 font-bold">
            {$_("features_mods_no_sources")}
          </p>
          <Button
            class="border-solid rounded-sm bg-orange-400 hover:bg-orange-600 text-sm text-slate-900 font-semibold px-5 py-2 mt-2"
            href="/settings/mod">{$_("features_mods_go_to_settings")}</Button
          >
        </div>
      {/if}

      {#if Object.keys(installedMods).length}
        <h2 class="font-bold mt-2">{$_("features_mods_installed_header")}</h2>
        <div class="grid grid-cols-2 gap-6 mt-2">
          {#each installedMods as mod}
            <!-- TODO: this part here is rough and i need to fix it, but it works... -->
            <ModCard {mod} activeGame={toSupportedGame(mod.game)!} />
          {/each}
        </div>
      {/if}
      {#if !isModsEmpty()}
        <h1 hidden={!activeGame} class="font-bold mt-5">
          {$_("features_mods_available_header")}
        </h1>
        {#each Object.entries(filteredMods) as [game, gameMods]}
          <div hidden={gameMods.length === 0} class="py-2">
            <h2 hidden={activeGame !== undefined}>
              {$_(`gameName_${game}`)}
            </h2>

            <div id={game} class="grid grid-cols-2 gap-6 mt-2">
              {#each gameMods as mod}
                {#if !mod.installed}
                  <ModCard {mod} activeGame={toSupportedGame(game)!} />
                {/if}
              {/each}
            </div>
          </div>
        {/each}
      {/if}
    </div>
  {/if}
</div>
