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

  const selectMoviePath = async (jakVersion) => {
  const allowedFormats = ["webm", "mp4"];
  const newMovieDir = await filePrompt(allowedFormats, "Movie", "Pick a movie");

  if (newMovieDir && newMovieDir !== window[`jak${jakVersion}BackgroundPath`]) {
    let errMsg = null;
    if (jakVersion == "jak1") {
      errMsg = await setJak1MoviePath(newMovieDir);
      if (errMsg === null) {
        currentJak1BackgroundPath = newMovieDir;
      }
    }
    if (jakVersion == "jak2") {
      errMsg = await setJak2MoviePath(newMovieDir);
      if (errMsg === null) {
        currentJak2BackgroundPath = newMovieDir;
      }
    }
  }
};

  const resetMoviePath = async (jakVersion) => {
    if(jakVersion === "jak1"){
      currentJak1BackgroundPath = "";
      await setJak1MoviePath("");
    }
    if(jakVersion === "jak2"){
      currentJak2BackgroundPath = "";
      await setJak2MoviePath("");
    }
  }

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

  <div>
    <Label for="jak1-input" class="block mb-2">Click box to set Jak 1 Background Movie</Label>
    <Input id="jak1-input" bind:value={currentJak1BackgroundPath} on:click={() => selectMoviePath("jak1")} />
      <button class="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2" on:click={() => resetMoviePath("jak1") }>
        Reset
      </button>
  </div>
  
  <div>
    <Label for="jak2-input" class="block mb-2">Click box to set Jak 2 Background Movie</Label>
    <Input id="jak2-input" bind:value={currentJak2BackgroundPath} on:click={() => selectMoviePath("jak2")} />
      <button class="flex-shrink border-solid rounded bg-white hover:bg-orange-400 text-sm text-slate-900 font-semibold px-5 py-2" on:click={() => resetMoviePath("jak2") }>
        Reset
      </button>
  </div>
</div>
