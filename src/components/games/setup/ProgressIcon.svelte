<script lang="ts">
  import IconCheck from "~icons/mdi/check";
  import IconDots from "~icons/mdi/dots-horizontal";
  import IconAlert from "~icons/mdi/stop-alert";
  import IconHourglass from "~icons/mdi/hourglass";

  export let step;
  let { status, label } = step;
</script>

<div>
  <div
    class="size-10 mx-auto bg-slate-900 rounded-full text-lg text-white flex justify-center items-center"
  >
    {#if status === "success"}
      <IconCheck color="#22c55e" />
    {:else if status === "pending"}
      <IconDots class="animate-pulse" color="#facc15" />
    {:else if status === "failed"}
      <IconAlert color="#ef4444" />
    {:else}
      <IconHourglass color="#737373" />
    {/if}
  </div>
  <div class="text-center text-outline font-semibold mt-2">
    {label}
  </div>
</div>
