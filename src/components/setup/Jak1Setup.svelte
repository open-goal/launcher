<script type="ts">
  import { navigate } from "svelte-navigator";
  import { filePrompt } from "$lib/utils/file";
  import { setInstallStatus } from "$lib/config";
  import { clearInstallLogs } from "$lib/utils/file";
  import {
    compileGame,
    decompileGameData,
    extractAndValidateISO,
  } from "$lib/setup";
  // components
  import Progress from "./Progress.svelte";
  // constants
  import { SupportedGame } from "../../lib/constants";
  import { isInstalling } from "../../stores/InstallStore";

  // TODO - set status from inside each install step function
  // TODO: MOVE THIS FUNCTION TO THE LIB DIR AND DELETE IMPORTS
  async function installProcess() {
    let isoPath: string | string[];
    isInstalling.update(() => true);
    try {
      await clearInstallLogs(SupportedGame.Jak1);
      isoPath = await filePrompt();
      await extractAndValidateISO(isoPath);
      await decompileGameData(isoPath);
      await compileGame(isoPath);
      await setInstallStatus(SupportedGame.Jak1, true);
      isInstalling.update(() => false);
      // NOTE - CHANGED THIS NAVIGATE, BUT I STILL THINK IT CAN BE BETTER
      navigate(0);
      return;
    } catch (err) {
      // TODO - MAKE SURE FUNCTIONS USING ENUMS WHEN THROWING ERRORS
      // InstallStatus.update(() => err.message);
      isInstalling.update(() => false);
      return false;
    }
  }
</script>

<div class="content">
  <Progress />
  <div style="text-align:center">
    {#if !$isInstalling}
      <button class="btn" on:click={async () => await installProcess()}>
        Setup
      </button>
    {/if}
  </div>
</div>
