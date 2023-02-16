<script>
  import { openDir } from "$lib/rpc/commands";
  import { appDir, join } from "@tauri-apps/api/path";
  import { Alert, Button, Tabs, TabItem } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { handleCheckUpdate } from "$lib/utils/updates";
  import { UpdateStore } from "$lib/stores/AppStore";
  import { Route, Router, useLocation, useParams } from "svelte-navigator";
  import General from "./settings/General.svelte";
  import Folders from "./settings/Folders.svelte";
  import Versions from "./settings/Versions.svelte";

  const params = useParams();
  $: activeTab = $params["tab"];

  let directory = undefined;
  let logDir = undefined;

  onMount(async () => {
    directory = await appDir();
    logDir = await join(directory, "/logs");
  });

  const tabItemActiveClasses =
    "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  const tabItemInactiveClasses =
    "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";
</script>

<div class="ml-20">
  <div class="flex flex-col h-screen bg-slate-900">
    <!-- https://flowbite-svelte.com/components/tab#Tabs_with_icons -->
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
          open={activeTab === "folders"}
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
