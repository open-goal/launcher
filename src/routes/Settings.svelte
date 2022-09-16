<script>
  import { openDir } from "$lib/rpc/commands";
  import { appDir } from "@tauri-apps/api/path";
  import { Alert, Button } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { handleCheckUpdate } from "$lib/utils/updates";

  let directory = undefined;

  onMount(async () => {
    directory = await appDir();
  });
</script>

<div class="ml-20">
  <div class="flex flex-col h-5/6 p-8">
    <Alert color="dark" rounded={false} accent>
      <span class="text-lg font-medium text-black-900 dark:text-blue-800 mb-4"
        >Settings</span
      >
      <div slot="extra">
        <div class="flex flex-col gap-2">
          <Button
            class="!rounded-none"
            size="md"
            outline
            color="green"
            on:click={() => openDir(directory)}
          >
            <i class="fa fa-folder mx-1" />
            Open Launcher Install Directory
          </Button>
          <Button
            class="!rounded-none"
            size="md"
            outline
            color="green"
            on:click={() => openDir(directory)}
          >
            <i class="fa fa-folder mx-1" />
            Open Logs Directory
          </Button>
          <Button
            class="!rounded-none"
            size="md"
            outline
            color="green"
            on:click={async () => await handleCheckUpdate()}
          >
            <i class="fa fa fa-download mx-1" />
            Check For Updates
          </Button>
          <!-- TODO: Add a button that lets users change the install directory -->
        </div>
      </div>
    </Alert>
  </div>
</div>
