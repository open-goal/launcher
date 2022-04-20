<script>
  import "../css/progress.css";
  let progressPercent = 10;
  export let status;
</script>

<section>
  <div class="status">
    <h2>{status}</h2>
  </div>

  <div class="progress">
    <div
      class="progress-bar progress-bar-animated"
      style="width: {progressPercent}%"
    />
  </div>
</section>
