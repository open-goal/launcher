// TODO some minimal templating might help - https://olado.github.io/doT/index.html

export const jak1_sidebar = `<div class="games">
<div class="jak-1 nav-item active">
  <a data-tooltip="Jak & Daxter: The Precursor Legacy">
    <img src="src/assets/images/jak-tpl.png">
  </a>
</div>
<div class="jak-2 nav-item">
  <a data-tooltip="Jak 2">
    <img src="src/assets/images/jak-2.png">
  </a>
</div>
<div class="jak-3 nav-item">
  <a data-tooltip="Jak 3">
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
</div>`;

export const jak1_main = `<div class="flex-center">
<div class="logo">
  <img id="logo" width="65%" src="src/assets/images/logo.png">
</div>

<div class="buttons" id="launcherControls"></div>
</div>`;
