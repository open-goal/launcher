<script>
  // Other Imports
  import { onMount } from "svelte";
  import { Router, Route } from "svelte-navigator";
  import Game from "./routes/Game.svelte";
  import Settings from "./routes/Settings.svelte";
  import Sidebar from "./components/sidebar/Sidebar.svelte";
  import Background from "./components/background/Background.svelte";
  import Header from "./components/header/Header.svelte";
  import Textures from "./routes/Textures.svelte";
  import Update from "./routes/Update.svelte";
  import GameInProgress from "./components/games/GameInProgress.svelte";
  import { isInDebugMode } from "$lib/utils/common";
  import { Toast } from "flowbite-svelte";
  import Help from "./routes/Help.svelte";
  import { toastStore } from "$lib/stores/ToastStore";
  import { isLoading } from "svelte-i18n";

  let revokeSpecificActions = false;

  // Events
  onMount(async () => {
    // Temporary fix related to https://github.com/open-goal/launcher/issues/110
    // TODO - this doesn't feel required anymore after i fixed the window switching
    // but let's keep it for now because im paranoid about the issue cropping up again...
    if (window.sessionStorage.getItem("refreshHack") !== "true") {
      location.reload();
      window.sessionStorage.setItem("refreshHack", "true");
    }
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
  <div class={`container h-screen max-w-none flex flex-col bg-black`}>
    {#if !$isLoading}
      <Background />
      <Header />
      <div class="flex h-full z-10">
        <Sidebar />
        <div id="content" class="basis-9/10">
          <Route path="/" component={Game} primary={false} let:params />
          <Route
            path="/:game_name"
            component={Game}
            primary={false}
            let:params
          />
          <Route
            path="/jak2"
            component={GameInProgress}
            primary={false}
            let:params
          />
          <Route
            path="/settings/:tab"
            component={Settings}
            primary={false}
            let:params
          />
          <Route path="/faq" component={Help} primary={false} />
          <Route path="/textures" component={Textures} primary={false} />
          <Route path="/update" component={Update} primary={false} />
        </div>
      </div>
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
    {/if}
  </div>
</Router>
