<script>
  import { launchGame } from "$lib/launch";

  function onClickConfig() {
    alert("TODO");
  }
  function onClickPlay() {
    launchGame();
  }
</script>

<div id="launcherControls">
  <button class="btn lg" on:click={onClickConfig} disabled>Config</button>
  <button class="btn lg" on:click={onClickPlay}>Play</button>
</div>
