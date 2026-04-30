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

<div
  class="flex-row w-full p-2 border border-zinc-600/40 bg-zinc-800/40 rounded-md"
>
  <Label
    for="default-input"
    class="block mb-2 text-gray-200 font-semibold cursor-pointer"
    >{$_("settings_folders_installationDir")}</Label
  >
  <Input
    id="default-input"
    class="rounded-sm border border-zinc-400/20! bg-zinc-950/80! text-gray-200 cursor-pointer"
    placeholder={installDir ? installDir : $_("splash_button_setInstallFolder")}
    onclick={chooseInstallFolder}
  />
</div>
