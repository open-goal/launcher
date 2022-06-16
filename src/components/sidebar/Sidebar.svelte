<script>
  import logoJak1 from "$assets/images/jak-tpl.webp";
  import logoJak2 from "$assets/images/jak-2.webp";
  import logoJak3 from "$assets/images/jak-3.webp";
  import { Link } from "svelte-navigator";
  import "./sidebar.css";
</script>

<nav id="sidebar">
  <div class="games">
    <div class="jak-1 nav-item active">
      <Link to="jak1" data-tooltip="Jak & Daxter: The Precursor Legacy">
        <img src={logoJak1} alt="Jak - TPL" />
      </Link>
    </div>
    <div class="jak-2 nav-item disabled">
      <Link to="" data-tooltip="Jak 2 - Not Available!">
        <img src={logoJak2} alt="Jak 2" />
      </Link>
    </div>
    <div class="jak-3 nav-item disabled">
      <Link to="" data-tooltip="Jak 3 - Not Available!">
        <img src={logoJak3} alt="Jak 3" />
      </Link>
    </div>
  </div>
  <div class="spacer" />
  <div class="controls">
    <div class="console nav-item">
      <Link to="" data-tooltip="Toggle Debug Console">
        <i class="bi bi-terminal-fill" />
      </Link>
    </div>
    <div class="settings nav-item">
      <Link to="settings" data-tooltip="Settings">
        <i class="bi bi-sliders" />
      </Link>
    </div>
  </div>
</nav>
