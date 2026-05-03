<script lang="ts">
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import IconDownload from "~icons/mdi/tray-arrow-down";
  import IconNew from "~icons/mdi/creation";
  // import IconDots from "~icons/mdi/dots-horizontal";
  import { navigate } from "/src/router";

  let {
    mod,
    activeGame,
  }: {
    mod: ModInfo;
    activeGame: SupportedGame;
  } = $props();

  const modInternalName = $derived(mod.name);
  const modSourceName = $derived(mod.source);
  const perGameConfig = $derived(mod.perGameConfig);
  const downloadCount = $derived(mod.downloadCount);

  function showNewIndicator(): boolean {
    const releaseDate = perGameConfig?.[activeGame]?.releaseDate;
    if (!releaseDate) return false;
    const releaseTime = Date.parse(releaseDate);
    const daysSinceRelease = (Date.now() - releaseTime) / (1000 * 3600 * 24);
    return daysSinceRelease < 30 && !mod.installed;
  }

  const thumbnailUrl = $derived(
    perGameConfig?.[activeGame]?.thumbnailArtUrl || mod.thumbnailArtUrl
  );
  const description = $derived(
    perGameConfig?.[activeGame]?.description || mod.description
  );
  const modDisplayName = $derived(
    perGameConfig?.[activeGame]?.displayName || mod.displayName
  );

  async function gotoMod(modName: string) {
    await navigate(`/:game_name/mods/:source_name/:mod_name`, {
      params: {
        game_name: activeGame,
        source_name: encodeURI(modSourceName),
        mod_name: encodeURI(modName),
      },
    });
  }
</script>

<div
  role="button"
  tabindex="0"
  onclick={async () => gotoMod(modInternalName)}
  onkeydown={(e) => e.key === "Enter" && gotoMod(modInternalName)}
  class="flex gap-4 rounded-xl border h-56 border-zinc-700/60 bg-zinc-900/60 pr-4 transition-all duration-200
  shadow-[0_8px_24px_rgba(0,0,0,0.6)]
  hover:border-zinc-500/60
  hover:bg-zinc-900/80
  hover:scale-[1.01]"
>
  <div dir="ltr" class="relative aspect-4/5 shrink-0 overflow-hidden">
    <img
      src={thumbnailUrl}
      alt={modDisplayName}
      class="h-full w-full rounded-l-lg"
    />
  </div>

  <div class="flex flex-1 flex-col py-2 overflow-hidden">
    <div class="flex flex-col gap-2">
      <h2 class="text-lg font-bold text-white">
        {modDisplayName}
      </h2>

      <p class="text-sm text-zinc-400">
        by {mod.authors.join(", ")}
      </p>
    </div>

    <p class="mt-2 line-clamp-3 text-sm text-zinc-300">
      {description}
    </p>

    <div class="mt-auto flex items-center gap-2 pt-2 text-sm text-zinc-400">
      <IconDownload class="text-base" />
      {downloadCount}
      {#if showNewIndicator()}
        <IconNew class="text-base text-green-400 ml-auto animate-pulse"
        ></IconNew>
      {/if}
      <!-- TODO: ADD TINY SETTINGS POPUP WITH WEBSITE AND UNINSTALL BUTTON -->
      <!-- <IconDots class="text-base ml-auto"></IconDots> -->
    </div>
  </div>
</div>
