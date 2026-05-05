<script lang="ts">
  import {
    doesActiveToolingVersionMeetMinimum,
    setRipCollisionEnabled,
    setRipLevelsEnabled,
    setRipStreamedAudioEnabled,
    setRipTexturesEnabled,
  } from "$lib/rpc/config";
  import { Toggle, Label } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { config } from "/src/state/config.svelte";

  const decompilerSettings = $state(config?.decompilerSettings!);
  let ripLevels: boolean = $state(decompilerSettings?.ripLevelsEnabled!);
  let ripCollision: boolean = $state(decompilerSettings?.ripCollisionEnabled!);
  let ripTextures: boolean = $state(decompilerSettings?.ripTexturesEnabled!);
  let ripAudio: boolean = $state(decompilerSettings?.ripStreamedAudioEnabled!);

  let decompilerOptionsAllowed = true;

  onMount(async () => {
    decompilerOptionsAllowed = await doesActiveToolingVersionMeetMinimum(
      0,
      2,
      14,
    );
  });
</script>

<!-- TODO - bug if a game is uninstalled and the directory isn't found, it fails -->

<div class="flex flex-col gap-4 mt-2 *:text-gray-200">
  {#if !decompilerOptionsAllowed}
    <p class="text-red-500">{$_("settings_decompiler_toolingVersionTooLow")}</p>
  {:else}
    <Label class="text-gray-200">{$_("settings_decompiler_explanation")}</Label>
    <Toggle
      checked={ripLevels}
      color="orange"
      onchange={async (evt) => {
        await setRipLevelsEnabled(evt.currentTarget.checked);
      }}>{$_("settings_decompiler_ripLevels")}</Toggle
    >
    <Toggle
      checked={ripCollision}
      color="orange"
      onchange={async (evt) => {
        await setRipCollisionEnabled(evt.currentTarget.checked);
      }}>{$_("settings_decompiler_ripCollision")}</Toggle
    >
    <Toggle
      checked={ripTextures}
      color="orange"
      onchange={async (evt) => {
        await setRipTexturesEnabled(evt.currentTarget.checked);
      }}>{$_("settings_decompiler_ripTextures")}</Toggle
    >
    <Toggle
      checked={ripAudio}
      color="orange"
      onchange={async (evt) => {
        await setRipStreamedAudioEnabled(evt.currentTarget.checked);
      }}>{$_("settings_decompiler_ripStreamedAudio")}</Toggle
    >
  {/if}
</div>
