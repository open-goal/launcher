<script>
  import { message } from "@tauri-apps/api/dialog";
  import { onMount } from "svelte";
  import { Link } from "svelte-routing";
  import { filePrompt } from "$lib/utils/file";
  import { SupportedGame, setInstallStatus } from "$lib/config";
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
  import { SETUP_SUCCESS, SETUP_ERROR } from "../../lib/constants";

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
    await clearInstallLogs(SupportedGame.Jak1);
    currentStatus = SETUP_SUCCESS.awaitingISO;
    const res = await Promise.resolve()
      .then(async () => (isoPath = await filePrompt()))
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
        await setInstallStatus(SupportedGame.Jak1, true);
        await message("READY TO PLAY");
      })
      .catch((err) => {
        console.error(err);
        setStatus({ status: err.message, percent: -1 });
      });

    return res;
  }

  onMount(async () => {
    // in the future i want to save the requirements met in the settings.json store file so it doesnt need to be run every time
    // then the requirements met function can check against the store data to avoid running the external bins each time
    await areRequirementsMet();
    await installProcess();
  });

  function onClickBrowse() {
    installProcess();
  }
</script>

<div class="content">
  <Progress step={currentStatus} />
  <div style="text-align:center">
    {#if currentStatus.status === "No ISO File Selected!"}
      <button class="btn" on:click={onClickBrowse}> Browse for ISO </button>
    {/if}
    <Link to="/jak1">
      <button class="btn">Cancel</button>
    </Link>
  </div>
</div>
