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
  import { Toggle } from "flowbite-svelte";
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

<div class="flex flex-col gap-4 mt-2">
  {#if !decompilerOptionsAllowed}
    <p class="text-red-500">{$_("settings_decompiler_toolingVersionTooLow")}</p>
  {:else}
    <Toggle
      checked={ripLevels}
      color="orange"
      on:change={async (evt) => {
        await setRipLevelsEnabled(evt.target.checked);
        ripLevels = await isRipLevelsEnabled();
      }}>{$_("settings_decompiler_ripLevels")}</Toggle
    >
    <Toggle
      checked={ripCollision}
      color="orange"
      on:change={async (evt) => {
        await setRipCollisionEnabled(evt.target.checked);
        ripCollision = await isRipCollisionEnabled();
      }}>{$_("settings_decompiler_ripCollision")}</Toggle
    >
    <Toggle
      checked={ripTextures}
      color="orange"
      on:change={async (evt) => {
        await setRipTexturesEnabled(evt.target.checked);
        ripTextures = await isRipTexturesEnabled();
      }}>{$_("settings_decompiler_ripTextures")}</Toggle
    >
    <Toggle
      checked={ripStreamedAudio}
      color="orange"
      on:change={async (evt) => {
        await setRipStreamedAudioEnabled(evt.target.checked);
        ripStreamedAudio = await isRipStreamedAudioEnabled();
      }}>{$_("settings_decompiler_ripStreamedAudio")}</Toggle
    >
  {/if}
</div>
