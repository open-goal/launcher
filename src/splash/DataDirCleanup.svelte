<script>
  import { deleteOldDataDirectory } from "$lib/rpc/config";
  import { Button, ButtonGroup } from "flowbite-svelte";
  import { _ } from "svelte-i18n";
  import { onMount } from "svelte";

  export let oldDataDirToClean;
  export let currentStatusText;

  onMount(() => {
    currentStatusText = $_("splash_deleteOldInstallDir");
  });
</script>

<div class="flex flex-col align-center">
  <ButtonGroup divClass="flex justify-center">
    <Button
      color="yellow"
      data-testId="delete-old-data-dir-button"
      on:click={async () => {
        await deleteOldDataDirectory();
        oldDataDirToClean = false;
      }}
    >
      {$_("splash_button_deleteOldInstallDir_yes")}</Button
    >
    <Button
      color="yellow"
      data-testId="dont-delete-old-data-dir-button"
      on:click={() => {
        oldDataDirToClean = false;
      }}>{$_("splash_button_deleteOldInstallDir_no")}</Button
    >
  </ButtonGroup>
</div>
