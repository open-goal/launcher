<script>
  import { closeSplashScreen } from "$lib/commands";
  import {
    areRequirementsMet,
    initConfig,
    setGameInstallVersion,
    shouldUpdateGameInstall,
  } from "$lib/config";
  import {
    checkRequirements,
    compileGame,
    decompileGameData,
  } from "$lib/setup/setup";
  import { onMount } from "svelte";
  import logo from "$assets/images/logo.webp";
  import "./splash.css";
  import { copyDataDirectory, isDataDirectoryUpToDate } from "$lib/utils/file";
  import { SupportedGame } from "$lib/constants";
  import { appDir, join } from "@tauri-apps/api/path";
  import { InstallStatus } from "../stores/InstallStore";

  let dataFilesCopied = false;
  let unableToCopy = false;

  // Events
  onMount(async () => {
    await initConfig();
    if (!(await areRequirementsMet())) {
      await checkRequirements();
    }
    // See if we've copied the files to the AppDir yet
    // TODO - I don't think this should be done on the Splash -- load the app and let the user consent to the move
    // replace the "play" button or something?
    if (!(await isDataDirectoryUpToDate())) {
      try {
        await copyDataDirectory();
        dataFilesCopied = true;
      } catch (err) {
        console.log(err);
        unableToCopy = true;
      }
    }
    if (await shouldUpdateGameInstall(SupportedGame.Jak1)) {
      // copy latest tools to the proper directory
      const isoPath = await join(await appDir(), "/data/extracted_iso/");
      try {
        await copyDataDirectory();
        dataFilesCopied = true;
      } catch (err) {
        console.log(err);
        unableToCopy = true;
      }
      // decompile & compile game
      await decompileGameData(isoPath);
      await compileGame(isoPath);
      // update settings.json with latest tools version from metadata.json
      await setGameInstallVersion(SupportedGame.Jak1);
    }
    await new Promise((res) => setTimeout(res, 2500));
    await closeSplashScreen();
  });
</script>

<div class="splash-logo">
  <img src={logo} alt="" />
</div>
<div class="splash-status">
  {#if !dataFilesCopied}
    Copying Data Files...
    {#if unableToCopy}
      Error - Unable to Copy Data Files
    {/if}
  {:else}
    Checking Data Files...
    <br />
    {$InstallStatus.status}
  {/if}
</div>
