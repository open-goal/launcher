<script>
  // Assets
  import { fade } from "svelte/transition";
  import { getInstallStatus } from "$lib/config";
  import Setup from "../components/setup/Jak1Setup.svelte";
  import Jak1Main from "../components/games/Jak1Main.svelte";
  import Console from "../components/console/Console.svelte";
  import Logo from "../components/Logo.svelte";
  import { SupportedGame } from "$lib/constants";

  // NOTE: added this to shut up the console warning
  export let location;

  // State
  const gameInstalled = async () => await getInstallStatus(SupportedGame.Jak1);
</script>

<div class="flex-center" in:fade>
  <Logo />
  {#await gameInstalled() then installed}
    {#if installed}
      <Jak1Main />
    {:else}
      <Setup />
      <Console />
    {/if}
  {/await}
</div>
