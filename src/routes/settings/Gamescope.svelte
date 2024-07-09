<script lang="ts">
  import {
    isGameScopeInstalled,
    setGamescopeEnabled,
    isGamescopeEnabled,
    isMangoHudEnabled,
    setMangoHudEnabled,
    setWindowHeight,
    setWindowWidth,
    getWindowHeight,
    getWindowWidth,
    getWindowType,
    setWindowType,
    setUpscaleEnabled,
    isUpscaleEnabled,
    getUpscaleHeight,
    getUpscaleWidth,
    setUpscaleHeight,
    setUpscaleWidth,
    getUpscaleMethod,
    setUpscaleMethod,
    setHDREnabled,
    isHDREnabled,
    getGamescopeBinary,
    setGamescopeBinary,
  } from "$lib/rpc/config";
  import {
    Label,
    Input,
    Toggle,
    Select,
    Helper
  } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { invoke } from '@tauri-apps/api/tauri';
  import Settings from "../Settings.svelte";
  import { toastStore } from "$lib/stores/ToastStore";

  let windowTypes = [
    { value: true, name: 'Fullscreen'},
    { value: false, name: 'Borderless'},
  ]

  let upscaleMethods = [
    { value: "fsr", name: 'AMD FidelityFX™ Super Resolution 1.0 (FSR)'},
    { value: "nis", name: 'NVIDIA Image Scaling v1.0.3'},
    { value: "linear", name: 'Linear'},
    { value: "nearest", name: 'Nearest'}
  ]

  let windowHeight: string = '';
  let windowWidth: string = '';
  let upscaleWidth: string = '';
  let upscaleHeight: string = '';
  let gamescopeEnabled: Boolean = false;
  let fullscreen: boolean = false;
  let enableMangoHud: Boolean = false;
  let upscaleMethod: String = "fsr";
  let enableUpscale: boolean = false;
  let hdr: boolean = false;
  let gamescopeBinary: string = '';
  let isGamescopeFound = false;

  $: windowHeight = windowHeight.replace(/[^0-9]/g, '')
  $: windowWidth = windowWidth.replace(/[^0-9]/g, '')

  onMount(async () => {
    try {
      await checkAndSetGamescopeBinary();
      if (isGamescopeFound) {
        gamescopeEnabled = await isGamescopeEnabled();
        enableMangoHud = await isMangoHudEnabled();
        windowHeight = await getWindowHeight();
        windowWidth = await getWindowWidth();
        fullscreen = await getWindowType();
        enableUpscale = await isUpscaleEnabled();
        upscaleMethod = await getUpscaleMethod();
        upscaleWidth = await getUpscaleWidth();
        upscaleHeight = await getUpscaleHeight();
        hdr = await isHDREnabled();
      }
    } catch (error) {
      toastStore.makeToast("Failed to verify Gamescope install", "error"); 
    }
  });
  
  async function checkAndSetGamescopeBinary() {
    try {
      let binaryLocation = await getGamescopeBinary();
      isGamescopeFound = !!binaryLocation;

      if (!isGamescopeFound) {
        binaryLocation = await isGameScopeInstalled();
        isGamescopeFound = !!binaryLocation;
        if (isGamescopeFound) {
          toastStore.makeToast("Gamescope found!", "info");
          gamescopeBinary = binaryLocation;
          await setGamescopeBinary(gamescopeBinary);
        } else {
          toastStore.makeToast("Gamescope was not found. Make sure it is installed", "error"); 
        }
      }
    } catch (error) {
      toastStore.makeToast("Error setting gamescope binary", "error");
    }
  }

