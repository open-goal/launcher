<script lang="ts">
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { Indicator, Tooltip } from "flowbite-svelte";
  import IconGlobe from "~icons/mdi/globe";
  import { platform } from "@tauri-apps/plugin-os";
  import { isLatestVersionOfModSupportedOnCurrentPlatform } from "$lib/features/mods";
  import { navigate } from "/src/router";
  import { _ } from "svelte-i18n";

  let {
    activeGame,
    modInternalName,
    modDisplayName,
    modSourceName,
    modInfo,
    showNewIndicator,
    isInstalled,
    thumbnailUrl,
    href,
  }: {
    activeGame: SupportedGame;
    modInternalName: string;
    modDisplayName: string;
    modSourceName: string;
    modInfo: ModInfo | undefined;
    showNewIndicator: boolean;
    isInstalled: boolean;
    thumbnailUrl: string;
    href: string | null;
  } = $props();

  const userPlatform = platform();
</script>

{#if href !== null}
  <a
    {href}
    target="_blank"
    rel="noreferrer noopener"
    class="h-[200px] max-w-[160px] bg-cover p-1 flex justify-center items-end relative grayscale"
    style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{thumbnailUrl}'); background-size: cover;"
  >
    <h3 class="text-outline">
      {modDisplayName}
    </h3>
    <div class="absolute top-0 right-0 m-2 flex gap-1">
      <IconGlobe />
      <Tooltip placement="bottom">{modSourceName}</Tooltip>
    </div>
  </a>
{:else}
  <button
    disabled={!isInstalled &&
      !isLatestVersionOfModSupportedOnCurrentPlatform(userPlatform, modInfo)}
    class="h-[200px] max-w-[160px] bg-cover p-1 flex justify-center items-end relative"
    style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{thumbnailUrl}'); background-size: cover;"
    onclick={async () => {
      navigate(`/:game_name/mods/:source_name/:mod_name`, {
        params: {
          game_name: activeGame,
          source_name: encodeURI(modSourceName),
          mod_name: encodeURI(modInternalName),
        },
      });
    }}
  >
    <h3 class="text-outline">
      {modDisplayName}
    </h3>
    <div class="absolute top-0 right-0 m-2 flex gap-1">
      <IconGlobe />
      <Tooltip placement="bottom">{modSourceName}</Tooltip>
    </div>
    {#if !isInstalled && showNewIndicator}
      <Indicator color="green" border size="xl" placement="top-right">
        <span class="text-white text-xs font-bold">!</span>
      </Indicator>
    {/if}
  </button>
  {#if !isInstalled && !isLatestVersionOfModSupportedOnCurrentPlatform(userPlatform, modInfo)}
    <Tooltip placement="top"
      >{$_("features_mods_not_supported_platform_1")} ({userPlatform})<br />{$_(
        "features_mods_not_supported_platform_2",
      )}</Tooltip
    >
  {/if}
{/if}
