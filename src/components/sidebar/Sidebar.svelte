<script>
  import logoJak1 from "$assets/images/jak-tpl.webp";
  import logoJak2 from "$assets/images/jak-2.webp";
  import logoJak3 from "$assets/images/jak-3.webp";
  import { Link } from "svelte-navigator";
</script>

<nav id="sidebar">
  <div class="games">
    <div class="jak-1 nav-item active">
      <Link to="/jak1" data-tooltip="Jak & Daxter: The Precursor Legacy">
        <img src={logoJak1} alt="Jak - TPL" />
      </Link>
    </div>
    <div class="jak-2 nav-item disabled">
      <!-- <Link to="no" data-tooltip="Jak 2"> -->
        <img src={logoJak2} alt="Jak 2" />
      <!-- </Link> -->
    </div>
    <div class="jak-3 nav-item disabled">
      <!-- <Link to="no" data-tooltip="Jak 3"> -->
        <img src={logoJak3} alt="Jak 3" />
      <!-- </Link> -->
    </div>
  </div>
  <div class="spacer" />
  <div class="controls">
    <!-- <div class="console nav-item disabled">
      <Link to="" data-tooltip="Toggle Debug Console">
        <i class="bi bi-terminal-fill" />
      </Link>
    </div> -->
    <div class="settings nav-item">
      <Link to="settings" data-tooltip="Settings">
        <i class="fa-solid fa-gear"></i>
      </Link>
    </div>
  </div>
</nav>

<style>
  #sidebar {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--bg-blue);
  opacity: 75%;
  transition: 500ms ease;
  width: 10vw;
  max-width: 10vw;
}

#sidebar:hover {
  opacity: 100%;
}

.games {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-around;
  height: 50vh;
}

.nav-item {
  margin: 0 5px;
  filter: grayscale(100%);
  transition: 500ms ease;
  opacity: 50%;
}

.nav-item.active {
  filter: grayscale(0%);
}

.nav-item:hover {
  filter: grayscale(0%);
  opacity: 100%;
  cursor: pointer;
}

.nav-item.disabled:hover,
.nav-item.disabled {
  cursor: not-allowed;
  filter: grayscale(100%);
}

.nav-item a::before,
.nav-item a::after {
  --scale: 0;
  position: absolute;
  transform: translateX(55px) scale(var(--scale));
  transition: 200ms ease;
  transform-origin: left center;
}

.nav-item a::before {
  padding: 0.5rem;
  width: max-content;
  background: #333;
  color: white;
  content: attr(data-tooltip);
  border-radius: 0.3rem;
  text-align: center;
}

.nav-item a:hover::before {
  --scale: 1;
}

.nav-item img {
  width: 100%;
}

</style>
