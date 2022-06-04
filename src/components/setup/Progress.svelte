<script>
  import "./progress.css";
  import { InstallStatus } from "../../stores/InstallStore";
  import { tweened } from "svelte/motion";
  import { cubicOut } from "svelte/easing";

  const progress = tweened($InstallStatus.percent, {
    duration: 1000,
    easing: cubicOut,
  });

  InstallStatus.subscribe((InstallStatus) => {
    progress.update(() => InstallStatus.percent);
  });
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
        style="width: {$progress}%"
      />
    </div>
  {/if}
</section>
