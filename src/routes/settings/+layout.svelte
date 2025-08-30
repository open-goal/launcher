<script lang="ts">
  import type { LayoutProps } from "./$types";
  import { _ } from "svelte-i18n";
  import { page } from "$app/state";
  import { Navbar, NavLi, NavUl } from "flowbite-svelte";

  let { data, children }: LayoutProps = $props();
  let activeUrl = $derived(page.url.pathname);
</script>

<div class="flex flex-col h-full bg-zinc-900 p-4">
  <Navbar class="flex bg-zinc-900 items-center p-0 pb-4">
    <NavUl
      {activeUrl}
      class="min-w-full"
      ulClass="flex min-w-max justify-between p-0"
    >
      <NavLi href="/settings/general">General</NavLi>
      <NavLi href="/settings/versions">Versions</NavLi>
      <NavLi href="/settings/decompiler">Decompiler</NavLi>
      <NavLi href="/settings/mods">Mods</NavLi>
    </NavUl>
  </Navbar>
  {@render children()}
</div>
