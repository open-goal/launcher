<script lang="ts">
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import { _ } from "svelte-i18n";
  import { Input, Label } from "flowbite-svelte";

  let { installDir = $bindable() }: { installDir: string } = $props();

  async function chooseInstallFolder() {
    const newInstallDir = await folderPrompt(
      $_("splash_button_setInstallFolder_prompt"),
    );

    if (!newInstallDir) return;
    installDir = newInstallDir;
  }
</script>

<div class="flex-row w-full">
  <Label for="default-input" class="block mb-2 text-gray-200 font-semibold"
    >{$_("settings_folders_installationDir")}</Label
  >
  <Input
    id="default-input"
    placeholder={installDir ? installDir : $_("splash_button_setInstallFolder")}
    onclick={chooseInstallFolder}
  />
  <!-- <Helper class="text-xs mt-2 italic"
    >{$_("settings_general_installationDir_helper")}</Helper
  > -->
</div>
