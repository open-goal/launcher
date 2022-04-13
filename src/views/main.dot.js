import * as doT from 'dot';

export let mainTemplate = doT.template(`
<div class="flex-center">
  <div class="logo">
    <img id="logo" width="65%" src="src/assets/images/logo.png">
  </div>
  <div id="launcherControls">
  {{? it.gameInstalled }}
    <button class="btn" id="configBtn">CONFIG</button>
    <button class="btn" id="playBtn">PLAY</button>
  {{?? true }}
    <button class="btn" id="setupBtn">SETUP</button>
  {{?}}
  </div>
</div>
`);

export let setupTemplate = doT.template(`
<div class="content">
  <h1>Setup Process</h1>
  <p>Browse for your ISO - this should be a copy you obtain legitimately via dumping your official copy</p>
  <div>
    <button class="btn">Browse for ISO</button>
    File Path
  </div>
  <div>
    <h2>Progress</h2>
    <ul>
      <li>
        <span class="progress-row">
          <div class="loader"></div>
          Extracting ISO
        </span>
      </li>
      <li>
        <span class="progress-row">
          ✅
          Validating Game Data
        </span>
      </li>
      <li>
        <span class="progress-row">
          ❌
          Decompiling Game Data
        </span>
      </li>
      <li>
        <span class="progress-row">
          🕓
          Compiling the Game
        </span>
      </li>
    </ul>
  </div>
  <div class="row">
    <details>
      <summary>Installation Logs</summary>
      <div class="details-bg">
        Stuff
      </div>
    </details>
  </div>
  <button class="btn">Cancel</button>
</div>
`);
