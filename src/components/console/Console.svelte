<script>
  import { Console } from "../../stores/InstallStore";
  import "./console.css";
</script>

<div class="row">
  <details>
    <summary>Installation Logs</summary>
    <div class="logContainer">
      {#if $Console}
        {$Console}
      {/if}
    </div>
  </details>
</div>
