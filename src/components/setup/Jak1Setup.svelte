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
  import { SETUP_ERROR, SupportedGame } from "../../lib/constants";
  import { InstallStatus, isInstalling } from "../../stores/InstallStore";

  // TODO - set status from inside each install step function
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
      // NOTE - CHANGED THIS NAVIGATE, BUT I STILL THINK IT CAN BE BETTER
      isInstalling.update(() => false);
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
  <!-- TODO - EXCLUDE REQUIREMENTS MET FROM PROGRESS BAR -->
  <Progress />
  <div style="text-align:center">
    <!-- TODO - STOP THIS FROM RETRIGGER REQUIREMENTS CHECK ON PAGE CHANGE -->
    {#if !$isInstalling}
      <button class="btn" on:click={async () => await installProcess()}>
        Setup
      </button>
    {/if}
  </div>
</div>
