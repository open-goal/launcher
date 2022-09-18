<script>
  // Assets
  import bgVideo from "$assets/videos/background.mp4";
  import bgVideoPoster from "$assets/background/village1.png";
  // Other Imports
  import { onMount } from "svelte";
  import { Router, Route } from "svelte-navigator";
  import Game from "./routes/Game.svelte";
  import Settings from "./routes/Settings.svelte";
  import Sidebar from "./components/sidebar/Sidebar.svelte";
  import Background from "./components/background/Background.svelte";
  import { isInDebugMode } from "$lib/setup/setup";
  import { appWindow } from "@tauri-apps/api/window";
  import { isInstalling } from "./lib/stores/AppStore";
  import { log } from "$lib/utils/log";

  let revokeSpecificActions = false;

  // Events
  onMount(async () => {
    // TODO - tauri doesn't seem to handle this event being unlistented to properly (go back to closing the window)
    // - need to make an issue
    // For now, we'll just handle all close events ourselves
    await appWindow.listen("tauri://close-requested", async () => {
      if ($isInstalling) {
        const confirmed = await confirm(
          "Installation still in progress, are you sure you want to exit?"
        );
        if (confirmed) {
          await appWindow.close();
        }
        return;
      }
      await appWindow.close();
    });
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
        log.info("Hello World - Dev Tools Enabled!");
      }
    });
  }
</script>

<!-- TODO - Rewrite this to be more concise and simple, reduce nested crap -->
<Router>
  <div class="container">
    <Sidebar />
    <!-- TODO - pass background component current active game -->
    <Background {bgVideo} {bgVideoPoster} />
    <div id="main">
      <Route path="/" component={Game} primary={false} let:params />
      <Route path="/:game_name" component={Game} primary={false} let:params />
      <Route path="/settings" component={Settings} primary={false} />
    </div>
  </div>
</Router>
