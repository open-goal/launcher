<script>
  import { Link } from "svelte-routing";
</script>

<nav id="sidebar">
  <div class="games">
    <div class="jak-1 nav-item active">
      <Link to="/jak1" data-tooltip="Jak & Daxter: The Precursor Legacy">
        <img src="/src/assets/images/jak-tpl.png">
      </Link>
    </div>
    <div class="jak-2 nav-item">
      <Link to="/jak2" data-tooltip="Jak 2 - Not Available!">
        <img src="/src/assets/images/jak-2.png">
      </Link>
    </div>
    <div class="jak-3 nav-item">
      <Link to="/jak2" data-tooltip="Jak 3 - Not Available!">
        <img src="/src/assets/images/jak-3.png">
      </Link>
    </div>
  </div>
  <div class="spacer"></div>
  <div class="controls">
    <div class="console nav-item">
      <Link to="/console" data-tooltip="Toggle Debug Console">
        <i class="bi bi-terminal-fill"></i>
      </Link>
    </div>
    <div class="settings nav-item">
      <Link to="/settings" data-tooltip="Settings">
        <i class="bi bi-sliders" key="settings"></i>
      </Link>
    </div>
  </div>
</nav>
