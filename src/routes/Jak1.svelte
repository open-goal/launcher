<script>
  // Assets
  import logo from '$assets/images/logo.png';
  import { onMount } from 'svelte';
  import { Link } from "svelte-routing";
  import { getInstallStatus, SupportedGame } from '$lib/config';
  import { launchGame } from '$lib/launch';

  // State
  let gameInstalled = false;

  // Events
  onMount(async () => {
		gameInstalled = await getInstallStatus(SupportedGame.Jak1);
	});

  function onClickConfig() {
    alert("TODO");
  }
  function onClickPlay() {
    launchGame();
  }
</script>

<div class="flex-center">
  <div class="logo">
    <img id="logo" width="65%" src={logo} alt="OpenGOAL">
  </div>
  <div id="launcherControls">
    {#if gameInstalled}
    <button class="btn lg" on:click={onClickConfig} disabled>Config</button>
    <button class="btn lg" on:click={onClickPlay}>Play</button>
    {:else}
    <Link to="/setup/jak1">
      <button class="btn lg">Setup</button>
    </Link>
    {/if}
  </div>
</div>
