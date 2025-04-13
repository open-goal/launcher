<script>
  import { Tabs, TabItem } from "flowbite-svelte";
  import Versions from "./settings/Versions.svelte";
  import General from "./settings/General.svelte";
  import { _ } from "svelte-i18n";
  import Mods from "./settings/Mods.svelte";
  import Decompiler from "./settings/Decompiler.svelte";

  const tabItemActiveClasses =
    "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  const tabItemInactiveClasses =
    "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";
</script>

<div class="flex flex-col h-full bg-[#141414]">
  <Tabs
    style="underline"
    divider={false}
    defaultClass="flex flex-wrap space-x-2 rtl:space-x-reverse bg-[#1e1e1e]"
    contentClass="p-4 pt-0 rounded-lg mt-2 mb-5 overflow-y-auto"
    activeClasses={tabItemActiveClasses}
    inactiveClasses={tabItemInactiveClasses}
  >
    <TabItem open title={$_("settings_tabs_general")}>
      <General />
    </TabItem>
    <TabItem title={$_("settings_tabs_versions")}>
      <Versions />
    </TabItem>
    <TabItem title={$_("settings_tabs_decompiler")}>
      <Decompiler />
    </TabItem>
    <TabItem title={$_("settings_tabs_mods")}>
      <Mods />
    </TabItem>
  </Tabs>
</div>
