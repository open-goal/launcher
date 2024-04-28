<script lang="ts">
  import type { SupportedGame } from "$lib/constants";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { Button, Card } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";
  import { _ } from "svelte-i18n";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();

  export let installedVersion: String | undefined;
  export let installedVersionFolder: String | undefined;
</script>

<div class="flex flex-col h-full justify-center items-center">
  <Card size="lg" padding="xl">
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
  </Card>
</div>
