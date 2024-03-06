<!-- mod might want to override the background, this is possible (i think, maybe from a URL? (release mode?)) -->
<script lang="ts">
  import {
    getInstallationDirectory,
    setInstallationDirectory,
  } from "$lib/rpc/config";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import { Label, Input } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";

  let currentInstallationDirectory = "";

  onMount(async () => {
    currentInstallationDirectory = await getInstallationDirectory();
  });
</script>

<div class="flex flex-col gap-2 mt-2">
  <div>
    <Label for="default-input" class="block mb-2"
      >{$_("settings_folders_installationDir")}</Label
    >
    <Input
      id="default-input"
      placeholder={currentInstallationDirectory}
      on:click={async () => {
        const newInstallDir = await folderPrompt(
          $_("settings_folders_installationDir_prompt"),
        );
        if (
          newInstallDir !== undefined &&
          newInstallDir !== currentInstallationDirectory
        ) {
          const errMsg = await setInstallationDirectory(newInstallDir);
          if (errMsg === null) {
            if (currentInstallationDirectory !== newInstallDir) {
              $VersionStore.activeVersionType = null;
              $VersionStore.activeVersionName = null;
            }
            currentInstallationDirectory = newInstallDir;
          }
        }
      }}
    />
  </div>
</div>
