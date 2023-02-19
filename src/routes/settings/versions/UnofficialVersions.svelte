<script lang="ts">
  import Icon from "@iconify/svelte";
  import { Button, TabItem } from "flowbite-svelte";

  const tabItemActiveClasses =
    "inline-block text-sm font-bold text-center disabled:cursor-not-allowed p-4 text-orange-500 border-b-2 border-orange-500 dark:text-orange-500 dark:border-orange-500";
  const tabItemInactiveClasses =
    "inline-block text-sm font-normal text-center disabled:cursor-not-allowed p-4 border-b-2 border-transparent text-gray-400 hover:text-orange-300 hover:border-orange-500 dark:hover:text-orange-300 dark:text-orange-400";
</script>

<TabItem
  activeClasses={tabItemActiveClasses}
  inactiveClasses={tabItemInactiveClasses}
>
  <span slot="title">Unofficial</span>
  <p class="text-sm text-gray-400 dark:text-gray-300">
    Unofficial versions are typically modified `jak-project` releases to enable
    changes or new content
  </p>
  <div class="flex items-center mt-2 mb-2">
    <div class="grow">
      <p class="text-sm text-gray-400 dark:text-gray-300">
        These are not supported by the OpenGOAL team and will have to be
        manually added to the folder at this time
      </p>
    </div>
    <div class="flex">
      <Button
        btnClass="!p-2 mr-2 rounded-md dark:bg-orange-500 hover:dark:bg-orange-600 text-slate-900"
      >
        <Icon
          icon="material-symbols:refresh"
          width="20"
          height="20"
          alt="refresh official version list"
        />
      </Button>
      <Button
        btnClass="!p-2 rounded-md dark:bg-orange-500 hover:dark:bg-orange-600 text-slate-900"
      >
        <Icon
          icon="material-symbols:folder-open-rounded"
          width="20"
          height="20"
          alt="open official version folder"
        />
      </Button>
    </div>
  </div>
</TabItem>
