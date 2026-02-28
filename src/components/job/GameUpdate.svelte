<script lang="ts">
  import { getAutoUpdateGames, getInstalledVersion } from "$lib/rpc/config";
  import { Button, Card } from "flowbite-svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { navigate } from "/src/router";
  import { versionState } from "/src/state/VersionState.svelte";
  import { asJobType } from "$lib/job/jobs";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { route } from "/src/router";
  import { toSupportedGame } from "$lib/rpc/bindings/utils/SupportedGame";

  const gameParam = $derived(route.params.game_name);
  let activeGame: SupportedGame | undefined = $state(undefined);
  let installedVersion: String | undefined = $state(undefined);

  $effect(() => {
    (async () => {
      const g = toSupportedGame(gameParam);
      if (g) {
        activeGame = g;
        installedVersion = await getInstalledVersion(activeGame);
      }
    })();
  });

  const dispatch = createEventDispatcher();

  onMount(async () => {
    let shouldAutoUpdate = await getAutoUpdateGames();
    if (shouldAutoUpdate) {
      dispatch("job", {
        type: "updateGame",
      });
    }
  });
</script>

<div class="flex flex-col h-full justify-center items-center">
  <Card size="lg" class="!pt-5 px-8 !pb-5 !bg-[#141414]">
    <h5
      class="mb-2 text-xl font-bold text-gray-900 dark:text-white text-center"
    >
      {$_("gameUpdate_versionMismatch_title")}
    </h5>
    <p class="text-base text-gray-500 dark:text-gray-400 mb-1">
      {$_("gameUpdate_versionMismatch_currentlyInstalled")}...
    </p>
    <ul class="list-disc list-inside mb-2">
      <li>
        {$_("gameUpdate_versionMismatch_version")}:
        <strong>{installedVersion}</strong>
      </li>
    </ul>
    <p class="text-base text-gray-500 dark:text-gray-400 mb-1">
      ...{$_("gameUpdate_versionMismatch_currentlySelected")}
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
          navigate("/job/:job_type", {
            params: {
              job_type: asJobType("updateGame"),
            },
            search: {
              activeGame: `${activeGame}`,
              returnTo: `/${activeGame}`,
            } as any,
          });
        }}>{$_("gameUpdate_versionMismatch_button_updateGame")}</Button
      >
      <Button
        class="border-solid border-2 border-slate-500 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        onclick={async () => {
          navigate(`/settings/:tab`, {
            params: { tab: "versions" },
          });
        }}>{$_("gameUpdate_versionMismatch_button_changeVersion")}</Button
      >
    </div>
  </Card>
</div>
