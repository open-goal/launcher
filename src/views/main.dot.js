import * as doT from 'dot';

export let mainTemplate = doT.template(`
<div class="flex-center">
  <div class="logo">
    <img id="logo" width="65%" src="src/assets/images/logo.png">
  </div>
  <div id="launcherControls">
  {{? it.gameInstalled }}
    <button class="btn lg" id="configBtn" disabled>Config</button>
    <button class="btn lg" id="playBtn">Play</button>
  {{?? true }}
    <button class="btn lg" id="setupBtn">Setup</button>
  {{?}}
  </div>
</div>
`);

export let setupTemplate = doT.template(`
<div class="content">
  <h1>Setup Process</h1>
  <p>Browse for your ISO - this should be a copy you obtain legitimately via dumping your official copy</p>
  <div>
    <button class="btn" id="browseForIsoBtn">Browse for ISO</button>
    <span id="filePathLabel"></span>
  </div>
  <div>
    <h2>Progress</h2>
    <ul>
      <li>
        <span class="progress-row" id="progressExtract">
          ⏳ Extracting ISO
        </span>
      </li>
      <li>
        <span class="progress-row" id="progressValidating">
          ⏳ Validating Game Data
        </span>
      </li>
      <li>
        <span class="progress-row" id="progressDecompile">
          ⏳ Decompiling Game Data
        </span>
      </li>
      <li>
        <span class="progress-row" id="progressCompile">
          ⏳ Compiling the Game
        </span>
      </li>
    </ul>
  </div>
  <div class="row">
    <details>
      <summary>Installation Logs</summary>
      <div class="details-bg" id="installLogs"></div>
    </details>
  </div>
  <button class="btn" id="cancelBtn" disabled>Cancel</button>
</div>
`);
