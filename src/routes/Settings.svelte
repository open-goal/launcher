<script>
  import { Tabs, TabItem } from "flowbite-svelte";
  import { useParams } from "svelte-navigator";
  import Folders from "./settings/Folders.svelte";
  import Versions from "./settings/Versions.svelte";

  const params = useParams();
  $: activeTab = $params["tab"];

  const tabItemActiveClasses =
    "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  const tabItemInactiveClasses =
    "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";
</script>

<div class="ml-20">
  <!-- TODO - the static height here is kinda a hack, it's because the
    header and the rest of the layout aren't within a shared container -->
  <div class="flex flex-col h-[544px] bg-slate-900">
    <!-- https://flowbite-svelte.com/components/tab#Tabs_with_icons -->
    <Tabs
      style="underline"
      divider={false}
      contentClass="p-4 pt-0 rounded-lg mt-2 pb-20 overflow-y-auto"
    >
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
    </Tabs>
  </div>
</div>
