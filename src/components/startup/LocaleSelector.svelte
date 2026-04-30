<script lang="ts">
  import { AVAILABLE_LOCALES } from "$lib/i18n/i18n";
  import { _ } from "svelte-i18n";

  let { locale = $bindable() }: { locale: string } = $props();
</script>

<div class="flex flex-col w-full items-center gap-4">
  <h1 class="font-mono text-3xl tracking-wide text-white drop-shadow-lg">
    {$_("splash_selectLocale")}
  </h1>

  <div
    class="grid max-h-[48vh] w-full grid-cols-3 gap-3 overflow-y-auto p-2 text-nowrap border border-zinc-600/40 bg-zinc-800/40 rounded-md"
    role="radiogroup"
    aria-label={$_("splash_selectLocale")}
  >
    {#each AVAILABLE_LOCALES as item}
      <label
        class={[
          "flex cursor-pointer items-center gap-2 rounded bg-zinc-950/80 px-6 py-4",
          "text-lg font-bold text-orange-500 transition",
          "hover:bg-zinc-900 hover:text-orange-400",
          locale === item.id && "ring-2 ring-orange-400 bg-zinc-900",
        ]}
      >
        <input
          class="sr-only"
          type="radio"
          name="locale"
          value={item.id}
          bind:group={locale}
        />

        <span class="text-xl font-normal">{item.flag}</span>
        <span class="font-mono">{item.localizedName}</span>
      </label>
    {/each}
  </div>
</div>
