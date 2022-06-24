<script>
  import { closeSplashScreen } from "$lib/commands";
  import {
    areRequirementsMet,
    initConfig,
    isGameInstallLatest,
  } from "$lib/config";
  import { checkRequirements } from "$lib/setup";
  import { onMount } from "svelte";
  import logo from "$assets/images/logo.webp";
  import "./splash.css";
  import { copyDataDirectory, isDataDirectoryUpToDate } from "$lib/utils/file";
  import { SupportedGame } from "$lib/constants";

  let dataFilesCopied = false;
  let unableToCopy = false;

  // Events
  onMount(async () => {
    await initConfig();
    if (!(await areRequirementsMet())) {
      await checkRequirements();
    }
    if (!(await isGameInstallLatest(SupportedGame.Jak1))) {
      // copy latest tools to the proper directory
      // await copyDataDirectory();
      // re-decompile game
      // compile game
      // update settings.json with latest tools version from metadata.json
    }
    // See if we've copied the files to the AppDir yet
    if (!(await isDataDirectoryUpToDate())) {
      try {
        await copyDataDirectory();
        dataFilesCopied = true;
      } catch (err) {
        console.log(err);
        unableToCopy = true;
      }
    }
    // sleep 2.5 seconds then close splash screen
    setTimeout(async function () {
      await closeSplashScreen();
    }, 2500);
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
  {/if}
</div>
