import * as doT from 'dot';

export let mainTemplate = doT.template(`
<div class="flex-center">
  <div class="logo">
    <img id="logo" width="65%" src="src/assets/images/logo.png">
  </div>
  <div class="buttons" id="launcherControls">
  {{? it.gameInstalled }}
    <button id="configBtn">CONFIG</button>
    <button id="playBtn">PLAY</button>
  {{?? true }}
    <button id="setupBtn">SETUP</button>
  {{?}}
  </div>
</div>
`);
