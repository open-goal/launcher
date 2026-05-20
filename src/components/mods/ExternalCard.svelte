<script lang="ts">
  import type { ModInfo } from "$lib/rpc/bindings/ModInfo";
  import { _ } from "svelte-i18n";

  let {
    mod,
  }: {
    mod: ModInfo;
  } = $props();

  const thumbnailUrl = $derived(mod.thumbnailArtUrl);
  const description = $derived(mod.description);
  const modDisplayName = $derived(mod.displayName);
  const externalLink = $derived(mod.externalLink);
</script>

<a
  role="button"
  tabindex="0"
  href={externalLink}
  target="_blank"
  rel="noopener noreferrer"
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
        {$_("features_mods_by")}: {mod.authors.join(", ")}
      </p>
    </div>

    <p class="mt-2 line-clamp-3 text-sm text-zinc-300">
      {description}
    </p>
  </div>
</a>
