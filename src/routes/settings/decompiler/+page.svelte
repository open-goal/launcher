<script lang="ts">
  import { Label, Toggle } from "flowbite-svelte";
  import type { PageProps } from "./$types";
  import { _ } from "svelte-i18n";
  import {
    setRipCollisionEnabled,
    setRipLevelsEnabled,
    setRipStreamedAudioEnabled,
    setRipTexturesEnabled,
  } from "$lib/rpc/config";

  let { data }: PageProps = $props();
  const allowed = $derived(data.allowed);
  const settings = $derived.by(() => data.config.decompilerSettings);
  let ripCollision = $state(settings.ripCollisionEnabled);
  let ripLevels = $state(settings.ripLevelsEnabled);
  let ripStreamedAudio = $state(settings.ripStreamedAudioEnabled);
  let ripTextures = $state(settings.ripTexturesEnabled);
</script>

<div class="flex flex-col gap-4 mt-2 *:text-gray-200">
  {#if !allowed}
    <p class="text-red-500">{$_("settings_decompiler_toolingVersionTooLow")}</p>
  {:else}
    <Label class="text-gray-200">{$_("settings_decompiler_explanation")}</Label>
    <Toggle
      checked={ripLevels}
      color="orange"
      onchange={async (evt) => {
        await setRipLevelsEnabled(!ripLevels);
      }}>{$_("settings_decompiler_ripLevels")}</Toggle
    >
    <Toggle
      checked={ripCollision}
      color="orange"
      onchange={async (evt) => {
        await setRipCollisionEnabled(!ripCollision);
      }}>{$_("settings_decompiler_ripCollision")}</Toggle
    >
    <Toggle
      checked={ripTextures}
      color="orange"
      onchange={async (evt) => {
        await setRipTexturesEnabled(!ripTextures);
      }}>{$_("settings_decompiler_ripTextures")}</Toggle
    >
    <Toggle
      checked={ripStreamedAudio}
      color="orange"
      onchange={async (evt) => {
        await setRipStreamedAudioEnabled(!ripStreamedAudio);
      }}>{$_("settings_decompiler_ripStreamedAudio")}</Toggle
    >
  {/if}
</div>
