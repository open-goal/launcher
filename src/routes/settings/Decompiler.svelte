<script lang="ts">
  import {
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

  onMount(async () => {
    ripLevels = await isRipLevelsEnabled();
    ripCollision = await isRipCollisionEnabled();
    ripTextures = await isRipTexturesEnabled();
    ripStreamedAudio = await isRipStreamedAudioEnabled();
  });
</script>

<div class="flex flex-col gap-4 mt-2">
  <Toggle
    checked={ripLevels}
    color="orange"
    on:change={async (evt) => {
      await setRipLevelsEnabled(evt.target.checked);
      ripLevels = await isRipLevelsEnabled();
    }}>Rip Levels (.glb files)</Toggle
  >
  <Toggle
    checked={ripCollision}
    color="orange"
    on:change={async (evt) => {
      await setRipCollisionEnabled(evt.target.checked);
      ripCollision = await isRipCollisionEnabled();
    }}>Rip Collision (.obj files)</Toggle
  >
  <Toggle
    checked={ripTextures}
    color="orange"
    on:change={async (evt) => {
      await setRipTexturesEnabled(evt.target.checked);
      ripTextures = await isRipTexturesEnabled();
    }}>Rip Textures (.png files)</Toggle
  >
  <Toggle
    checked={ripStreamedAudio}
    color="orange"
    on:change={async (evt) => {
      await setRipStreamedAudioEnabled(evt.target.checked);
      ripStreamedAudio = await isRipStreamedAudioEnabled();
    }}>Rip Streamed Audio (.wav files)</Toggle
  >
</div>
