<script type="ts">
  export let bgVideo: string;
  export let bgVideoPoster: string;
</script>

<div class="height-screen">
  <video
    class="absolute -z-50 object-fill h-screen"
    poster={bgVideoPoster}
    src={bgVideo}
    autoplay
    muted
    loop
  />
</div>
