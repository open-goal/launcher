<script>
  export let bgVideo;
  import "./background.css";
</script>

<div class="video-container">
  <div class="overlay" />
  <video id="backgroundVideo" src={bgVideo} autoplay muted loop />
</div>
