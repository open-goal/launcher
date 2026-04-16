<script lang="ts">
  import {
    getAutoUpdateGames,
    getInstallationDirectory,
    getInstalledVersion,
    saveActiveVersionChange,
  } from "$lib/rpc/config";
  import { Button, Card } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { navigate, route } from "/src/router";
  import { versionState } from "/src/state/VersionState.svelte";
  import { asJobType } from "$lib/job/jobs";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";
  import { exists } from "@tauri-apps/plugin-fs";
  import { join } from "@tauri-apps/api/path";

  const gameParam = $derived(route.params.game_name);
  let activeGame: SupportedGame | undefined = $state(undefined);
  let installedVersion: String | undefined = $state(undefined);
  let isoDataExists = $state(false);

  $effect(() => {
    (async () => {
      const g = toSupportedGame(gameParam);
      if (g) {
        activeGame = g;
        installedVersion = await getInstalledVersion(activeGame);
      }
    })();
  });

  onMount(async () => {
    const installDir = await getInstallationDirectory();
    if (!installDir) return;
    if (!activeGame) return;

    const isoDataDir = await join(
      installDir,
      "active",
      activeGame,
      "data",
      "iso_data",
    );
    isoDataExists = await exists(isoDataDir);
    if (!isoDataExists) return;
    await autoUpdate();
  });

  async function autoUpdate() {
    const shouldAutoUpdate = await getAutoUpdateGames();
    if (!shouldAutoUpdate) return;

    navigate("/job/:job_type", {
      params: {
        job_type: asJobType("updateGame"),
      },
      search: {
        activeGame: `${activeGame}`,
        returnTo: `/${activeGame}`,
      } as any,
    });
  }

  async function handleUpdate() {
    if (!activeGame) return;
    if (!isoDataExists) {
      // user deleted their iso_data folder
      // prevent them from pressing the update button and experiencing: https://github.com/open-goal/launcher/issues/638
      navigate("/:game_name/setup", { params: { game_name: activeGame } });
      return;
    }

    navigate("/job/:job_type", {
      params: {
        job_type: asJobType("updateGame"),
      },
      search: {
        activeGame: `${activeGame}`,
        returnTo: `/${activeGame}`,
      } as any,
    });
  }
</script>

<div class="flex flex-col h-full justify-center items-center">
  <Card size="lg" class="pt-5! px-8 pb-5! bg-[#141414]!">
    <h5
      class="mb-2 text-xl font-bold text-gray-900 dark:text-white text-center"
    >
      {$_("gameUpdate_versionMismatch_title")}
    </h5>
    <p class="text-base text-gray-500 dark:text-gray-400 mb-1">
      {$_("gameUpdate_versionMismatch_currentlyInstalled")}:
    </p>
    <ul class="list-disc list-inside mb-2">
      <li>
        {$_("gameUpdate_versionMismatch_version")}:
        <strong>{installedVersion}</strong>
      </li>
    </ul>
    <p class="text-base text-gray-500 dark:text-gray-400 mb-1">
      {$_("gameUpdate_versionMismatch_currentlySelected")}
    </p>
    <ul class="list-disc list-inside mb-5">
      <li>
        {$_("gameUpdate_versionMismatch_version")}:
        <strong>{versionState.activeToolingVersion}</strong>
      </li>
    </ul>
    <p class="mb-3">
      {$_("gameUpdate_versionMismatch_nextSteps")}
    </p>
    <div
      class="justify-center items-center space-y-4 sm:flex sm:space-y-0 sm:space-x-4"
    >
      <Button
        class="border-solid border-2 border-slate-500 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          await handleUpdate();
        }}>{$_("gameUpdate_versionMismatch_button_updateGame")}</Button
      >
      <Button
        class="border-solid border-2 border-slate-500 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          const error = await saveActiveVersionChange(installedVersion!);
          if (!error) {
            navigate("/:game_name/", { params: { game_name: activeGame! } });
          }
        }}>{$_("gameUpdate_versionMismatch_button_changeVersion")}</Button
      >
    </div>
  </Card>
</div>
