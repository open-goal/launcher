<script type="ts">
  import { getGameInstallVersion, setInstallStatus } from "$lib/config";
  import { SupportedGame } from "$lib/constants";
  import { navigate } from "svelte-navigator";
  import { launchGame } from "$lib/launch";
  import { compileGame, decompileGameData } from "$lib/setup/setup";
  import { appDir, join } from "@tauri-apps/api/path";

  export let activeGame: SupportedGame;

  function onClickPlay() {
    launchGame();
  }

  async function onClickUninstall() {
    await setInstallStatus(SupportedGame.Jak1, false);
    // TODO - this is essentially a refresh, shouldn't be required
    // if our app is properly reactive, see if this can be eliminated
    navigate(0);
  }

  async function onClickDecompile() {
    // TODO - ensure this path is correct!
    const isoPath = await join(await appDir(), "data", "extracted_iso");
    await decompileGameData(isoPath);
  }

  async function onClickCompile() {
    // TODO - ensure this path is correct!
    const isoPath = await join(await appDir(), "data", "extracted_iso");
    await compileGame(isoPath);
  }
</script>

<div id="launcherControls">
  <button class="btn lg" on:click={onClickPlay}>Play</button>
  {#await getGameInstallVersion(activeGame) then version}
    <p>Game Version: {version}</p>
  {/await}
  <!-- TODO - add an "open saves/settings folder" -->
  <!-- TODO - when clicking decompile/compile -- show logs -->
  <div class="mt-1">
    <button class="btn md" on:click={onClickDecompile}>Decompile</button>
    <button class="btn md" on:click={onClickCompile}>Compile</button>
    <button class="btn md" on:click={onClickUninstall}>Uninstall</button>
  </div>
</div>
