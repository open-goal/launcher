<script type="ts">
  export let bgVideo: string;
</script>

<div class="height-screen">
  <video
    class="absolute -z-50 object-fill h-screen"
    src={bgVideo}
    autoplay
    muted
    loop
  />
</div>
