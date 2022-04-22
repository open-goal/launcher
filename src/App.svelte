<script>
  // Assets
  import bgVideo from "$assets/videos/background.webm";
  // Other Imports
  import { onMount } from "svelte";
  import { Router, Route } from "svelte-routing";
  import Jak1 from "/src/routes/Jak1.svelte";
  import Sidebar from "/src/components/Sidebar.svelte";
  import { initConfig } from "$lib/config";
  import { isInDebugMode } from "$lib/setup";

  export let url = "";

  let revokeSpecificActions = false;

  // Events
  onMount(async () => {
    await initConfig();
  });

  if (!isInDebugMode()) {
    revokeSpecificActions = true;
    // Disable Right Click
    document.addEventListener("contextmenu", (event) => {
      if (revokeSpecificActions) {
        event.preventDefault();
      }
    });
    // Disable Refreshing (F5 / Ctrl+R)
    document.addEventListener("keydown", (e) => {
      if (e.code == "F5") {
        if (revokeSpecificActions) {
          e.preventDefault();
        }
      }
      if (e.code == "KeyR" && e.ctrlKey) {
        if (revokeSpecificActions) {
          e.preventDefault();
        }
      }
      // super secret keybind to reverse the above so we can debug a release version
      // Shift+Ctrl F12
      if (e.code == "F12" && e.ctrlKey && e.shiftKey) {
        revokeSpecificActions = false;
        console.log("hello world");
      }
    });
  }
</script>

<Router {url}>
  <main>
    <div class="video-container">
      <div class="overlay" />
      <video id="backgroundVideo" src={bgVideo} autoplay muted loop />
    </div>
    <div class="container">
      <Sidebar />
      <div id="main">
        <Route path="/" component={Jak1} />
        <Route path="/jak1" component={Jak1} />
      </div>
    </div>
  </main>
</Router>
