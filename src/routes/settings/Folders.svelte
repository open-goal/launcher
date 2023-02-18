<script lang="ts">
  import {
    getInstallationDirectory,
    setInstallationDirectory,
  } from "$lib/rpc/config";
  import { folderPrompt } from "$lib/utils/file";
  import { Label, Input } from "flowbite-svelte";
  import { onMount } from "svelte";

  let currentInstallationDirectory = "";

  onMount(async () => {
    currentInstallationDirectory = await getInstallationDirectory();
  });
</script>

<div class="flex flex-col gap-2 mt-2">
  <div>
    <Label for="default-input" class="block mb-2">Installation Directory</Label>
    <Input
      id="default-input"
      placeholder={currentInstallationDirectory}
      on:click={async () => {
        const newInstallDir = await folderPrompt("Pick an Installation Folder");
        if (
          newInstallDir !== undefined &&
          newInstallDir !== currentInstallationDirectory
        ) {
          await setInstallationDirectory(newInstallDir);
          currentInstallationDirectory = newInstallDir;
        }
      }}
    />
  </div>
</div>
