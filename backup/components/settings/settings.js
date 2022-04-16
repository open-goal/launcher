import "./settings.css";

export const actions = `<div class="actions">
<div class="general nav-item" data-tooltip="General">
  <i class="bi bi-sliders2-vertical" key="general"></i>
</div>
<div class="Files nav-item" data-tooltip="Files">
  <i class="bi bi-folder2" key="files"></i>
</div>
<div class="links nav-item" data-tooltip="Links">
  <i class="bi bi-link-45deg" key="links"></i>
</div>
</div>

<div class="spacer"></div>

<div class="return nav-item" data-tooltip="Back">
  <i class="bi bi-arrow-return-left" key="return"></i>
</div>
`;

export const general_pane = `<div class="pane">
<div id="general">
<ul>
  <li>
    <button>Check for updates</button>
  </li>
  <li>
    <button>Toggle dynamic backgrounds</button>
  </li>
  <li>
    <button>Toggle automatic updates</button>
  </li>
</ul>
</div>
</div>`;

export const files_pane = `<div class="pane">
<div id="files">
<ul>
  <li>
    <label for="install-dir">Current install directory:</label>
    <textarea id="install-dir" name="install-dir" disabled>%/your/install/dir/here</textarea>
  </li>
  <li>
    <Button>Decompile Game</Button>
  </li>
  <li>
    <Button>Build Game</Button>
  </li>
  <li>
    <button>Uninstall</button>
  </li>
</ul>
</div>
</div>`;

export const links_pane = `<div class="pane">
<div id="links">
<a href="https://github.com/open-goal/jak-project" target="_blank" rel="noopener noreferrer">
  <i class="bi bi-github"> OpenGOAL Github Repository</i>
</a>
<br>

<a href="https://open-goal.github.io/#/" target="_blank" rel="noopener noreferrer">
  <i class="bi bi-files"> OpenGOAL Documentation</i>
</a>
<br>

<a href="https://discord.gg/VZbXMHXzWv" target="_blank" rel="noopener noreferrer">
  <i class="bi bi-discord"> Discord Server</i>
</a>
</div>
</div>`;
