<script>
  import "./progress.css";
  import { InstallStatus } from "../../stores/InstallStore";
</script>

<section>
  {#if $InstallStatus.status !== undefined}
    <div class="status">
      <h2>{$InstallStatus.status}</h2>
    </div>
  {/if}

  {#if $InstallStatus.percent >= 0}
    <div class="progress">
      <div
        class="progress-bar progress-bar-animated"
        style="width: {$InstallStatus.percent}%"
      />
    </div>
  {/if}
</section>
