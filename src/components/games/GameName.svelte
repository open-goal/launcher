<script lang="ts">
  import type { SupportedGame } from "$lib/rpc/bindings/SupportedGame";
  import { _ } from "svelte-i18n";
  import { config } from "/src/state/config.svelte";

  let {
    activeGame,
  }: {
    activeGame: SupportedGame;
  } = $props();

  // this doesnt work for the following languages: ja-JP, he-IL
  let [title, subtitle] = $derived.by(() => {
    const name = $_(`gameName_${activeGame}`);
    const [t, s] = name.split(":");
    return [t, s?.trimStart()];
  });
</script>

{#if !config?.ui?.hideGamename}
  <p
    class="font-title text-3xl font-semibold tracking-wider text-gray-100 pointer-events-none uppercase"
  >
    {title}
  </p>
  <p class="text-2xl font-medium text-amber-500">{subtitle}</p>
{/if}
