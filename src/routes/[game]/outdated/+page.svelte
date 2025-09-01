<script lang="ts">
  import { Alert, Button } from "flowbite-svelte";
  import type { PageProps } from "./$types";
  import { _ } from "svelte-i18n";
  import { goto } from "$app/navigation";

  let { data }: PageProps = $props();
  const game = data.game;
  const config = $derived(data.config);
  const activeVersion = $derived(data.config.activeVersion);
  const installedVersion = $derived(config.games[game].version);

  async function updateGame() {
    // TODO: make this actually do something
  }
</script>

<div class="flex flex-col h-screen p-4 justify-center items-center">
  <Alert class="rounded-none">
    <div class="flex items-center justify-center gap-3">
      <span class="text-lg font-medium"
        >{$_("gameUpdate_versionMismatch_title")}</span
      >
    </div>
    <p class="mt-2 mb-4 text-sm">
      {$_("gameUpdate_versionMismatch_currentlyInstalled")}
      {$_("gameUpdate_versionMismatch_version")}:
      <strong>{installedVersion}</strong>
      {$_("gameUpdate_versionMismatch_currentlySelected")}
      {$_("gameUpdate_versionMismatch_version")}:
      <strong>{activeVersion}</strong>
      {$_("gameUpdate_versionMismatch_nextSteps")}
    </p>
    <div class="flex gap-2 justify-center">
      <Button size="md" outline onclick={async () => updateGame()}
        >{$_("gameUpdate_versionMismatch_button_updateGame")}</Button
      >
      <Button
        size="md"
        outline
        onclick={async () => {
          goto("/settings/versions");
        }}>{$_("gameUpdate_versionMismatch_button_changeVersion")}</Button
      >
    </div>
  </Alert>
</div>