</script>
{#if isGamescopeFound}
<div class="flex flex-col gap-4 mt-2">
  <div class="gap-2">
    <Toggle
      checked={gamescopeEnabled}
      color="orange"
      on:change={async (evt) => {
        await setGamescopeEnabled(evt.target.checked);
        gamescopeEnabled = await isGamescopeEnabled();
      }}
    >
      {$_("settings_gamescope_enableGamescope")}
    </Toggle>
  </div>
  
  {#if gamescopeEnabled}
    <div class="gap-2">
      <Toggle
        checked={enableUpscale}
        color="orange"
        on:change={async (evt) => {
          await setUpscaleEnabled(evt.target.checked);
          enableUpscale = await isUpscaleEnabled();
        }}
      >
        {$_("settings_gamescope_enableUpscale")}
      </Toggle>
    </div>
    <div class="flex flex-row gap-2">
      <div class="flex flex-col gap-2">
        <Label for="window-width" class="block mb-2">
          {$_("settings_gamescope_windowWidth")}
        </Label>
        <Input
          id="window-width"
          bind:value={windowWidth}
          on:input={async () => {
            await setWindowWidth(windowWidth);
            windowWidth = await getWindowWidth();
          }}
        />
      </div>
      <div class="flex flex-col gap-2">
        <Label for="window-height" class="block mb-2">
          {$_("settings_gamescope_windowHeight")}
        </Label>
        <Input
          id="window-height"
          bind:value={windowHeight}
          on:input={async () => {
            await setWindowHeight(windowHeight);
            windowHeight = await getWindowHeight();
          }}
        />
      </div>
    </div>

    {#if enableUpscale}
      <div class="flex flex-row gap-2">
        <div class="flex flex-col gap-2">
          <Label for="upscale-width" class="block mb-2">
            {$_("settings_gamescope_upscaleWidth")}
          </Label>
          <Input
            id="upscale-width"
            bind:value={upscaleWidth}
            on:input={async () => {
              await setUpscaleWidth(upscaleWidth);
              upscaleWidth = await getUpscaleWidth();
            }}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="upscale-height" class="block mb-2">
            {$_("settings_gamescope_upscaleHeight")}
          </Label>
          <Input
            id="upscale-height"
            bind:value={upscaleHeight}
            on:input={async () => {
              await setUpscaleHeight(upscaleHeight);
              upscaleHeight = await getUpscaleHeight();
            }}
          />
        </div>
      </div>

      <div class="flex flex-col gap-2">
        <Label for="upscale-type" class="block mb-2">
          {$_("settings_gamescope_upscaleType")}
        </Label>
        <Select
          class="p-3"
          items={upscaleMethods}
          id="window-type"
          bind:value={upscaleMethod}
          on:change={async () => {
            await setUpscaleMethod(upscaleMethod);
            upscaleMethod = await getUpscaleMethod();
          }}
        />
      </div>
    {/if}

    <div class="flex flex-col gap-2">
      <Label for="window-type" class="block mb-2">
        {$_("settings_gamescope_windowType")}
      </Label>
      <Select
        class="p-3"
        items={windowTypes}
        id="window-type"
        bind:value={fullscreen}
        on:change={async () => {
          await setWindowType(fullscreen);
          fullscreen = await getWindowType();
        }}
      />
    </div>
    
    <div class="gap-2">
      <Toggle
        checked={hdr}
        color="orange"
        on:change={async (evt) => {
          await setHDREnabled(evt.target.checked);
          hdr = await isHDREnabled();
        }}
      >
        {$_("settings_gamescope_enableHDR")}
      </Toggle>
    </div>
    
    <div class="gap-2">
      <Toggle
        checked={enableMangoHud}
        color="orange"
        on:change={async (evt) => {
          await setMangoHudEnabled(evt.target.checked);
          enableMangoHud = await isMangoHudEnabled();
        }}
      >
        {$_("settings_gamescope_enableMangoHud")}
      </Toggle>
      <Helper class="text-xs mt-2 italic">
        {$_("settings_gamescope_enableMangoHud_helper")}
      </Helper>
    </div>
    <Helper class="text-xs mt-2 italic">
      {$_("settings_gamescope_saving_helper")}
    </Helper>
  {/if}
</div>
{/if}


