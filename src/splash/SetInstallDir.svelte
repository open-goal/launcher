<script>
  import { setInstallationDirectory } from "$lib/rpc/config";
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import { Button } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import { onMount } from "svelte";

  export let currentProgress;
  export let currentStatusText;
  export let stepError;
  export let installationDirSet;

  onMount(() => {
    currentStatusText = $_("splash_step_pickInstallFolder");
  });
</script>

<Button
  data-testId="pick-install-folder-button"
  color="yellow"
  class="mx-4 mt-2"
  on:click={async () => {
    // This is part of what allows for the user to install the games and such wherever they want
    currentProgress = 25;
    const newInstallDir = await folderPrompt(
      $_("splash_button_setInstallFolder_prompt"),
    );
    if (newInstallDir !== undefined) {
      const result = await setInstallationDirectory(newInstallDir);
      if (result !== null) {
        stepError = result;
      } else {
        installationDirSet = true;
      }
    }
  }}
  >{$_("splash_button_setInstallFolder")}
</Button>
