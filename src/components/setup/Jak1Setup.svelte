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

  let currentStatus = {};
  const setStatus = (status) => (currentStatus = status);

  async function areRequirementsMet() {
    try {
      setStatus(SETUP_SUCCESS.checkCompatible);
      await isAVXSupported();
      setStatus(SETUP_SUCCESS.avxSupported);
      await isOpenGLVersionSupported("4.3");
      setStatus(SETUP_SUCCESS.openGLSupported);
      return true;
    } catch (err) {
      // TODO - if they aren't met, it would be nice to display which ones aren't
      setStatus({ status: err.message, percent: -1 });
      return false;
    }
  }

  // TODO - set status from inside each install step function
  async function installProcess() {
    let isoPath;
    try {
      await clearInstallLogs(SUPPORTED_GAME.Jak1);
      setStatus(SETUP_SUCCESS.awaitingISO);
      isoPath = await filePrompt();
      setStatus(SETUP_SUCCESS.extractingISO);
      await extractAndValidateISO(isoPath);
      setStatus(SETUP_SUCCESS.decompiling);
      await decompileGameData(isoPath);
      setStatus(SETUP_SUCCESS.compiling);
      await compileGame(isoPath);
      setStatus(SETUP_SUCCESS.ready);
      await setInstallStatus(SUPPORTED_GAME.Jak1, true);
      navigate("/", { replace: true });
    } catch (err) {
      console.log(err.message);
      setStatus({ status: err.message, percent: -1 });
      return false;
    }
  }
</script>

<div class="content">
  <!-- TODO - DONT INCLUDE REQUIREMENTS MET IN PROGRESS BAR -->
  <Progress step={currentStatus} />
  <div style="text-align:center">
    {#await areRequirementsMet() then requirementsMet}
      {#if requirementsMet && (currentStatus.status === SETUP_SUCCESS.openGLSupported.status || currentStatus.status === SETUP_ERROR.noISO.status)}
        <button class="btn" on:click={async () => await installProcess()}>
          Setup
        </button>
      {/if}
    {/await}
  </div>
</div>
