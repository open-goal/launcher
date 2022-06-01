<script>
  import { message } from "@tauri-apps/api/dialog";
  import { onMount } from "svelte";
  import { Link, navigate } from "svelte-routing";
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

  let isoPath;

  let currentStatus = {};
  const setStatus = (status) => (currentStatus = status);

  async function areRequirementsMet() {
    const res = await Promise.resolve()
      .then(async () => {
        setStatus(SETUP_SUCCESS.checkCompatible);
        await isAVXSupported();
        setStatus(SETUP_SUCCESS.avxSupported);
      })
      .then(async () => {
        await isOpenGLVersionSupported("4.3");
        setStatus(SETUP_SUCCESS.openGLSupported);
      })
      .catch((err) => {
        setStatus({ status: err.message, percent: -1 });
        console.error(err);
      });

    return res;
  }

  async function installProcess() {
    await clearInstallLogs(SUPPORTED_GAME.Jak1);
    const res = await Promise.resolve()
      .then(async () => {
        setStatus(SETUP_SUCCESS.awaitingISO);
        isoPath = await filePrompt();
      })
      .then(async () => {
        setStatus(SETUP_SUCCESS.extractingISO);
        await extractAndValidateISO(isoPath);
      })
      .then(async () => {
        setStatus(SETUP_SUCCESS.decompiling);
        await decompileGameData(isoPath);
      })
      .then(async () => {
        setStatus(SETUP_SUCCESS.compiling);
        await compileGame(isoPath);
      })
      .then(async () => {
        setStatus(SETUP_SUCCESS.ready);
        await setInstallStatus(SUPPORTED_GAME.Jak1, true);
        navigate("/", { replace: true });
      })
      .catch((err) => {
        console.error(err);
        setStatus({ status: err.message, percent: -1 });
      });

    return res;
  }
</script>

<div class="content">
  <Progress step={currentStatus} />
  <div style="text-align:center">
    <!-- TODO - !requirementsMet then dont render the setup button  -->
    {#if currentStatus.status === undefined || currentStatus.status === SETUP_ERROR.noISO.status}
      <button class="btn" on:click={async () => await installProcess()}>
        Setup
      </button>
    {/if}
    <!-- <Link to="/jak1">
      <button class="btn">Cancel</button>
    </Link> -->
  </div>
</div>
