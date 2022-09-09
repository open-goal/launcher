<script type="ts">
  import { launcherConfig } from "$lib/config";
  import { getInternalName, SupportedGame } from "$lib/constants";
  import { launchGame } from "$lib/launch";
  import { openDir } from "$lib/rpc/commands";
  import { compileGame, decompileGameData } from "$lib/setup/setup";
  import { appDir, configDir, join } from "@tauri-apps/api/path";
  import { createEventDispatcher, onMount } from "svelte";
  import LogViewer from "./setup/LogViewer.svelte";
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

  async function onClickUninstall() {
    const confirmed = await confirm("Are you sure you want to uninstall?");
    if (confirmed) {
      await launcherConfig.setInstallStatus(activeGame, false);
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

<!-- add larger font to the play button -->
{#if componentLoaded}
  <ButtonGroup>
    <Button
      class="w-56"
      color="dark"
      on:click={onClickPlay}
      disabled={$isDecompiling || $isCompiling}>Play</Button
    >

    <Button disabled={$isDecompiling || $isCompiling}
      ><Chevron placement="top">Extras</Chevron></Button
    >
    <Dropdown placement="top">
      <DropdownItem href="#">Boot In Debug</DropdownItem>
      <DropdownItem href="#">Open REPL</DropdownItem>
      <DropdownDivider />
      <!-- NOTE: Wrapped these two dropdown items in a tags for the use:link, otherwise the dropdownitem doesnt support it -->
      <a use:link href="/textures"><DropdownItem>Texture Packs</DropdownItem></a
      >
      <a use:link href="/mods">
        <DropdownItem>Mods</DropdownItem>
      </a>
    </Dropdown>

    <Button disabled={$isDecompiling || $isCompiling}
      ><Chevron placement="top"><i class="fa fa-cog" /></Chevron></Button
    >
    <Dropdown placement="top">
      <DropdownItem href="#" on:click={() => openDir(configPath)}
        >Open Settings & Saves</DropdownItem
      >
      <DropdownItem href="#" on:click={() => openDir(screenshotsPath)}
        >Open Screenshots Directory</DropdownItem
      >
      <DropdownDivider />
      <DropdownItem href="#" on:click={() => onClickDecompile()}
        >Decompile</DropdownItem
      >
      <DropdownItem href="#" on:click={() => onClickCompile()}
        >Compile</DropdownItem
      >
      <DropdownDivider />
      <DropdownItem href="#" color="red" on:click={() => onClickUninstall()}
        >Uninstall</DropdownItem
      >
    </Dropdown>
  </ButtonGroup>
  {#if $isDecompiling || $isCompiling}
    <Spinner />
  {/if}
  {#if $ProcessLogs}
    <LogViewer />
  {/if}
{/if}
