<script lang="ts">
  import type { SupportedGame } from "$lib/constants";
  import { Button, Card } from "flowbite-svelte";
  import { createEventDispatcher } from "svelte";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();

  export let installedVersion: String;
  export let installedVersionFolder: String;
  export let activeVersion: String;
  export let activeVersionFolder: String;
</script>

<div class="flex flex-col h-full justify-center items-center">
  <Card size="lg" padding="xl">
    <h5
      class="mb-2 text-xl font-bold text-gray-900 dark:text-white text-center"
    >
      Version Mismatch Detected!
    </h5>
    <p class="text-base text-gray-500 dark:text-gray-400 mb-1">
      The game is already installed with...
    </p>
    <ul class="list-disc list-inside mb-2">
      <li>Version: <strong>{installedVersion}</strong></li>
      <li>Type: <strong>{installedVersionFolder}</strong></li>
    </ul>
    <p class="text-base text-gray-500 dark:text-gray-400 mb-1">
      ...but you currently have selected
    </p>
    <ul class="list-disc list-inside mb-5">
      <li>Version: <strong>{activeVersion}</strong></li>
      <li>Type: <strong>{activeVersionFolder}</strong></li>
    </ul>
    <p class="mb-3">
      You can either update the game to this new version (no save data will be
      lost) or you can rollback your active version to match
    </p>
    <div
      class="justify-center items-center space-y-4 sm:flex sm:space-y-0 sm:space-x-4"
    >
      <Button
        btnClass="border-solid border-2 border-slate-500 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        on:click={async () => {
          dispatch("job", {
            type: "updateGame",
          });
        }}>Update Game</Button
      >
      <Button
        btnClass="border-solid border-2 border-slate-500 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
        href="/settings/versions">Change Version</Button
      >
    </div>
  </Card>
</div>
