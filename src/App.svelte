<script>
  import { Router, Link, Route } from "svelte-routing";
  import Jak1 from "/src/routes/Jak1.svelte";
  import Jak1_Setup from "/src/routes/setup/Jak1.svelte";
  import Settings from "/src/routes/Settings.svelte";
  import Sidebar from "/src/components/Sidebar.svelte";

  export let url = "";

  // TODO - setup tauri app / initialize config
</script>

<Router url={url}>
  <main>
    <div class="video-container">
      <div class="overlay"></div>
      <video id="backgroundVideo" src="/src/assets/videos/background.mp4" autoplay muted loop></video>
    </div>
    <div class="container">
      <Sidebar></Sidebar>
      <div id="main">
        <Route path="/" component={Jak1} />
        <Route path="/jak1" component={Jak1} />
        <Route path="/setup/jak1" component={Jak1_Setup} />
      </div>
    </div>
  </main>
</Router>

