<script>
  // Other Imports
  import { onMount } from "svelte";
  import { Router, Route } from "svelte-navigator";
  import Game from "./routes/Game.svelte";
  import Settings from "./routes/Settings.svelte";
  import Sidebar from "./components/sidebar/Sidebar.svelte";
  import Background from "./components/background/Background.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import Header from "./components/header/Header.svelte";
  import Textures from "./routes/Textures.svelte";
  import Update from "./routes/Update.svelte";
  import GameInProgress from "./components/games/GameInProgress.svelte";
  import { isInDebugMode } from "$lib/utils/common";
  import { Toast } from "flowbite-svelte";
  import Help from "./routes/Help.svelte";
  import { toastStore } from "$lib/stores/ToastStore";

  let revokeSpecificActions = false;

  // Events
  onMount(async () => {
    // TODO - tauri doesn't seem to handle this event being unlistented to properly (go back to closing the window)
    // - need to make an issue
    // For now, we'll just handle all close events ourselves
    await appWindow.listen("tauri://close-requested", async () => {
      // TODO - confirm during an install
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
      }
    });
  }
</script>

<Router>
  <div class="container h-screen max-w-none">
    <Background />
    <Header />
    <Sidebar />
    <Route path="/" component={Game} primary={false} let:params />
    <Route path="/:game_name" component={Game} primary={false} let:params />
    <Route path="/jak2" component={GameInProgress} primary={false} let:params />
    <Route
      path="/settings/:tab"
      component={Settings}
      primary={false}
      let:params
    />
    <Route path="/faq" component={Help} primary={false} />
    <Route path="/textures" component={Textures} primary={false} />
    <Route path="/update" component={Update} primary={false} />
    {#if $toastStore.msg !== undefined}
      <!-- TODO - make these look nice for info/warn/error levels -->
      <Toast
        color="green"
        position="top-right"
        class="top-20"
        divClass="w-full max-w-xs p-2 pl-4"
      >
        {$toastStore.msg}
      </Toast>
    {/if}
  </div>
</Router>
