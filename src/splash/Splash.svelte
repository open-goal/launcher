<script>
  import { closeSplashScreen } from "$lib/commands";
  import {
    areRequirementsMet,
    initConfig,
  } from "$lib/config";
  import {
    checkRequirements,
  } from "$lib/setup/setup";
  import { onMount } from "svelte";
  import logo from "$assets/images/icon.webp";
  import {
    copyDataDirectory,
    dataDirectoryExists,
    isDataDirectoryUpToDate,
  } from "$lib/utils/file";
  import { dataFilesOutOfDate } from "../stores/AppStore";

  let currentProgress = 0;
  let currentStatusText = "Initializing Config";

  // Events
  onMount(async () => {
    await initConfig();
    currentStatusText = "Checking Requirements";
    currentProgress = 10;
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
        console.log(err);
        currentStatusText = `Error - ${err}`;
        return;
      }
    } else {
      // - otherwise, we'll check it to see if it needs updating
      // we don't actually do the copy here, let the user consent to it later
      const outOfDate = await isDataDirectoryUpToDate();
      dataFilesOutOfDate.update(() => outOfDate);
    }
    currentStatusText = "Finishing Up";
    currentProgress = 100;
    await new Promise((res) => setTimeout(res, 2500));
    await closeSplashScreen();

    // TODO - check this in the actual game route, not here!
    // if (await shouldUpdateGameInstall(SupportedGame.Jak1)) {
    //   // copy latest tools to the proper directory
    //   const isoPath = await join(await appDir(), "/data/extracted_iso/");
    //   try {
    //     await copyDataDirectory();
    //     dataFilesCopied = true;
    //   } catch (err) {
    //     console.log(err);
    //     unableToCopy = true;
    //   }
    //   // decompile & compile game
    //   await decompileGameData(isoPath);
    //   await compileGame(isoPath);
    //   // update settings.json with latest tools version from metadata.json
    //   await setGameInstallVersion(SupportedGame.Jak1);
    // }
  });
</script>

<div class="content">
  <div class="splash-logo">
    <img src={logo} alt="" />
  </div>
  <div class="splash-status-text">{currentStatusText}</div>
  <div>
    <div class="splash-status-bar fg" style="width: {currentProgress}%"></div>
    <div class="splash-status-bar bg"></div>
  </div>
</div>

<style>
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
    font-family: monospace;
    margin-bottom: 1em;
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
