<script>
  import { setInstallStatus } from "$lib/config";
  import { SUPPORTED_GAME } from "$lib/constants";

  import { launchGame } from "$lib/launch";

  function onClickConfig() {}

  function onClickPlay() {
    launchGame();
  }

  async function onClickUninstall() {
    // TODO - refresh the UI
    await setInstallStatus(SUPPORTED_GAME.Jak1, false);
  }
</script>

<div id="launcherControls">
  <button class="btn lg" on:click={onClickPlay}>Play</button>
  <div class="mt-1">
    <button class="btn md" on:click={onClickConfig} disabled>Config</button>
    <button class="btn md" on:click={onClickUninstall}>Uninstall</button>
  </div>
</div>
