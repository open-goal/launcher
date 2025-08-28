<script lang="ts">
  import { setInstallationDirectory } from "$lib/rpc/config";
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import { _ } from "svelte-i18n";
  import { Button } from "flowbite-svelte";
</script>

<Button
  data-testId="pick-install-folder-button"
  class="splash-button pointer-events-auto bg-orange-500 p-1 mt-1 text-black font-bold rounded hover:bg-orange-700"
  onclick={async () => {
    // This is part of what allows for the user to install the games and such wherever they want
    const newInstallDir = await folderPrompt(
      $_("splash_button_setInstallFolder_prompt"),
    );
    if (newInstallDir !== undefined) {
      const result = await setInstallationDirectory(newInstallDir);
    }
  }}>{$_("splash_button_setInstallFolder")}</Button
>
