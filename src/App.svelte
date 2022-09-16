<script>
  // Assets
  import bgVideo from "$assets/videos/background.mp4";
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
  import Header from "./components/header/Header.svelte";
  import Faq from "./routes/FAQ.svelte";
  import Textures from "./routes/Textures.svelte";
  import Update from "./routes/Update.svelte";

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

<Router>
  <div class="container h-screen max-w-none">
    <!-- TODO - pass background component current active game -->
    <Background {bgVideo} />
    <Header />
    <Sidebar />
    <Route path="/" component={Game} primary={false} let:params />
    <Route path="/:game_name" component={Game} primary={false} let:params />
    <Route path="/settings" component={Settings} primary={false} />
    <Route path="/faq" component={Faq} primary={false} />
    <Route path="/textures" component={Textures} primary={false} />
    <Route path="/update" component={Update} primary={false} />
  </div>
</Router>
