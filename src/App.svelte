<script>
  // Assets
  import bgVideo from "$assets/videos/background.webm";
  // Other Imports
  import { onMount } from "svelte";
  import { Router, Route } from "svelte-navigator";
  import Jak1 from "./routes/Jak1.svelte";
  import Settings from "./routes/Settings.svelte";
  import Sidebar from "./components/sidebar/Sidebar.svelte";
  import Background from "./components/background/Background.svelte";
  import { initConfig } from "$lib/config";
  import { isInDebugMode } from "$lib/setup";

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

<!-- TODO - Rewrite this to be more concise and simple, reduce nested crap -->
<Router>
  <div class="container">
    <Sidebar />
    <!-- TODO - pass background component current active game -->
    <Background {bgVideo} />
    <div class="test">
      <div id="main">
        <Route path="/" component={Jak1} />
        <Route path="/jak1" component={Jak1} />
        <Route path="/settings" component={Settings} />
      </div>
    </div>
  </div>
</Router>
