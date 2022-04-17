<script>
  import { Link, navigate } from "svelte-routing";
  import { filePrompt } from "/src/lib/utils/file";
  import {
    compileGame,
    decompileGameData,
    extractISO,
    validateGameData,
  } from "/src/lib/setup";
  import { SupportedGame, setInstallStatus } from "/src/lib/config";
  import { InstallationStatus } from "/src/lib/setup";
  import {
    appendToInstallLog,
    appendToInstallErrorLog,
    clearInstallLogs,
  } from "/src/lib/utils/file";

  let setupInProgress = false;
  let isoPath = undefined;

  let currStep = 0;
  let installSteps = [
    {
      status: InstallationStatus.Pending,
      text: "Extracting ISO",
      logs: "",
      errorLogs: "",
    },
    {
      status: InstallationStatus.Pending,
      text: "Validating Game Data",
      logs: "",
      errorLogs: "",
    },
    {
      status: InstallationStatus.Pending,
      text: "Decompiling the Game",
      logs: "",
      errorLogs: "",
    },
    {
      status: InstallationStatus.Pending,
      text: "Compiling the Game",
      logs: "",
      errorLogs: "",
    },
  ];

  function statusIndicator(status) {
    if (status === InstallationStatus.InProgress) {
      return `spinner`;
    } else if (status === InstallationStatus.Success) {
      return "✅";
    } else if (status === InstallationStatus.Failed) {
      return "❌";
    } else {
      return "⏳";
    }
  }

  function finishStep(output) {
    appendLogs(output);
    installSteps[currStep].status =
      output.code === 0
        ? InstallationStatus.Success
        : InstallationStatus.Failed;
    currStep++;
    if (currStep < installSteps.length) {
      installSteps[currStep].status = InstallationStatus.InProgress;
    }
  }

  async function appendLogs(output) {
    const separator = `----${installSteps[currStep].text}----\n`;
    await appendToInstallLog(SupportedGame.Jak1, "\n" + separator + output.stdout);
    await appendToInstallErrorLog(
      SupportedGame.Jak1,
      "\n" + separator + output.stderr
    );
    installSteps[currStep].logs += "\n" + separator + output.stdout;
    installSteps[currStep].errorLogs += output.stderr;
  }

  async function installProcess() {
    await clearInstallLogs(SupportedGame.Jak1);
    // TODO - forbid refreshing
    setupInProgress = true;
    installSteps[currStep].status = InstallationStatus.InProgress;
    let output = await extractISO(isoPath);
    finishStep(output);
    if (output.code === 0) {
      console.log("[OpenGOAL]: Extraction Completed");
      output = await validateGameData(isoPath);
      finishStep(output);
    }
    if (output.code === 0) {
      console.log("[OpenGOAL]: Validation Completed");
      output = await decompileGameData(isoPath);
      finishStep(output);
    }
    if (output.code === 0) {
      console.log("[OpenGOAL]: Decompilation Completed");
      output = await compileGame(isoPath);
      finishStep(output);
    }
    if (output.code === 0) {
      console.log("[OpenGOAL]: Compilation Completed");
      await setInstallStatus(SupportedGame.Jak1, true);
      navigate("/jak1", { replace: true });
    }
  }

  // Events
  async function onClickBrowse() {
    isoPath = await filePrompt();
    installProcess();
  }
</script>

<div class="content">
  <h1>Setup Process</h1>
  <p>Browse for your ISO - Obtained by dumping your own legitimate copy</p>
  <div>
    <button class="btn" on:click={onClickBrowse}>Browse for ISO</button>
    {#if isoPath}
      {isoPath}
    {/if}
    <span id="filePathLabel" />
  </div>
  {#if !setupInProgress}
  <div class="row">
    <Link to="/jak1">
      <button class="btn">Cancel</button>
    </Link>
  </div>
  {:else}
  <div>
    <h2>Progress</h2>
    <ul>
      {#each installSteps as step}
        <li>
          <span class="progress-row">
            {#if step.status === InstallationStatus.InProgress}
              <div class="loader" />
            {:else}
              {statusIndicator(step.status)}
            {/if}
            {step.text}
          </span>
        </li>
      {/each}
    </ul>
  </div>
  <div class="row">
    <details>
      <summary>Installation Logs</summary>
      <div class="logContainer">
        {#each installSteps as step}
          {#if step.logs !== ""}
            {step.logs}
          {/if}
          {#if step.errorLogs !== ""}
            <div class="errorLogs">
              {step.errorLogs}
            </div>
          {/if}
        {/each}
      </div>
    </details>
  </div>
  {/if}
</div>
