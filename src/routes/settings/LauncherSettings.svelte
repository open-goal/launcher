<script lang="ts">
  import { getMoviePath, setMoviePath } from "$lib/rpc/config";
  import { VersionStore } from "$lib/stores/VersionStore";
  import { filePrompt } from "$lib/utils/file";
  import { Label, Input } from "flowbite-svelte";
  import { onMount } from "svelte";

  let currentBackgroundPath = "";

  onMount(async () => {
    currentBackgroundPath = await getMoviePath();
  });
</script>

<div class="flex flex-col gap-2 mt-2">
  <div>
    <Label for="default-input" class="block mb-2"
      >Click box to set Background Movie</Label
    >
    <Input
      id="default-input"
      placeholder={currentBackgroundPath}
      on:click={async () => {
        const newMovieDir = await filePrompt(
          ["webm", "mp4"],
          "Movie",
          "Pick a movie"
        );
        if (
          newMovieDir !== undefined &&
          newMovieDir !== currentBackgroundPath
        ) {
          const errMsg = await setMoviePath(newMovieDir);
          if (errMsg === null) {
            if (currentBackgroundPath !== newMovieDir) {
              $VersionStore.activeVersionType = null;
              $VersionStore.activeVersionName = null;
            }
            currentBackgroundPath = newMovieDir;
          }
        }
      }}
    />
  </div>
</div>
