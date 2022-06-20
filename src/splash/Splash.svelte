<script>
  import { closeSplashScreen } from "$lib/commands";
  import { areRequirementsMet, initConfig } from "$lib/config";
  import { checkRequirements } from "$lib/setup";
  import { onMount } from "svelte";
  import logo from "$assets/images/logo.png";
  import "./splash.css";
import { copyDataDirectory, isDataDirectoryUpToDate } from "$lib/utils/file";

  let dataFilesCopied = false;
  let unableToCopy = false;

  // Events
  onMount(async () => {
    await initConfig();
    if (!(await areRequirementsMet())) {
      await checkRequirements();
    }
    // See if we've copied the files to the AppDir yet
    dataFilesCopied = await isDataDirectoryUpToDate();
    if (dataFilesCopied) {
      // Lies!
      setTimeout(async function() { await closeSplashScreen() },2500);
    } else {
      // Copy the files
      let ok = await copyDataDirectory();
      if (!ok) {
        unableToCopy = true;
      } else {
        await closeSplashScreen();
      }
    }
  });
</script>

<div class="splash-logo">
  <img src={logo}>
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
