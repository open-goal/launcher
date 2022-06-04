<script>
  import { navigate } from "svelte-routing";
  import { filePrompt } from "$lib/utils/file";
  import { setInstallStatus } from "$lib/config";
  import { clearInstallLogs } from "$lib/utils/file";
  import {
    compileGame,
    decompileGameData,
    extractAndValidateISO,
    isAVXSupported,
    isOpenGLVersionSupported,
  } from "$lib/setup";
  // components
  import Progress from "./Progress.svelte";
  // constants
  import { SETUP_ERROR, SUPPORTED_GAME } from "../../lib/constants";
  import { InstallStatus, isInstalling } from "../../stores/InstallStore";

  async function areRequirementsMet() {
    try {
      await isAVXSupported();
      await isOpenGLVersionSupported("4.3");
      return true;
    } catch (err) {
      // TODO - MAKE SURE FUNCTIONS USING ENUMS WHEN THROWING ERRORS
      // InstallStore.update(err.message);
      return false;
    }
  }

  // TODO - set status from inside each install step function
  async function installProcess() {
    let isoPath;
    isInstalling.update(() => true);
    try {
      await clearInstallLogs(SUPPORTED_GAME.Jak1);
      isoPath = await filePrompt();
      await extractAndValidateISO(isoPath);
      await decompileGameData(isoPath);
      await compileGame(isoPath);
      await setInstallStatus(SUPPORTED_GAME.Jak1, true);
      // TODO - RETHINK THIS NAVIGATE LOGIC
      navigate("/", { replace: true });
      isInstalling.update(() => false);
    } catch (err) {
      // TODO - MAKE SURE FUNCTIONS USING ENUMS WHEN THROWING ERRORS
      // InstallStatus.update(() => err.message);
      isInstalling.update(() => false);
      return false;
    }
  }
</script>

<div class="content">
  <!-- TODO - DONT INCLUDE REQUIREMENTS MET IN PROGRESS BAR -->
  <Progress />
  <div style="text-align:center">
    <!-- TODO - STOP THIS FROM RETRIGGER REQUIREMENTS CHECK ON PAGE CHANGE -->
    {#if !$isInstalling}
      {#await areRequirementsMet() then requirementsMet}
        {#if requirementsMet}
          <button class="btn" on:click={async () => await installProcess()}>
            Setup
          </button>
        {/if}
      {/await}
    {/if}
  </div>
</div>
