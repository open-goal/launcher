<script lang="ts">
  import type { LayoutProps } from "./$types";
  import { _ } from "svelte-i18n";
  import { page } from "$app/state";

  let { data, children }: LayoutProps = $props();
  let activeUrl = $derived(page.url.pathname);
</script>

<div class="flex flex-col h-full bg-zinc-900 p-4">
  <nav class="flex h-10 mx-auto bg-zinc-900">
    <div class="flex gap-8">
      <a href="/settings/general">General</a>
      <a href="/settings/versions">Versions</a>
      <a href="/settings/decompiler">Decompiler</a>
      <a href="/settings/mods">Mods</a>
    </div>
  </nav>
  {@render children()}
</div>
