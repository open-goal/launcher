<script>
  import { Tabs, TabItem } from "flowbite-svelte";
  import { useParams } from "svelte-navigator";
  import Versions from "./settings/Versions.svelte";
  import General from "./settings/General.svelte";
  import { _ } from "svelte-i18n";
  import Mods from "./settings/Mods.svelte";
  import Decompiler from "./settings/Decompiler.svelte";

  const params = useParams();
  $: activeTab = $params["tab"];

  const tabItemActiveClasses =
    "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  const tabItemInactiveClasses =
    "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";
</script>

<div class="flex flex-col h-full bg-[#141414]">
  <!-- https://flowbite-svelte.com/components/tab#Tabs_with_icons -->
  <Tabs
    style="underline"
    divider={false}
    defaultClass="flex flex-wrap space-x-2 rtl:space-x-reverse bg-[#1e1e1e]"
    contentClass="p-4 pt-0 rounded-lg mt-2 mb-5 overflow-y-auto"
  >
    <TabItem
      open={!activeTab || activeTab === "general"}
      title={$_("settings_tabs_general")}
      activeClasses={tabItemActiveClasses}
      inactiveClasses={tabItemInactiveClasses}
    >
      <General />
    </TabItem>
    <TabItem
      open={activeTab === "versions"}
      title={$_("settings_tabs_versions")}
      activeClasses={tabItemActiveClasses}
      inactiveClasses={tabItemInactiveClasses}
    >
      <Versions />
    </TabItem>
    <TabItem
      open={activeTab === "decompiler"}
      title={$_("settings_tabs_decompiler")}
      activeClasses={tabItemActiveClasses}
      inactiveClasses={tabItemInactiveClasses}
    >
      <Decompiler />
    </TabItem>
    <TabItem
      open={activeTab === "mods"}
      title={$_("settings_tabs_mods")}
      activeClasses={tabItemActiveClasses}
      inactiveClasses={tabItemInactiveClasses}
    >
      <Mods />
    </TabItem>
  </Tabs>
</div>
