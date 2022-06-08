<script>
  import {
    appendToInstallLog,
    appendToInstallErrorLog,
    clearInstallLogs,
  } from "$lib/utils/file";

  import { SupportedGame } from "$lib/constants";

  let installErrors;
  let installSteps = [];

  async function appendLogs(output) {
    // const separator = `----${installSteps[currStep].text}----\n`;
    // await appendToInstallLog(
    //   SupportedGame.Jak1,
    //   "\n" + separator + output.stdout
    // );
    // await appendToInstallErrorLog(
    //   SupportedGame.Jak1,
    //   "\n" + separator + output.stderr
    // );
    // if (installSteps[currStep] != undefined) {
    //   installSteps[currStep].logs += "\n" + separator + output.stdout;
    //   installSteps[currStep].errorLogs += output.stderr;
    // }
  }
</script>

<div class="row">
  <details>
    <summary>Installation Logs</summary>
    <div class="logContainer" />
  </details>
</div>
