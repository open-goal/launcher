<script>
  import { onMount } from "svelte";
  import { Link } from "svelte-routing";
  import { filePrompt } from "$lib/utils/file";
  import {
    compileGame,
    decompileGameData,
    extractAndValidateISO,
    isAVXSupported,
    isOpenGLVersionSupported,
  } from "$lib/setup";
  import { SupportedGame, setInstallStatus } from "$lib/config";
  import {
    appendToInstallLog,
    appendToInstallErrorLog,
    clearInstallLogs,
  } from "$lib/utils/file";

  import Progress from "../../components/Progress.svelte";
  import { message } from "@tauri-apps/api/dialog";

  let setupInProgress = false;
  let isoPath;

  const progressSteps = {
    avxSupported: { status: "AVX SUPPORTED", percent: 10 },
    openGLSupported: { status: "OPENGL SUPPORTED", percent: 20 },
    checkCompatible: { status: "Checking Compatibility", percent: 0 },
    awaitingISO: { status: "Awaiting ISO File", percent: 20 },
    extractingISO: {
      status: "Extracting and Validating ISO contents",
      percent: 40,
    },
    decompiling: { status: "Decompiling the game", percent: 60 },
    compiling: { status: "Compiling the game", percent: 80 },
    ready: { status: "Ready to Play!", percent: 100 },
  };

  const progressErrors = {
    noISO: { status: "No ISO File Selected!", percent: -1 },
  };

  let currentStatus = {};

  async function areRequirementsMet() {
    const res = await Promise.resolve()
      .then(async () => await isAVXSupported())
      .then(() => (currentStatus = progressSteps.avxSupported))
      .then(async () => await isOpenGLVersionSupported("4.3"))
      .then(() => (currentStatus = progressSteps.openGLSupported))
      .catch((err) => {
        currentStatus = { status: err.message, percent: -1 };
        console.error(err);
      });

    return res;
  }

  async function installProcess() {
    await clearInstallLogs(SupportedGame.Jak1);
    currentStatus = progressSteps.awaitingISO;
    const res = await Promise.resolve()
      .then(async () => (isoPath = await filePrompt()))
      .then(async () => {
        currentStatus = progressSteps.extractingISO;
        await extractAndValidateISO(isoPath);
      })
      .then(async () => {
        currentStatus = progressSteps.decompiling;
        await decompileGameData(isoPath);
      })
      .then(async () => {
        currentStatus = progressSteps.compiling;
        await compileGame(isoPath);
      })
      .then(async () => {
        currentStatus = progressSteps.ready;
        await setInstallStatus(SupportedGame.Jak1, true);
        await message("READY TO PLAY");
      })
      .catch((err) => {
        console.error(err);
        currentStatus = { status: err.message, percent: -1 };
      });

    return res;
  }

  onMount(async () => {
    currentStatus = progressSteps.checkCompatible;
    // in the future i want to save the requirements met in the settings.json store file so it doesnt need to be run every time
    // then the requirements met function can check against the store data to avoid running the external bins each time
    await areRequirementsMet();
    await installProcess();
  });

  function onClickBrowse() {
    installProcess();
  }
  // function handleError(output) {
  //   if (output.code === 0) {
  //     return;
  //   }
  //   switch (output.code) {
  //     case 4000:
  //       installErrors = [
  //         ...installErrors,
  //         {
  //           title: `can't locate ELF in ISO's contents`,
  //           description: "unable to determine the version of the game",
  //         },
  //       ];
  //       break;
  //     case 4001:
  //       installErrors = [
  //         ...installErrors,
  //         {
  //           title: `Unsupported serial`,
  //           description:
  //             "ISO containing an unsupported game serial or version was provided",
  //         },
  //       ];
  //       break;
  //     case 4002:
  //     case 4010:
  //     case 4011:
  //       installErrors = [
  //         ...installErrors,
  //         {
  //           title: `Unsupported game version`,
  //           description:
  //             "ISO contains files that are for an unsupported version, were modified from the original, or is an incomplete dump",
  //         },
  //       ];
  //       break;
  //     case 4020:
  //       installErrors = [
  //         ...installErrors,
  //         {
  //           title: `Unexpected Extraction Result`,
  //           description:
  //             "The extracted ISO's contents were not as expected, installation cannot proceed",
  //         },
  //       ];
  //       break;
  //     default:
  //       installErrors = [
  //         ...installErrors,
  //         {
  //           title: `Unexpected Error Code ${output.code}`,
  //           description:
  //             "An unexpected error occurred during installation, check logs",
  //         },
  //       ];
  //   }
  // }

  // async function appendLogs(output) {
  //   const separator = `----${installSteps[currStep].text}----\n`;
  //   await appendToInstallLog(
  //     SupportedGame.Jak1,
  //     "\n" + separator + output.stdout
  //   );
  //   await appendToInstallErrorLog(
  //     SupportedGame.Jak1,
  //     "\n" + separator + output.stderr
  //   );
  //   if (installSteps[currStep] != undefined) {
  //     installSteps[currStep].logs += "\n" + separator + output.stdout;
  //     installSteps[currStep].errorLogs += output.stderr;
  //   }
  // }

  // async function installProcess() {
  //   await clearInstallLogs(SupportedGame.Jak1);
  //   // Reset experience
  //   installErrors = [];
  //   for (let i = 0; i < installSteps.length; i++) {
  //     installSteps[i].logs = "";
  //     installSteps[i].errorLogs = "";
  //     installSteps[i].status = InstallationStatus.Pending;
  //   }
  //   currStep = 0;
  //   setupStarted = true;
  //   setupInProgress = true;
  //   installSteps[currStep].status = InstallationStatus.InProgress;
  //   let output = await extractAndValidateISO(isoPath);
  //   finishStep(output);
  //   if (output.code === 0) {
  //     console.log("[OpenGOAL]: Extraction and Validation Completed");
  //     output = await decompileGameData(isoPath);
  //     finishStep(output);
  //   }
  //   if (output.code === 0) {
  //     console.log("[OpenGOAL]: Decompilation Completed");
  //     output = await compileGame(isoPath);
  //     finishStep(output);
  //   }
  //   if (output.code === 0) {
  //     console.log("[OpenGOAL]: Compilation Completed");
  //     await setInstallStatus(SupportedGame.Jak1, true);
  //     navigate("/jak1", { replace: true });
  //   }
  //   setupInProgress = false;
  //   isoPath = undefined;
  // }
</script>

<div class="content">
  <Progress step={currentStatus} />
  <div style="text-align:center">
    {#if currentStatus.status === "No ISO File Selected!"}
      <button class="btn" disabled={setupInProgress} on:click={onClickBrowse}>
        Browse for ISO
      </button>
    {/if}
    <Link to="/jak1">
      <button class="btn">Cancel</button>
    </Link>
    <span id="filePathLabel" />
  </div>
  <div class="row">
    <details>
      <summary>Installation Logs</summary>
      <div class="logContainer" />
    </details>
  </div>
</div>
