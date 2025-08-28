<script lang="ts">
  import type { LayoutProps } from "./$types";
  import { TabItem, Tabs } from "flowbite-svelte";
  import General from "./General.svelte";
  import Versions from "./Versions.svelte";
  import Mods from "./Mods.svelte";
  import Decompiler from "./Decompiler.svelte";
  import { page } from "$app/state";
  import { _ } from "svelte-i18n";

  let { data, children }: LayoutProps = $props();

  const activeClass =
    "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  const inactiveClass =
    "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";

  const activeTab = $derived(page.params.tab);
</script>

{@render children()}

<div class="flex flex-col h-full bg-gray-900">
  <Tabs
    tabStyle="underline"
    class="flex flex-wrap space-x-2 rtl:space-x-reverse"
    contentClass="p-4 pt-0 rounded-lg mt-2 mb-5 overflow-y-auto !bg-gray-900"
  >
    <TabItem
      {activeClass}
      {inactiveClass}
      open={activeTab.includes("general")}
      title={$_("settings_tabs_general")}
    >
      <General />
    </TabItem>
    <TabItem
      {activeClass}
      {inactiveClass}
      open={activeTab.includes("versions")}
      title={$_("settings_tabs_versions")}
    >
      <Versions />
    </TabItem>
    <TabItem
      {activeClass}
      {inactiveClass}
      title={$_("settings_tabs_decompiler")}
    >
      <Decompiler />
    </TabItem>
    <TabItem
      {activeClass}
      {inactiveClass}
      open={activeTab.includes("mod")}
      title={$_("settings_tabs_mods")}
    >
      <Mods />
    </TabItem>
  </Tabs>
</div>
