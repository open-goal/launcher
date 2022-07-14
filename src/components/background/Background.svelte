<script type="ts">
  export let bgVideo: string;
</script>

<div class="video-container">
  <div class="overlay" />
  <video id="backgroundVideo" src={bgVideo} autoplay muted loop />
</div>

<style>
  .video-container {
    position: absolute;
    z-index: -1;
    height: 100vh;
  }

  .overlay {
    position: absolute;
    top: 0;
    left: 0;
    background-color: #181818;
    width: 100%;
    height: 100vh;
    opacity: 0.35;
  }

  #backgroundVideo {
    z-index: -1;
    height: 100vh;
  }
</style>
