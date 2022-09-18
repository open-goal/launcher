<script type="ts">
  export let bgVideo: string;
  export let bgPoster: string;
</script>

<div class="height-screen">
  <video
    class="absolute -z-50 object-fill h-screen"
    poster={bgPoster}
    src={bgVideo}
    autoplay
    muted
    loop
  />
</div>
