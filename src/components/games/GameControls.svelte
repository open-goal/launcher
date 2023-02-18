<script type="ts">
  import { launcherConfig } from "$lib/config";
  import { getGameTitle, getInternalName, SupportedGame } from "$lib/constants";
  import { launchGameInDebug } from "$lib/launch";
  import { openDir, openREPL } from "$lib/rpc/commands";
  import { uninstallGame } from "$lib/setup/setup";
  import Icon from "@iconify/svelte";
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
  import { launchGame } from "$lib/rpc/game";

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
    launchGame("jak1");
  }

  async function onClickOpenREPL() {
    await openREPL();
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
    // await decompileGameData(isoPath);
  }

  async function onClickCompile() {
    ProcessLogs.update(() => "");
    const isoPath = await join(
      await appDir(),
      "data",
      "iso_data",
      getInternalName(activeGame)
    );
    // await compileGame(isoPath);
  }
</script>

<!-- TODO - texture replacements left out for now, get everything else working end-to-end first -->
<!-- TOOO - time played -->

<div class="flex flex-col justify-end items-end mt-auto">
  <h1
    class="tracking-tighter text-2xl font-bold pb-3 text-orange-500 text-outline pointer-events-none"
  >
    {getGameTitle(activeGame)}
  </h1>
  <div class="flex flex-row gap-2">
    <Button
      btnClass="border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800 text-sm text-white font-semibold px-5 py-2"
      on:click={onClickPlay}
      disabled={$isDecompiling || $isCompiling}>Play</Button
    >
    <Button
      btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-5 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      ><Chevron placement="top">Features</Chevron></Button
    >
    <Dropdown placement="top">
      <DropdownItem>Dashboard</DropdownItem>
      <DropdownDivider />
      <DropdownItem>Settings</DropdownItem>
      <DropdownItem>Earnings</DropdownItem>
      <DropdownItem slot="footer">Separated link</DropdownItem>
    </Dropdown>
    <Button
      btnClass="text-center font-semibold focus:ring-0 focus:outline-none inline-flex items-center justify-center px-2 py-2 text-sm text-white border-solid border-2 border-slate-900 rounded bg-slate-900 hover:bg-slate-800"
      >
        <Icon icon="material-symbols:settings" width={24} height={24} />
      </Button
    >
    <Dropdown placement="top-end" frameClass="mr-1">
      <DropdownItem>Dashboard</DropdownItem>
      <DropdownDivider />
      <DropdownItem>Settings</DropdownItem>
      <DropdownItem>Earnings</DropdownItem>
      <DropdownItem slot="footer">Separated link</DropdownItem>
    </Dropdown>
  </div>
</div>

<!-- {#if componentLoaded}
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
      {#if !($isDecompiling || $isCompiling)}
        <DropdownItem href="#" on:click={onClickBootDebug}
          >Boot In Debug</DropdownItem
        >
      {/if}
      <DropdownItem href="#" on:click={onClickOpenREPL}>Open REPL</DropdownItem>
      <DropdownDivider />
      <!-- NOTE: Wrapped these two dropdown items in a tags for the use:link, otherwise the dropdownitem doesnt support it -->
<!-- <a use:link href="/textures"><DropdownItem>Texture Packs</DropdownItem></a
      >
      <!-- <a use:link href="/mods">
        <DropdownItem>Mods</DropdownItem>
      </a> -->
<!-- </Dropdown>

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
      {#if !($isDecompiling || $isCompiling)}
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
      {/if}
    </Dropdown>
  </ButtonGroup>
{/if} -->
