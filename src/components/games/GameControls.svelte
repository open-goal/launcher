<script type="ts">
  import { launcherConfig } from "$lib/config";
  import { getInternalName, SupportedGame } from "$lib/constants";
  import { launchGame, launchGameInDebug } from "$lib/launch";
  import { openDir } from "$lib/rpc/commands";
  import {
    compileGame,
    decompileGameData,
    uninstallGame,
  } from "$lib/setup/setup";
  import { appDir, configDir, join } from "@tauri-apps/api/path";
  import { createEventDispatcher, onMount } from "svelte";
  import {
    isCompiling,
    isDecompiling,
    ProcessLogs,
  } from "$lib/stores/AppStore";
  import { link } from "svelte-navigator";

  import {
    Button,
    ButtonGroup,
    Chevron,
    Dropdown,
    DropdownItem,
    DropdownDivider,
    Spinner,
  } from "flowbite-svelte";

  export let activeGame: SupportedGame;

  const dispatch = createEventDispatcher();
  let componentLoaded = false;
  let configPath = undefined;
  let screenshotsPath = undefined;

  onMount(async () => {
    configPath = await join(
      await configDir(),
      "OpenGOAL",
      getInternalName(activeGame)
    );

    screenshotsPath = await join(
      await configDir(),
      "OpenGOAL-Launcher/data/screenshots"
    );
    componentLoaded = true;
  });

  function onClickPlay() {
    launchGame();
  }

  function onClickBootDebug() {
    launchGameInDebug();
  }

  async function onClickUninstall() {
    const confirmed = await confirm("Are you sure you want to uninstall?");
    if (confirmed) {
      await launcherConfig.setInstallStatus(activeGame, false);
      await uninstallGame(activeGame);
      dispatch("change");
    }
  }

  async function onClickDecompile() {
    ProcessLogs.update(() => "");
    const isoPath = await join(
      await appDir(),
      "data",
      "iso_data",
      getInternalName(activeGame)
    );
    await decompileGameData(isoPath);
  }

  async function onClickCompile() {
    ProcessLogs.update(() => "");
    const isoPath = await join(
      await appDir(),
      "data",
      "iso_data",
      getInternalName(activeGame)
    );
    await compileGame(isoPath);
  }
</script>

{#if componentLoaded}
  <ButtonGroup>
    {#if $isDecompiling || $isCompiling}
      <Spinner class="min-h-full mx-2" />
    {/if}
    <Button
      class="w-56 !rounded-none !bg-[#222222] border-none !text-white hover:!text-blue-500 !text-2xl"
      on:click={onClickPlay}
      disabled={$isDecompiling || $isCompiling}>Play</Button
    >

    <Button disabled={$isDecompiling || $isCompiling}
      ><Chevron placement="top">Extras</Chevron></Button
    >
    <Dropdown class="!rounded-none" placement="top">
      <DropdownItem href="#" on:click={onClickBootDebug}
        >Boot In Debug</DropdownItem
      >
      <!-- <DropdownItem href="#">Open REPL</DropdownItem> -->
      <DropdownDivider />
      <!-- NOTE: Wrapped these two dropdown items in a tags for the use:link, otherwise the dropdownitem doesnt support it -->
      <a use:link href="/textures"><DropdownItem>Texture Packs</DropdownItem></a
      >
      <!-- <a use:link href="/mods">
        <DropdownItem>Mods</DropdownItem>
      </a> -->
    </Dropdown>

    <Button class="!rounded-none" disabled={$isDecompiling || $isCompiling}
      ><Chevron placement="top"><i class="fa fa-cog" /></Chevron></Button
    >
    <Dropdown class="!rounded-none" placement="top">
      <DropdownItem href="#" on:click={() => openDir(configPath)}
        >Open Settings & Saves</DropdownItem
      >
      <DropdownItem href="#" on:click={() => openDir(screenshotsPath)}
        >Open Screenshots Directory</DropdownItem
      >
      <DropdownDivider />
      <DropdownItem href="#" on:click={async () => await onClickDecompile()}
        >Decompile</DropdownItem
      >
      <DropdownItem href="#" on:click={async () => await onClickCompile()}
        >Compile</DropdownItem
      >
      <DropdownDivider />
      <DropdownItem href="#" color="red" on:click={() => onClickUninstall()}
        >Uninstall</DropdownItem
      >
    </Dropdown>
  </ButtonGroup>
{/if}
