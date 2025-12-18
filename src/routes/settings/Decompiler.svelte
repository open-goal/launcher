<script lang="ts">
  import {
    doesActiveToolingVersionMeetMinimum,
    isRipCollisionEnabled,
    isRipLevelsEnabled,
    isRipStreamedAudioEnabled,
    isRipTexturesEnabled,
    setRipCollisionEnabled,
    setRipLevelsEnabled,
    setRipStreamedAudioEnabled,
    setRipTexturesEnabled,
  } from "$lib/rpc/config";
  import { Toggle, Label } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";

  let ripLevels: boolean = false;
  let ripCollision: boolean = false;
  let ripTextures: boolean = false;
  let ripStreamedAudio: boolean = false;

  let decompilerOptionsAllowed = true;

  onMount(async () => {
    ripLevels = await isRipLevelsEnabled();
    ripCollision = await isRipCollisionEnabled();
    ripTextures = await isRipTexturesEnabled();
    ripStreamedAudio = await isRipStreamedAudioEnabled();

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
        ripLevels = await isRipLevelsEnabled();
      }}>{$_("settings_decompiler_ripLevels")}</Toggle
    >
    <Toggle
      checked={ripCollision}
      color="orange"
      onchange={async (evt) => {
        await setRipCollisionEnabled(evt.currentTarget.checked);
        ripCollision = await isRipCollisionEnabled();
      }}>{$_("settings_decompiler_ripCollision")}</Toggle
    >
    <Toggle
      checked={ripTextures}
      color="orange"
      onchange={async (evt) => {
        await setRipTexturesEnabled(evt.currentTarget.checked);
        ripTextures = await isRipTexturesEnabled();
      }}>{$_("settings_decompiler_ripTextures")}</Toggle
    >
    <Toggle
      checked={ripStreamedAudio}
      color="orange"
      onchange={async (evt) => {
        await setRipStreamedAudioEnabled(evt.currentTarget.checked);
        ripStreamedAudio = await isRipStreamedAudioEnabled();
      }}>{$_("settings_decompiler_ripStreamedAudio")}</Toggle
    >
  {/if}
</div>
