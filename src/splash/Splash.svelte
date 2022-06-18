<script>
  import { closeSplashScreen } from "$lib/commands";
  import { areRequirementsMet, initConfig } from "$lib/config";
  import { checkRequirements } from "$lib/setup";
  import { onMount } from "svelte";
  import "./splash.css";

  // Events
  onMount(async () => {
    await initConfig();
    if (!(await areRequirementsMet())) {
      await checkRequirements();
    }
    await closeSplashScreen();
  });
</script>
