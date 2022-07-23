<script>
  import { closeSplashScreen } from "$lib/rpc/commands";
  import { areRequirementsMet, initConfig } from "$lib/config";
  import { checkRequirements } from "$lib/setup/setup";
  import { onMount } from "svelte";
  import logo from "$assets/images/icon.webp";
import { copyDataDirectory, dataDirectoryExists } from "$lib/utils/data-files";
import { log } from "$lib/utils/log";

  let currentProgress = 0;
  let currentStatusText = "Initializing Config";

  // Events
  onMount(async () => {
    await initConfig();
    currentStatusText = "Checking Requirements";
    currentProgress = 10;
    // NOTE - potentially has problems if the user changes hardware
    if (!(await areRequirementsMet())) {
      await checkRequirements();
    }
    currentStatusText = "Checking Data Files";
    currentProgress = 25;

    // Check if the data directory exists or not
    // - if it doesn't this is the users first launch, so lets copy it
    if (!(await dataDirectoryExists())) {
      try {
        currentStatusText = "Copying Data Files";
        currentProgress = 50;
        await copyDataDirectory();
      } catch (err) {
        log.error("error encountered when copying data files", {
          error: err
        });
        currentStatusText = `Error - ${err}`;
        return;
      }
    }
    currentStatusText = "Finishing Up";
    currentProgress = 100;
    await new Promise((res) => setTimeout(res, 2500));
    await closeSplashScreen();
  });
</script>

<div class="content">
  <div class="splash-logo">
    <img src={logo} alt="" />
  </div>
  <div class="splash-status-text">{currentStatusText}</div>
  <div>
    <div class="splash-status-bar fg" style="width: {currentProgress}%" />
    <div class="splash-status-bar bg" />
  </div>
</div>

<style>
  @font-face {
    font-family: "Roboto Mono";
    src: url("/src/assets/fonts/Roboto_Mono/static/RobotoMono-Regular.ttf");
  }
  @font-face {
    font-family: "Roboto Mono";
    src: url("/src/assets/fonts/Roboto_Mono/static/RobotoMono-Bold.ttf");
    font-weight: 700;
  }

  .content {
    color: white;
  }

  .splash-logo {
    height: 65vh;
    margin-bottom: 1em;
    padding: 10px;
  }

  .splash-logo img {
    object-fit: contain;
    height: 100%;
    width: 100%;
  }

  .splash-status-text {
    text-align: center;
    font-family: "Roboto Mono", monospace;
    font-size: 8pt;
    margin-bottom: 1.5em;
  }

  .splash-status-bar {
    width: 100%;
    height: 15px;
  }

  .splash-status-bar.bg {
    background-color: #775500;
    position: absolute;
  }
  .splash-status-bar.fg {
    background-color: #ffb807;
    position: absolute;
    z-index: 999;
  }
</style>
