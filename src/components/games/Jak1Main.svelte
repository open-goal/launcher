<script>
  import { setInstallStatus } from "$lib/config";
  import { SupportedGame } from "$lib/constants";
  import { navigate } from "svelte-navigator";
  import { launchGame } from "$lib/launch";

  function onClickConfig() {}

  function onClickPlay() {
    launchGame();
  }

  async function onClickUninstall() {
    await setInstallStatus(SupportedGame.Jak1, false);
    navigate("/jak1", { replace: true });
  }
</script>

<div id="launcherControls">
  <button class="btn lg" on:click={onClickPlay}>Play</button>
  <div class="mt-1">
    <button class="btn md" on:click={onClickConfig} disabled>Config</button>
    <button class="btn md" on:click={onClickUninstall}>Uninstall</button>
  </div>
</div>
