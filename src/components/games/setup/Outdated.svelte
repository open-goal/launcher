<script>
  import { Alert, Button, Spinner } from "flowbite-svelte";
  import { copyDataDirectory } from "$lib/utils/data-files";
  import { launcherConfig } from "$lib/config";
  import { gameNeedsReinstall } from "$lib/stores/AppStore";

  export let updatingDataDir;
  export let activeGame;
  let errorText = "";

  async function syncDataDirectory() {
    updatingDataDir = true;
    errorText = "";
    try {
      await copyDataDirectory();
      // Now that the directory is up to date, let's see if they need to reinstall the game
      if (await launcherConfig.shouldUpdateGameInstall(activeGame)) {
        gameNeedsReinstall.update(() => true);
      }
    } catch (err) {
      errorText = `Error encountered when syncing data files - ${err}`;
    }
    updatingDataDir = false;
  }
</script>

<div class="ml-20">
  <div class="flex flex-col h-5/6 p-8 space-y-2">
    <Alert color="red" rounded={false} accent>
      <span class="font-medium text-2xl">Notice:</span>
      <ul slot="extra" class="mt-0 ml-8 list-disc list-inside text-lg">
        <li>Local data files must be synced up in-order to proceed</li>
        <li>This may overwrite any modifications to the game's source code</li>
        <li>Save files and settings will not be modified</li>
      </ul>
    </Alert>
    {#if !updatingDataDir}
      <Button
        class="!rounded-none !bg-[#222222] text-xl"
        on:click={syncDataDirectory}>Sync Data Files</Button
      >
    {:else}
      <Spinner />
    {/if}

    {#if errorText != ""}
      <!-- TODO: Add styling for this -->
      {errorText}
    {/if}
  </div>
</div>
