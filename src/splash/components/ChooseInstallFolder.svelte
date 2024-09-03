<script lang="ts">
  import { setInstallationDirectory } from "$lib/rpc/config";
  import { folderPrompt } from "$lib/utils/file-dialogs";
  import { createEventDispatcher, onMount } from "svelte";
  import { _ } from "svelte-i18n";

  const dispatch = createEventDispatcher();

  let stepError = "";

  // Events
  onMount(async () => {});
</script>

{#if stepError !== ""}
  <p class="text-wrap text-pretty">{stepError}</p>
{:else}
  <p class="text-wrap text-pretty">{$_("splash_noInstallDirSet")}</p>
{/if}
<button
  data-testId="pick-install-folder-button"
  class="splash-button pointer-events-auto bg-orange-500 p-1 mt-1 text-black font-bold rounded hover:bg-orange-700"
  on:click={async () => {
    stepError = "";
    // This is part of what allows for the user to install the games and such wherever they want
    const newInstallDir = await folderPrompt(
      $_("splash_button_setInstallFolder_prompt"),
    );
    if (newInstallDir !== undefined) {
      const result = await setInstallationDirectory(newInstallDir);
      if (result !== null) {
        stepError = result;
      } else {
        dispatch("complete", {});
      }
    }
  }}>{$_("splash_button_setInstallFolder")}</button
>
