<script>
  import {
    appendToInstallLog,
    appendToInstallErrorLog,
    clearInstallLogs,
  } from "$lib/utils/file";

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

<div class="row">
  <details>
    <summary>Installation Logs</summary>
    <div class="logContainer" />
  </details>
</div>
