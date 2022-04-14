<script>
  import { Link, navigate } from "svelte-routing";
  import { filePrompt } from '/src/lib/utils/file'
  import { compileGame, decompileGameData, extractISO, validateGameData } from '/src/lib/setup';
  import { SupportedGame, setInstallStatus } from '/src/lib/config';

  let setupInProgress = false;
  let isoPath = undefined;

  let progressExtraction = "pending";
  let progressValidation = "pending";
  let progressDecompilation = "pending";
  let progressCompilation = "pending";

  let logs = "";

  // TODO - kinda temporary, instead the status list should be an array
  function statusIndicator(val) {
    if (val === "inprogress") {
      return `<div class="loader"></div>`;
    } else if (val === "success") {
      return "✅";
    } else if (val === "fail") {
      return "❌";
    } else {
      return "⏳";
    }
  }

  function appendLogs(stdout, stderr) {
    // TODO - logs should go to a file as well
    logs += stdout.replaceAll("\n", "<br>") + stderr.replaceAll("\n", "<br>");
  }

  async function installProcess() {
    // TODO - forbid refreshing
    setupInProgress = true;
    progressExtraction = "inprogress";
    let output = await extractISO(isoPath);
    appendLogs(output.stdout, output.stderr);
    progressExtraction = output.code === 0 ? "success" : "fail";
    if (output.code === 0) {
      console.log("[OpenGOAL]: Extraction Completed");
      progressValidation = "inprogress";
      output = await validateGameData(isoPath);
      appendLogs(output.stdout, output.stderr);
      progressValidation = output.code === 0 ? "success" : "fail";
    }
    if (output.code === 0) {
      console.log("[OpenGOAL]: Validation Completed");
      progressDecompilation = "inprogress";
      output = await decompileGameData(isoPath);
      appendLogs(output.stdout, output.stderr);
      progressDecompilation = output.code === 0 ? "success" : "fail";
    }
    if (output.code === 0) {
      console.log("[OpenGOAL]: Decompilation Completed");
      progressCompilation = "inprogress";
      output = await compileGame(isoPath);
      appendLogs(output.stdout, output.stderr);
      progressCompilation = output.code === 0 ? "success" : "fail";
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
    <span id="filePathLabel"></span>
  </div>
  <div>
    <h2>Progress</h2>
    <ul>
      <li>
        <span class="progress-row">
          {statusIndicator(progressExtraction)} Extracting ISO
        </span>
      </li>
      <li>
        <span class="progress-row">
          {statusIndicator(progressValidation)} Validating Game Data
        </span>
      </li>
      <li>
        <span class="progress-row">
          {statusIndicator(progressDecompilation)} Decompiling Game Data
        </span>
      </li>
      <li>
        <span class="progress-row">
          {statusIndicator(progressCompilation)} Compiling the Game
        </span>
      </li>
    </ul>
  </div>
  <div class="row">
    <details>
      <summary>Installation Logs</summary>
      <div class="details-bg"></div>
    </details>
  </div>
  {#if !setupInProgress}
    <Link to="/jak1">
      <button class="btn">Cancel</button>
    </Link>
  {/if}
</div>
