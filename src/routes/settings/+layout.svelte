<script lang="ts">
  import type { LayoutProps } from "./$types";
  import { _ } from "svelte-i18n";
  import { page } from "$app/state";
  import { Navbar, NavLi, NavUl } from "flowbite-svelte";

  let { data, children }: LayoutProps = $props();
  let activeUrl = $derived(page.url.pathname);

  // const activeClass =
  //   "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  // const inactiveClass =
  //   "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";
</script>

<div class="flex flex-col h-full bg-zinc-900 p-4">
  <Navbar class="flex bg-zinc-900 items-center p-0 pb-4">
    <NavUl
      {activeUrl}
      class="min-w-full"
      ulClass="flex min-w-max justify-between p-0"
    >
      <NavLi href="/settings/general">{$_("settings_tabs_general")}</NavLi>
      <NavLi href="/settings/versions">{$_("settings_tabs_versions")}</NavLi>
      <NavLi href="/settings/decompiler">{$_("settings_tabs_decompiler")}</NavLi
      >
      <NavLi href="/settings/mods">{$_("settings_tabs_mods")}</NavLi>
    </NavUl>
  </Navbar>
  {@render children()}
</div>
