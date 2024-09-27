<script lang="ts">
  import type { SupportedGame } from "$lib/constants";
  import { isMinimumVCCRuntimeInstalled } from "$lib/rpc/config";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { type } from "@tauri-apps/api/os";
  import { Button, Card } from "flowbite-svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import { _ } from "svelte-i18n";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();

  export let installedVersion: String | undefined;
  export let installedVersionFolder: String | undefined;

  let displayVCCWarning = false;

  onMount(async () => {
    const osType = await type();
    if (osType == "Windows_NT") {
      const isVCCInstalled = await isMinimumVCCRuntimeInstalled();
      displayVCCWarning = !isVCCInstalled;
    }
  });
</script>

<div class="flex flex-col h-full justify-center items-center">
  <Card size="lg" padding="xl" class="!pt-5 !pb-5 !bg-[#141414]">
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
      <li>
        {$_("gameUpdate_versionMismatch_type")}:
        <strong>{installedVersionFolder}</strong>
      </li>
    </ul>
    <p class="text-base text-gray-500 dark:text-gray-400 mb-1">
      ...{$_("gameUpdate_versionMismatch_currentlySelected")}
    </p>
    <ul class="list-disc list-inside mb-5">
      <li>
        {$_("gameUpdate_versionMismatch_version")}:
        <strong>{$VersionStore.activeVersionName}</strong>
      </li>
      <li>
        {$_("gameUpdate_versionMismatch_type")}:
        <strong>{$VersionStore.activeVersionType}</strong>
      </li>
    </ul>
    {#if displayVCCWarning}
      <span class="font-bold"
        >{$_("requirements_windows_vccRuntimeNotInstalled")}</span
      >
      <p>
        {$_("gameUpdate_windows_vccRuntimeExplanation")}
        <a
          class="font-bold text-blue-500"
          target="_blank"
          rel="noreferrer"
          href="https://aka.ms/vs/17/release/vc_redist.x64.exe"
          >{$_("requirements_windows_vccRuntimeExplanation_downloadLink")}</a
        >
      </p>
    {:else}
      <p class="mb-3">
        {$_("gameUpdate_versionMismatch_nextSteps")}
      </p>
      <div
        class="justify-center items-center space-y-4 sm:flex sm:space-y-0 sm:space-x-4"
      >
        <Button
          class="border-solid border-2 border-slate-500 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
          on:click={async () => {
            dispatch("job", {
              type: "updateGame",
            });
          }}>{$_("gameUpdate_versionMismatch_button_updateGame")}</Button
        >
        <Button
          class="border-solid border-2 border-slate-500 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
          href="/settings/versions"
          >{$_("gameUpdate_versionMismatch_button_changeVersion")}</Button
        >
      </div>
    {/if}
  </Card>
</div>
