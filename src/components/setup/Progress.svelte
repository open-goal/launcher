<script>
  import "./progress.css";
  import InstallStore from "../../stores/InstallStore";

  let status, percent;
  const setStatus = (currentStatus) => (status = currentStatus);
  const setPercent = (currentPercent) => (percent = currentPercent);

  InstallStore.subscribe(([{ currentStatus }]) => {
    let { status, percent } = currentStatus ? currentStatus : {};
    setStatus(status);
    setPercent(percent);
  });
</script>

<section>
  {#if status !== undefined}
    <div class="status">
      <h2>{status}</h2>
    </div>
  {/if}

  {#if percent >= 0}
    <div class="progress">
      <div
        class="progress-bar progress-bar-animated"
        style="width: {percent}%"
      />
    </div>
  {/if}
</section>
