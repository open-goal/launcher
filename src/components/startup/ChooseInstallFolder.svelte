<script lang="ts">
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import { _ } from "svelte-i18n";

  let { installDir = $bindable() }: { installDir: string } = $props();

  async function chooseInstallFolder() {
    const newInstallDir = await folderPrompt(
      $_("splash_button_setInstallFolder_prompt"),
    );

    if (!newInstallDir) return;
    installDir = newInstallDir;
  }
</script>

<button
  data-testId="pick-install-folder-button"
  class="splash-button pointer-events-auto bg-orange-500 p-1 mt-1 text-black font-bold rounded hover:bg-orange-700"
  onclick={chooseInstallFolder}
  >{$_("splash_button_setInstallFolder")}
</button>
