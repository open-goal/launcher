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
  import {
    SETUP_SUCCESS,
    SETUP_ERROR,
    SUPPORTED_GAME,
  } from "../../lib/constants";
  import InstallStore from "../../stores/InstallStore";

  let isInstalling;
  InstallStore.subscribe((data) => {
    [{ isInstalling }] = data;
  });

  async function areRequirementsMet() {
    try {
      await isAVXSupported();
      await isOpenGLVersionSupported("4.3");
      return true;
    } catch (err) {
      // TODO - MAKE SURE FUNCTIONS USING ENUMS WHEN THROWING ERRORS
      // InstallStore.set([{ currentStatus: err.message }]);
      return false;
    }
  }

  // TODO - set status from inside each install step function
  async function installProcess() {
    let isoPath;
    InstallStore.set([{ isInstalling: true }]);
    try {
      await clearInstallLogs(SUPPORTED_GAME.Jak1);
      isoPath = await filePrompt();
      await extractAndValidateISO(isoPath);
      await decompileGameData(isoPath);
      await compileGame(isoPath);
      await setInstallStatus(SUPPORTED_GAME.Jak1, true);
      navigate("/", { replace: true });
      InstallStore.set([{ isInstalling: false }]);
    } catch (err) {
      // TODO - MAKE SURE FUNCTIONS USING ENUMS WHEN THROWING ERRORS
      // InstallStore.set([{ currentStatus: err.message }]);
      InstallStore.set([{ isInstalling: false }]);
      return false;
    }
  }
</script>

<div class="content">
  <!-- TODO - DONT INCLUDE REQUIREMENTS MET IN PROGRESS BAR -->
  <Progress />
  <div style="text-align:center">
    {#await areRequirementsMet() then requirementsMet}
      {#if requirementsMet && !isInstalling}
        <button class="btn" on:click={async () => await installProcess()}>
          Setup
        </button>
      {/if}
    {/await}
  </div>
</div>
