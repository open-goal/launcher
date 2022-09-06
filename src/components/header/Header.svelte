<script>
  import { appWindow } from "@tauri-apps/api/window";
  import icon from "$assets/images/icon.png";
  import { onMount } from "svelte";
  import { launcherConfig } from "$lib/config";
  import { SupportedGame } from "$lib/constants";
  import { getVersion } from "@tauri-apps/api/app";
  let gameVersion;
  let launcherVerison;
  // TODO: get the active game rigged up here properly
  let activeGame = SupportedGame.Jak1;

  onMount(async () => {
    gameVersion = await launcherConfig.getGameInstallVersion(activeGame);
    launcherVerison = await getVersion();
  });
</script>

<header data-tauri-drag-region>
  <div id="logo">
    <img src={icon} alt="" />
    <p>OpenGOAL</p>
  </div>

  <div id="break" />

  <div id="versions">
    <p>v{launcherVerison} Launcher</p>
    <p>{gameVersion} Tools</p>
  </div>

  <div id="buttons">
    <!-- TODO: Conditional rendering of the alert bell. One case would be if there is an update available. -->
    <!-- <i class="fa-solid fa-bell" /> -->
    <i class="fa fa-window-minimize" on:click={() => appWindow.minimize()} />
    <i class="fa-solid fa-times" on:click={() => appWindow.close()} />
  </div>
</header>

<style>
  header {
    display: flex;
    flex-direction: row;
    align-items: center;
    height: 50px;
    background-color: #101010;
    padding: 5px;
  }

  #logo {
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  #logo img {
    height: 45px;
  }

  #logo p {
    padding: 10px;
    /* TODO: Update text to proper font */
    font-family: "Gill Sans", "Gill Sans MT", Calibri, "Trebuchet MS",
      sans-serif;
  }

  #break {
    border-left: 2px solid #9f9f9f;
    height: 90%;
    margin: 10px;
  }

  #versions {
    padding: 5px;
  }

  #versions p {
    margin: 0;
    padding: 2px 0px;
    /* TODO: Update text to proper font */
    font-family: "Gill Sans", "Gill Sans MT", Calibri, "Trebuchet MS",
      sans-serif;
    color: #9f9f9f;
  }

  #buttons {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    margin-left: auto;
    flex-basis: 0;
  }

  #buttons i {
    display: flex;
    justify-content: center;
    width: 25px;
    font-size: 20px;
    padding: 10px;
  }

  #buttons i:hover {
    background-color: #9f9f9f;
  }

  #buttons i:nth-last-child(2):hover {
    color: orange;
  }

  #buttons i:last-child:hover {
    color: red;
  }
</style>
