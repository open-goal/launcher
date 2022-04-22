<script>
  // Assets
  import { getInstallStatus, SupportedGame } from "$lib/config";
  import Setup from "../components/setup/Jak1Setup.svelte";
  import Jak1Main from "../components/games/Jak1Main.svelte";
  import Console from "../components/Console.svelte";
  import Logo from "../components/Logo.svelte";

  // State
  const gameInstalled = async () => await getInstallStatus(SupportedGame.Jak1);
</script>

<div class="flex-center">
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
