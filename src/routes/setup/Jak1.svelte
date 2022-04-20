<script>
  import { onMount } from "svelte";
  import { Link, navigate } from "svelte-routing";
  import { filePrompt } from "$lib/utils/file";
  import {
    compileGame,
    decompileGameData,
    extractAndValidateISO,
    isAVXSupported,
    isOpenGLVersionSupported,
  } from "$lib/setup";
  import { SupportedGame, setInstallStatus } from "$lib/config";
  import { InstallationStatus, RequirementStatus } from "$lib/setup";
  import {
    appendToInstallLog,
    appendToInstallErrorLog,
    clearInstallLogs,
  } from "$lib/utils/file";

  import Progress from "../../components/Progress.svelte";

  let setupStarted = false;
  let setupInProgress = false;
  let isoPath;

  // let requirementChecks = [
  //   {
  //     status: RequirementStatus.Checking,
  //     text: `CPU Supports&nbsp;<a href="https://en.wikipedia.org/wiki/Advanced_Vector_Extensions" target="_blank"><strong>AVX or AVX2</strong></a>`,
  //     check: async () => await isAVXSupported(),
  //   },
  //   {
  //     status: RequirementStatus.Checking,
  //     text: `GPU Supports&nbsp;OpenGL&nbsp;<span class="orange-text"><strong>4.3</strong></span>`,
  //     check: async () => await isOpenGLVersionSupported("4.3"),
  //   },
  // ];

  // let currStep = 0;
  // let installSteps = [
  //   {
  //     status: InstallationStatus.Pending,
  //     text: "Extracting and Validating ISO",
  //     logs: "",
  //     errorLogs: "",
  //   },
  //   {
  //     status: InstallationStatus.Pending,
  //     text: "Decompiling the Game",
  //     logs: "",
  //     errorLogs: "",
  //   },
  //   {
  //     status: InstallationStatus.Pending,
  //     text: "Compiling the Game",
  //     logs: "",
  //     errorLogs: "",
  //   },
  // ];

  let installErrors = [];

  async function areRequirementsMet() {
    const res = await Promise.resolve()
      .then(isAVXSupported)
      .then(isOpenGLVersionSupported("4.3"))
      .catch((err) => console.log(err));
    console.log(res);

    return res;
  }

  async function installProcess() {
    await clearInstallLogs(SupportedGame.Jak1);

    const res = await Promise.resolve()
      // .then(filePrompt)
      .then(await extractAndValidateISO(isoPath))
      .then(await decompileGameData(isoPath))
      .then(await compileGame(isoPath))
      .catch((err) => console.log(err));
    console.log(res);

    return res;
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

  // Events

  async function onClickBrowse() {
    isoPath = await filePrompt();
    installProcess();
  }

  const progressSteps = {
    checkCompatible: "Checking compatibility",
    awaitingISO: "Awaiting ISO File",
    extractingISO: "Extracting and Validating ISO contents",
    decompiling: "Decompiling",
    compiling: "Compiling",
    ready: "Ready",
  };

  let status = "";
</script>

<div class="content">
  <Progress status="mike jones" />
  {#if areRequirementsMet()}
    <div>
      <button class="btn" disabled={setupInProgress} on:click={onClickBrowse}>
        Browse for ISO
      </button>
      <Link to="/jak1">
        <button class="btn">Cancel</button>
      </Link>
      <span id="filePathLabel" />
    </div>
    {#if setupStarted}
      <div class="row">
        <details>
          <summary>Installation Logs</summary>
          <div class="logContainer" />
        </details>
      </div>
    {/if}
  {/if}
</div>
