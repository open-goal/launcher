<script>
  import { Tabs, TabItem } from "flowbite-svelte";
  import { useParams } from "svelte-navigator";
  import Folders from "./settings/Folders.svelte";
  import LauncherSettings from "./settings/LauncherSettings.svelte";
  import Versions from "./settings/Versions.svelte";
  import General from "./settings/General.svelte";

  const params = useParams();
  $: activeTab = $params["tab"];

  const tabItemActiveClasses =
    "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  const tabItemInactiveClasses =
    "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";
</script>

<div class="flex flex-col h-full bg-slate-900">
  <!-- https://flowbite-svelte.com/components/tab#Tabs_with_icons -->
  <!-- TODO - the pb-20 is a bit of a hack figure out the actual problem with the DOM -->
  <Tabs
    style="underline"
    divider={false}
    contentClass="p-4 pt-0 rounded-lg mt-2 pb-20 overflow-y-auto"
  >
    <TabItem
      open={!activeTab || activeTab === "general"}
      title="General"
      activeClasses={tabItemActiveClasses}
      inactiveClasses={tabItemInactiveClasses}
    >
      <General />
    </TabItem>
    <TabItem
      open={!activeTab || activeTab === "folders"}
      title="Folders"
      activeClasses={tabItemActiveClasses}
      inactiveClasses={tabItemInactiveClasses}
    >
      <Folders />
    </TabItem>
    <TabItem
      open={activeTab === "versions"}
      title="Version Management"
      activeClasses={tabItemActiveClasses}
      inactiveClasses={tabItemInactiveClasses}
    >
      <Versions />
    </TabItem>
    <TabItem
      open={activeTab === "launchersettings"}
      title="Launcher Settings"
      activeClasses={tabItemActiveClasses}
      inactiveClasses={tabItemInactiveClasses}
    >
    <LauncherSettings />
    </TabItem>
  </Tabs>
</div>
