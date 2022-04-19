<script>
  // Assets
  import bgVideo from '$assets/videos/background.webm';
  // Other Imports
  import { onMount } from "svelte";
  import { Router, Route } from "svelte-routing";
  import Jak1 from "/src/routes/Jak1.svelte";
  import Jak1_Setup from "/src/routes/setup/Jak1.svelte";
  import Sidebar from "/src/components/Sidebar.svelte";
  import { initConfig } from "$lib/config";
  import { isInDebugMode } from "$lib/setup";

  export let url = "";

  // Events
  onMount(async () => {
    await initConfig();
  });

  if (!isInDebugMode()) {
    // Disable Right Click
    document.addEventListener("contextmenu", (event) => event.preventDefault());
    // Disable Refreshing (F5 / Ctrl+R)
    document.addEventListener("keydown", (e) => {
      if (e.code == "F5") {
        e.preventDefault();
      }
      if (e.code == "KeyR" && e.ctrlKey) {
        e.preventDefault();
      }
    });
  }
</script>

<Router {url}>
  <main>
    <div class="video-container">
      <div class="overlay" />
      <video
        id="backgroundVideo"
        src={bgVideo}
        autoplay
        muted
        loop
      />
    </div>
    <div class="container">
      <Sidebar />
      <div id="main">
        <Route path="/" component={Jak1} />
        <Route path="/jak1" component={Jak1} />
        <Route path="/setup/jak1" component={Jak1_Setup} />
      </div>
    </div>
  </main>
</Router>

