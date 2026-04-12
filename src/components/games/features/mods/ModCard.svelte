<script lang="ts">
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import AlertCircleOutline from "~icons/mdi/alert-circle-outline";
  import { navigate } from "/src/router";

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
</script>

{#if href !== null}
  <a
    {href}
    target="_blank"
    rel="noreferrer noopener"
    class="h-50 max-w-40 bg-cover p-1 flex justify-center items-end relative grayscale"
    style="background: linear-gradient(to bottom, rgba(0, 0, 0, 0), rgba(0, 0, 0, 0.6)), url('{thumbnailUrl}'); background-size: cover;"
  >
    <h3 class="text-outline">
      {modDisplayName}
    </h3>
  </a>
{:else}
  <button
    class="h-50 max-w-40 bg-cover p-1 flex justify-center items-end relative"
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
    <div class="absolute top-0 right-0 m-1 flex gap-1">
      {#if !isInstalled && showNewIndicator}
        <AlertCircleOutline
          class="text-green-400 text-lg bg-neutral-800 rounded border border-neutral-700"
        />
      {/if}
    </div>
  </button>
{/if}
