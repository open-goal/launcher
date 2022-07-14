<script>
  import { ProcessLogs } from "$lib/stores/AppStore";
</script>

<div class="row">
  <details>
    <summary>Installation Logs</summary>
    <div class="logContainer">
      {#if $ProcessLogs}
        {$ProcessLogs}
      {/if}
    </div>
  </details>
</div>

<style>
  .logContainer {
    max-height: 10em;
    min-height: 100px;
    overflow-y: scroll;
    padding: 1em;
    background: black;
    white-space: pre-wrap;
    font-family: monospace;
  }

  details {
    min-width: 75vw;
    max-width: 75vw;
  }

  summary {
    text-align: center;
  }

  .logContainer {
    text-align: start;
  }

  details > summary:hover {
    cursor: pointer;
  }
</style>
