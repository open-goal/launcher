<script lang="ts">
  import {
    getInstallationDirectory,
    setInstallationDirectory,
    getJak1MoviePath,
    getJak2MoviePath,
    setJak1MoviePath,
    setJak2MoviePath
  } from "$lib/rpc/config";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { folderPrompt,
           filePrompt
  } from "$lib/utils/file";
  import { Label, Input } from "flowbite-svelte";
  import { onMount } from "svelte";

  let currentInstallationDirectory = "";
  let currentJak1BackgroundPath = "";
  let currentJak2BackgroundPath = "";

  onMount(async () => {
    currentInstallationDirectory = await getInstallationDirectory();
    currentJak1BackgroundPath = await getJak1MoviePath();
    currentJak2BackgroundPath = await getJak2MoviePath();
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
  <!-- TODO Jak 1 background stuff beta -->
  <div>
    <Label for="default-input" class="block mb-2"
      >Click box to set Jak 1 Background Movie</Label
    >
    <Input
      id="default-input"
      placeholder={currentJak1BackgroundPath}
      on:click={async () => {
        const newMovieDir = await filePrompt(
          ["webm", "mp4"],
          "Movie",
          "Pick a movie"
        );
        if (
          newMovieDir !== undefined &&
          newMovieDir !== currentJak1BackgroundPath
        ) {
          const errMsg = await setJak1MoviePath(newMovieDir);
          if (errMsg === null) {
            if (currentJak1BackgroundPath !== newMovieDir) {
              $VersionStore.activeVersionType = null;
              $VersionStore.activeVersionName = null;
            }
            currentJak1BackgroundPath = newMovieDir;
          }
        }
      }}
    />
  </div>
  <!-- TODO Jak 2 background stuff beta -->
  <div>
    <Label for="default-input" class="block mb-2"
      >Click box to set Jak 2 Background Movie</Label
    >
    <Input
      id="default-input"
      placeholder={currentJak2BackgroundPath}
      on:click={async () => {
        const newMovieDir = await filePrompt(
          ["webm", "mp4"],
          "Movie",
          "Pick a movie"
        );
        if (
          newMovieDir !== undefined &&
          newMovieDir !== currentJak1BackgroundPath
        ) {
          const errMsg = await setJak2MoviePath(newMovieDir);
          if (errMsg === null) {
            if (currentJak2BackgroundPath !== newMovieDir) {
              $VersionStore.activeVersionType = null;
              $VersionStore.activeVersionName = null;
            }
            currentJak2BackgroundPath = newMovieDir;
          }
        }
      }}
    />
  </div>
</div>
