<script>
  import { InstallStatus } from "$lib/stores/AppStore";
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

<style>
  .progress {
  display: flex;
  overflow: hidden;
  height: 30px;
  background-color: #ecf0f1;
  width: 45vw;
  max-width: 45vw;
  margin: 20px auto;
}

.progress-bar {
  overflow: hidden;
  text-align: center;
  background-color: dodgerblue;
}

.progress-bar-animated {
  position: relative;
}

.progress-bar-animated::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(to right, transparent, rgba(255, 255, 255, 0.5));
  transform: translateX(-100%);
  animation: progress-bar-shine 2s infinite;
}

@keyframes progress-bar-shine {
  to {
    transform: translateX(0);
    opacity: 0.1;
  }
}

.status {
  justify-content: center;
  align-items: center;
  text-align: center;
}

</style>
