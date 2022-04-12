import * as doT from 'dot';

export let sidebarTemplate = doT.template(`
<div class="games">
  <div class="jak-1 nav-item{{? it.jak1Active }} active{{?? true}} disabled{{?}}">
    <a data-tooltip="Jak & Daxter: The Precursor Legacy">
      <img src="src/assets/images/jak-tpl.png">
    </a>
  </div>
  <div class="jak-2 nav-item{{? it.jak2Active }} active{{?? true}} disabled{{?}}">
    <a data-tooltip="Jak 2 - Not Available!">
      <img src="src/assets/images/jak-2.png">
    </a>
  </div>
  <div class="jak-3 nav-item{{? it.jak3Active }} active{{?? true}} disabled{{?}}">
    <a data-tooltip="Jak 3 - Not Available!">
      <img src="src/assets/images/jak-3.png">
    </a>
  </div>
</div>
<div class="spacer"></div>
<div class="controls">
  <div class="console nav-item">
    <a data-tooltip="Toggle Debug Console">
      <i class="bi bi-terminal-fill"></i>
    </a>
  </div>
  <div class="settings nav-item">
    <a data-tooltip="Settings">
      <i class="bi bi-sliders" key="settings"></i>
    </a>
  </div>
</div>
`);
