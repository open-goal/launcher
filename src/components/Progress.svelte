<script>
  import "../css/progress.css";
  export let step;
</script>

<section>
  <div class="status">
    <h2>{step.status}</h2>
  </div>

  {#if step.percent >= 0}
    <div class="progress">
      <div
        class="progress-bar progress-bar-animated"
        style="width: {step.percent}%"
      />
    </div>
  {/if}
</section>
