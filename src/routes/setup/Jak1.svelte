<script>
  import { onMount } from "svelte";
  import { Link, navigate } from "svelte-routing";
  import { filePrompt } from "/src/lib/utils/file";
  import {
    compileGame,
    decompileGameData,
    extractISO,
    validateGameData,
    isAVXSupported,
    isOpenGLVersionSupported,
  } from "/src/lib/setup";
  import { SupportedGame, setInstallStatus } from "/src/lib/config";
  import { InstallationStatus, RequirementStatus } from "/src/lib/setup";
  import {
    appendToInstallLog,
    appendToInstallErrorLog,
    clearInstallLogs,
  } from "/src/lib/utils/file";

  let setupInProgress = false;
  let isoPath = undefined;

  let requirementChecks = [
    {
      status: RequirementStatus.Checking,
      text: `CPU Supports&nbsp;<a href="https://en.wikipedia.org/wiki/Advanced_Vector_Extensions"><strong>AVX or AVX2</strong></a>`,
      check: async () => await isAVXSupported(),
    },
    {
      status: RequirementStatus.Checking,
      text: `GPU Supports&nbsp;OpenGL&nbsp;<span class="orange-text"><strong>4.3</strong></span>`,
      check: async () => await isOpenGLVersionSupported("4.3"),
    },
  ];

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

  // TODO - move this to the enum
  function statusIndicator(status) {
    if (
      status === InstallationStatus.InProgress ||
      status === RequirementStatus.Checking
    ) {
      return `<div class="loader" />`;
    } else if (
      status === InstallationStatus.Success ||
      status === RequirementStatus.Met
    ) {
      return "✅";
    } else if (
      status === InstallationStatus.Failed ||
      status === RequirementStatus.Failed
    ) {
      return "❌";
    } else if (status === RequirementStatus.Unknown) {
      return "⚠️";
    } else {
      return "⏳";
    }
  }

  function areRequirementsMet(checks) {
    for (let i = 0; i < checks.length; i++) {
      if (
        checks[i].status !== RequirementStatus.Met &&
        checks[i].status !== RequirementStatus.Unknown
      ) {
        return false;
      }
    }
    return true;
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
    await appendToInstallLog(
      SupportedGame.Jak1,
      "\n" + separator + output.stdout
    );
    await appendToInstallErrorLog(
      SupportedGame.Jak1,
      "\n" + separator + output.stderr
    );
    installSteps[currStep].logs += "\n" + separator + output.stdout;
    installSteps[currStep].errorLogs += output.stderr;
  }

  async function installProcess() {
    await clearInstallLogs(SupportedGame.Jak1);
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
  onMount(async () => {
    for (let i = 0; i < requirementChecks.length; i++) {
      requirementChecks[i].status = await requirementChecks[i].check();
    }
  });

  async function onClickBrowse() {
    isoPath = await filePrompt();
    installProcess();
  }
</script>

<div class="content">
  <h1>Setup Process</h1>
  <h2>Minimum Requirements</h2>
  <ul class="no-decoration">
    {#each requirementChecks as check}
      <li>
        <span class="progress-row">
          {@html statusIndicator(check.status)}
          {@html check.text}
        </span>
        {#if check.status === RequirementStatus.Unknown}
          <ul class="no-decoration">
            <li>Unable to determine this requirement</li>
          </ul>
        {/if}
      </li>
    {/each}
  </ul>
  {#if areRequirementsMet(requirementChecks)}
    <p>Browse for your ISO - Obtained by dumping your own legitimate copy</p>
    <div>
      <button class="btn" disabled={setupInProgress} on:click={onClickBrowse}>Browse for ISO</button>
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
        <ul class="no-decoration">
          {#each installSteps as step}
            <li>
              <span class="progress-row">
                {@html statusIndicator(step.status)}
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
  {/if}
</div>
